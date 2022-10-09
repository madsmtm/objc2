use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use apple_sdk::{Platform, SdkPath};
use clang::{Clang, EntityVisitResult, Index};
use quote::{format_ident, quote};

use header_translator::{format_method_macro, run_cargo_fmt, run_rustfmt, Config, RustFile, Stmt};

const FORMAT_INCREMENTALLY: bool = false;

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir.parent().unwrap();
    let crate_src = workspace_dir.join("icrate/src");

    println!("status: loading configs...");
    let configs = load_configs(&crate_src);
    println!("status: loaded {} configs", configs.len());

    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, true, true);

    let mut result = None;

    // TODO: Parse multiple SDKs, and compare
    for path in [
        workspace_dir.join("ideas/MacOSX-SDK-changes/MacOSXA.B.C.sdk"),
        // Path::new("/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk"),
    ] {
        let sdk = SdkPath::from_path(path).expect("sdk from path");
        println!("status: parsing {:?}...", sdk.platform);
        result = Some(parse(&index, &configs, &sdk));
        println!("status: done parsing {:?}", sdk.platform);
    }

    for (library, files) in result.unwrap() {
        println!("status: writing framework {library}...");
        let output_path = crate_src.join("generated").join(&library);
        output_files(&output_path, files, FORMAT_INCREMENTALLY);
        println!("status: written framework {library}");
    }

    if !FORMAT_INCREMENTALLY {
        println!("status: formatting");
        run_cargo_fmt("icrate");
    }
}

fn load_configs(crate_src: &Path) -> BTreeMap<String, Config> {
    crate_src
        .read_dir()
        .expect("read_dir")
        .filter_map(|dir| {
            let dir = dir.expect("dir");
            if !dir.file_type().expect("file type").is_dir() {
                return None;
            }
            let path = dir.path();
            let file = path.join("translation-config.toml");
            match Config::from_file(&file) {
                Ok(config) => Some((
                    path.file_name()
                        .expect("framework name")
                        .to_string_lossy()
                        .to_string(),
                    config,
                )),
                Err(err) if err.kind() == io::ErrorKind::NotFound => None,
                Err(err) => panic!("{file:?}: {err}"),
            }
        })
        .collect()
}

fn parse(
    index: &Index<'_>,
    configs: &BTreeMap<String, Config>,
    sdk: &SdkPath,
) -> BTreeMap<String, BTreeMap<String, RustFile>> {
    let (target, version_min) = match &sdk.platform {
        Platform::MacOsX => ("--target=x86_64-apple-macos", "-mmacosx-version-min=10.14"),
        Platform::IPhoneOs => ("--target=x86_64-apple-ios", "-miphoneos-version-min=10.14"),
        _ => ("--target=x86_64-apple-macos", "-mmacosx-version-min=10.14"),
    };

    let tu = index
        .parser(&Path::new(env!("CARGO_MANIFEST_DIR")).join("framework-includes.h"))
        .detailed_preprocessing_record(true)
        .incomplete(true)
        .skip_function_bodies(true)
        .keep_going(true)
        // .single_file_parse(true)
        .include_attributed_types(true)
        .visit_implicit_attributes(true)
        // .ignore_non_errors_from_included_files(true)
        .retain_excluded_conditional_blocks(true)
        .arguments(&[
            "-x",
            "objective-c",
            target,
            "-Wall",
            "-Wextra",
            "-fobjc-arc",
            "-fobjc-arc-exceptions",
            "-fobjc-abi-version=2", // 3??
            // "-fparse-all-comments",
            "-fapinotes",
            version_min,
            "-isysroot",
            sdk.path.to_str().unwrap(),
        ])
        .parse()
        .unwrap();

    println!("status: initialized translation unit {:?}", sdk.platform);

    dbg!(&tu);
    dbg!(tu.get_target());
    dbg!(tu.get_memory_usage());
    dbg!(tu.get_diagnostics());

    // let dbg_file = |file: File<'_>| {
    //     dbg!(
    //         &file,
    //         file.get_module(),
    //         file.get_skipped_ranges(),
    //         file.is_include_guarded(),
    //         // file.get_includes(),
    //         // file.get_references(),
    //     );
    // };
    //
    // dbg_file(tu.get_file(&header).unwrap());
    // dbg_file(tu.get_file(&dir.join("NSAccessibility.h")).unwrap());
    // let cursor_file = tu.get_file(&dir.join("NSCursor.h")).unwrap();
    // dbg_file(cursor_file);

    let entity = tu.get_entity();

    dbg!(&entity);
    dbg!(entity.get_availability());

    let mut result: BTreeMap<String, BTreeMap<String, RustFile>> = BTreeMap::new();

    let framework_dir = sdk.path.join("System/Library/Frameworks");
    entity.visit_children(|entity, _parent| {
        if let Some(location) = entity.get_location() {
            if let Some(file) = location.get_file_location().file {
                let path = file.get_path();
                if let Ok(path) = path.strip_prefix(&framework_dir) {
                    let mut components = path.components();
                    let library = components
                        .next()
                        .expect("components next")
                        .as_os_str()
                        .to_str()
                        .expect("component to_str")
                        .strip_suffix(".framework")
                        .expect("framework fileending");
                    let path = components.as_path();

                    if let Some(config) = configs.get(library) {
                        let name = path
                            .file_stem()
                            .expect("path file stem")
                            .to_string_lossy()
                            .to_owned();
                        if name != library {
                            let files = result.entry(library.to_string()).or_default();
                            let file = files.entry(name.to_string()).or_insert_with(RustFile::new);
                            if let Some(stmt) = Stmt::parse(&entity, &config) {
                                file.add_stmt(stmt);
                            }
                        }
                    } else {
                        // println!("library not found {library}");
                    }
                }
            }
        }
        EntityVisitResult::Continue
    });

    result
}

fn output_files(
    output_path: &Path,
    files: impl IntoIterator<Item = (String, RustFile)>,
    format_incrementally: bool,
) {
    let declared: Vec<_> = files
        .into_iter()
        .map(|(name, file)| {
            let (declared_types, tokens) = file.finish();

            let mut path = output_path.join(&name);
            path.set_extension("rs");

            let output = if format_incrementally {
                run_rustfmt(tokens)
            } else {
                let mut buf = Vec::new();
                write!(buf, "{}", tokens).unwrap();
                format_method_macro(&buf)
            };

            fs::write(&path, output).unwrap();

            (format_ident!("{}", name), declared_types)
        })
        .collect();

    let mod_names = declared.iter().map(|(name, _)| name);
    let mod_imports = declared.iter().filter_map(|(name, declared_types)| {
        if !declared_types.is_empty() {
            let declared_types = declared_types.iter().map(|name| format_ident!("{}", name));
            Some(quote!(super::#name::{#(#declared_types,)*}))
        } else {
            None
        }
    });

    let tokens = quote! {
        #(pub(crate) mod #mod_names;)*

        mod __exported {
            #(pub use #mod_imports;)*
        }
    };

    let output = if format_incrementally {
        run_rustfmt(tokens)
    } else {
        let mut buf = Vec::new();
        write!(buf, "{}", tokens).unwrap();
        buf
    };

    // truncate if the file exists
    fs::write(output_path.join("mod.rs"), output).unwrap();
}
