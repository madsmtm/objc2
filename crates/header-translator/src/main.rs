use std::collections::BTreeMap;
use std::fmt::{self, Write};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use apple_sdk::{AppleSdk, DeveloperDirectory, Platform, SdkPath, SimpleSdk};
use clang::{Clang, Entity, EntityKind, EntityVisitResult, Index};

use header_translator::{run_cargo_fmt, run_rustfmt, Config, RustFile, Stmt};

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

    let developer_dir = DeveloperDirectory::from(PathBuf::from(
        std::env::args_os()
            .skip(1)
            .next()
            .expect("must specify developer directory as first argument"),
    ));

    let sdks = developer_dir
        .platforms()
        .expect("developer dir platforms")
        .into_iter()
        .filter_map(|platform| {
            matches!(&*platform, Platform::MacOsX | Platform::IPhoneOs).then(|| {
                let sdks: Vec<_> = platform
                    .find_sdks::<SimpleSdk>()
                    .expect("platform sdks")
                    .into_iter()
                    .filter(|sdk| !sdk.is_symlink() && sdk.platform() == &*platform)
                    .collect();
                if sdks.len() != 1 {
                    panic!("found multiple sdks {sdks:?} in {:?}", &*platform);
                }
                sdks[0].sdk_path()
            })
        });

    let mut result: BTreeMap<String, BTreeMap<String, RustFile>> = configs
        .iter()
        .map(|(library, _)| (library.clone(), BTreeMap::new()))
        .collect();

    // TODO: Compare SDKs
    for sdk in sdks {
        println!("status: parsing {:?}...", sdk.platform);

        let mut preprocessing = true;

        parse_and_visit_stmts(&index, &sdk, |library, file_name, entity| {
            if let Some(config) = configs.get(library) {
                let files = result.get_mut(library).expect("files");
                match entity.get_kind() {
                    EntityKind::InclusionDirective if preprocessing => {
                        // println!("{library}/{file_name}.h: {entity:?}");
                        // If umbrella header
                        let name = entity.get_name().expect("inclusion name");
                        let mut iter = name.split('/');
                        let framework = iter.next().expect("inclusion name has framework");
                        if framework == library {
                            let included = iter
                                .next()
                                .expect("inclusion name has file")
                                .strip_suffix(".h")
                                .expect("inclusion name file is header")
                                .to_string();
                            if iter.count() != 0 {
                                panic!("invalid inclusion of {name:?}");
                            }

                            // If inclusion is not umbrella header
                            if included != library {
                                // The file is often included twice, even
                                // within the same file, so insertion can fail
                                files.entry(included).or_insert_with(RustFile::new);
                            }
                        }
                    }
                    EntityKind::MacroExpansion if preprocessing => {}
                    EntityKind::MacroDefinition if preprocessing => {}
                    _ => {
                        if preprocessing {
                            println!("status: preprocessed {:?}...", sdk.platform);
                        }
                        preprocessing = false;
                        // No more includes / macro expansions after this line
                        let file = files.get_mut(file_name).expect("file");
                        if let Some(stmt) = Stmt::parse(&entity, &config) {
                            if sdk.platform == Platform::MacOsX {
                                file.add_stmt(stmt);
                            }
                        }
                    }
                }
            } else {
                // println!("library not found {library}");
            }
        });
        println!("status: done parsing {:?}", sdk.platform);
    }

    for (library, files) in result {
        println!("status: writing framework {library}...");
        let output_path = crate_src.join("generated").join(&library);
        output_files(&output_path, files, FORMAT_INCREMENTALLY).unwrap();
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

fn parse_and_visit_stmts(
    index: &Index<'_>,
    sdk: &SdkPath,
    mut f: impl FnMut(&str, &str, Entity<'_>),
) {
    let (target, version_min) = match &sdk.platform {
        Platform::MacOsX => ("--target=x86_64-apple-macos", "-mmacosx-version-min=10.7"),
        Platform::IPhoneOs => ("--target=arm64-apple-ios", "-miphoneos-version-min=7.0"),
        _ => todo!(),
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
                    let name = path
                        .file_stem()
                        .expect("path file stem")
                        .to_string_lossy()
                        .to_owned();

                    f(&library, &name, entity);
                }
            }
        }
        EntityVisitResult::Continue
    });
}

fn output_files(
    output_path: &Path,
    files: impl IntoIterator<Item = (String, RustFile)>,
    format_incrementally: bool,
) -> fmt::Result {
    let declared: Vec<_> = files
        .into_iter()
        .map(|(name, file)| {
            let (declared_types, tokens) = file.finish();

            let mut path = output_path.join(&name);
            path.set_extension("rs");

            let output = if format_incrementally {
                run_rustfmt(tokens)
            } else {
                tokens.into()
            };

            fs::write(&path, output).unwrap();

            (name, declared_types)
        })
        .collect();

    let mut tokens = String::new();

    for (name, _) in &declared {
        writeln!(tokens, "pub(crate) mod {name};")?;
    }
    writeln!(tokens, "")?;
    writeln!(tokens, "mod __exported {{")?;
    for (name, declared_types) in declared {
        if !declared_types.is_empty() {
            let declared_types: Vec<_> = declared_types.into_iter().collect();
            writeln!(
                tokens,
                "    pub use super::{name}::{{{}}};",
                declared_types.join(",")
            )?;
        }
    }
    writeln!(tokens, "}}")?;

    let output = if format_incrementally {
        run_rustfmt(tokens)
    } else {
        tokens.into()
    };

    // truncate if the file exists
    fs::write(output_path.join("mod.rs"), output).unwrap();

    Ok(())
}
