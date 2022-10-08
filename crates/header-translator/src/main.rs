use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io;
use std::path::Path;

use clang::{Clang, EntityVisitResult, Index};
use quote::{format_ident, quote};

use header_translator::{run_rustfmt, Config, RustFile, Stmt};

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir.parent().unwrap();
    let crate_src = workspace_dir.join("icrate/src");

    // let sysroot = Path::new("/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk");
    let sysroot = workspace_dir.join("ideas/MacOSX-SDK-changes/MacOSXA.B.C.sdk");
    let framework_dir = sysroot.join("System/Library/Frameworks");

    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, true, true);
    let tu = index
        .parser(&manifest_dir.join("framework-includes.h"))
        .detailed_preprocessing_record(true)
        // .single_file_parse(true)
        .skip_function_bodies(true)
        .include_attributed_types(true)
        .visit_implicit_attributes(true)
        .retain_excluded_conditional_blocks(true)
        .arguments(&[
            "-x",
            "objective-c",
            "-fobjc-arc",
            "-fobjc-abi-version=2", // 3??
            // "-mmacosx-version-min="
            "-fparse-all-comments",
            // "-fapinotes",
            "-isysroot",
            sysroot.to_str().unwrap(),
        ])
        .parse()
        .unwrap();

    println!("status: initialized clang");

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

    let configs: HashMap<String, Config> = crate_src
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
        .collect();

    println!("status: loaded {} configs", configs.len());

    let mut result: HashMap<String, BTreeMap<String, RustFile>> = HashMap::new();

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

    println!("status: loaded data");

    for (library, files) in result.into_iter() {
        let output_path = crate_src.join("generated").join(&library);

        let declared: Vec<_> = files
            .into_iter()
            .map(|(name, file)| {
                let (declared_types, tokens) = file.finish();

                let mut path = output_path.join(&name);
                path.set_extension("rs");

                fs::write(&path, run_rustfmt(tokens)).unwrap();

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

        // truncate if the file exists
        fs::write(output_path.join("mod.rs"), run_rustfmt(tokens)).unwrap();

        println!("status: written framework {library}");
    }
}
