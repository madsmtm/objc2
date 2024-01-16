use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use apple_sdk::{AppleSdk, DeveloperDirectory, Platform, SdkPath, SimpleSdk};
use clang::{Clang, EntityKind, EntityVisitResult, Index, TranslationUnit};
use tracing::{debug_span, error, info, info_span, trace, trace_span};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::{Layer, SubscriberExt};
use tracing_subscriber::registry::Registry;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_tree::HierarchicalLayer;

use header_translator::{run_cargo_fmt, Cache, Config, Context, File, Output, Stmt};

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

fn main() -> Result<(), BoxError> {
    // use tracing_subscriber::fmt;
    Registry::default()
        // .with(
        //     fmt::Layer::default()
        //         .compact()
        //         .without_time()
        //         .with_target(false)
        //         .with_span_events(fmt::format::FmtSpan::ACTIVE)
        //         .with_filter(LevelFilter::INFO)
        //         .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
        //             metadata.is_span() && metadata.level() == &tracing::Level::INFO
        //         })),
        // )
        // .with(tracing_subscriber::fmt::Layer::default().with_filter(LevelFilter::ERROR))
        .with(
            HierarchicalLayer::new(2)
                .with_targets(false)
                .with_indent_lines(true)
                // Note: Change this to DEBUG if you want to see more info
                .with_filter(LevelFilter::INFO),
        )
        .init();
    let _span = info_span!("running").entered();

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir.parent().unwrap();
    let crate_src = workspace_dir.join("icrate/src");

    let config = load_config(manifest_dir);

    clang_sys::load()?;
    info!(clang_version = clang::get_version());

    let clang = Clang::new()?;
    let index = Index::new(&clang, true, true);

    let developer_dir = if let Some(path) = std::env::args_os().nth(1) {
        DeveloperDirectory::from(PathBuf::from(path))
    } else {
        DeveloperDirectory::from_xcode_select()?
    };

    let sdks: Vec<_> = developer_dir
        .platforms()
        .expect("developer dir platforms")
        .into_iter()
        .map(|platform| {
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
        .collect();

    if sdks.len() != 8 {
        error!("should have one of each platform: {sdks:?}");
    }

    let mut final_result = None;

    // TODO: Compare between SDKs
    for sdk in sdks {
        // These are found using the `get_llvm_targets.fish` helper script
        let llvm_targets: &[_] = match &sdk.platform {
            Platform::MacOsX => &[
                "x86_64-apple-macosx10.12.0",
                // "arm64-apple-macosx11.0.0",
                // "i686-apple-macosx10.12.0",
            ],
            Platform::IPhoneOs => &[
                // "arm64-apple-ios10.0.0",
                // "armv7s-apple-ios10.0.0",
                // "arm64-apple-ios14.0-macabi",
                // "x86_64-apple-ios13.0-macabi",
            ],
            // Platform::IPhoneSimulator => &[
            //     "arm64-apple-ios10.0.0-simulator",
            //     "x86_64-apple-ios10.0.0-simulator",
            //     "i386-apple-ios10.0.0-simulator",
            // ],
            // Platform::AppleTvOs => &["arm64-apple-tvos", "x86_64-apple-tvos"],
            // Platform::WatchOs => &["arm64_32-apple-watchos", "armv7k-apple-watchos"],
            // Platform::WatchSimulator => &[
            //     "arm64-apple-watchos5.0.0-simulator",
            //     "x86_64-apple-watchos5.0.0-simulator",
            // ],
            _ => continue,
        };

        let mut result: Option<Output> = None;

        for llvm_target in llvm_targets {
            let _span = info_span!("parsing", platform = ?sdk.platform, llvm_target).entered();
            let curr_result = parse_sdk(&index, &sdk, llvm_target, &config);

            if let Some(prev_result) = &result {
                let _span = info_span!("comparing results").entered();
                prev_result.compare(&curr_result);

                // Extra check in case our comparison above was not exaustive
                assert_eq!(*prev_result, curr_result);
            } else {
                result = Some(curr_result);
            }
        }

        if sdk.platform == Platform::MacOsX {
            final_result = result;
        }
    }

    let mut final_result = final_result.expect("got a result");
    let span = info_span!("analyzing").entered();
    let cache = Cache::new(&final_result, &config);
    cache.update(&mut final_result);
    drop(span);

    let generated_dir = crate_src.join("generated");
    fs::create_dir_all(&generated_dir)?;

    for (library_name, files) in &final_result.libraries {
        let _span = info_span!("writing", library_name).entered();
        let output_path = generated_dir.join(library_name);
        fs::create_dir_all(&output_path)?;
        files.output(&output_path).unwrap();
    }

    final_result
        .output_module(&generated_dir.join("mod.rs"))
        .unwrap();

    let span = info_span!("writing features").entered();
    const FEATURE_SECTION_PATTERN:
        &str = "# This section has been automatically generated by `objc2`'s `header-translator`.\n# DO NOT EDIT\n";
    let mut cargo_toml = {
        let path = crate_src.parent().unwrap().join("Cargo.toml");
        fs::OpenOptions::new().read(true).append(true).open(path)?
    };
    // find the features section
    if let Some(pos) = {
        let mut text = String::new();
        cargo_toml.read_to_string(&mut text)?;
        text.find(FEATURE_SECTION_PATTERN)
    } {
        // truncate the file to the section header before writing the features
        let len = u64::try_from(pos + FEATURE_SECTION_PATTERN.len())?;
        cargo_toml.set_len(len)?;
    } else {
        return Err("feature section not found in icrate/Cargo.toml".into());
    }
    for (feature, required_features) in final_result.cargo_features(&config) {
        write!(cargo_toml, "{feature} = [")?;
        if !required_features.is_empty() {
            writeln!(cargo_toml)?;
        }
        for feature in required_features {
            writeln!(cargo_toml, "    \"{feature}\",")?;
        }
        writeln!(cargo_toml, "]")?;
    }
    drop(cargo_toml);
    drop(span);

    let _span = info_span!("formatting").entered();
    run_cargo_fmt("icrate");

    Ok(())
}

fn load_config(manifest_dir: &Path) -> Config {
    let _span = info_span!("loading config").entered();

    Config::from_file(&manifest_dir.join("translation-config.toml")).expect("read config")
}

fn parse_sdk(index: &Index<'_>, sdk: &SdkPath, llvm_target: &str, config: &Config) -> Output {
    let tu = get_translation_unit(index, sdk, llvm_target);

    let mut preprocessing = true;
    let mut result = Output::from_libraries(&config.libraries);

    let mut library_span = None;
    let mut library_span_name = String::new();
    let mut file_span = None;
    let mut file_span_name = String::new();

    let mut context = Context::new(config, sdk);

    tu.get_entity().visit_children(|entity, _parent| {
        let _span = trace_span!("entity", ?entity).entered();
        if let Some((library_name, Some(file_name))) = context.get_library_and_file_name(&entity) {
            if library_span_name != library_name {
                library_span.take();
                file_span.take();
                file_span_name = String::new();

                library_span_name = library_name.clone();
                library_span = Some(debug_span!("library", name = library_name).entered());
            }
            if file_span_name != file_name {
                file_span.take();

                file_span_name = file_name.clone();
                file_span = Some(debug_span!("file", name = file_name).entered());
            }

            if let Some(library) = result.libraries.get_mut(&library_name) {
                match entity.get_kind() {
                    EntityKind::InclusionDirective if preprocessing => {
                        let name = entity.get_name().expect("inclusion name");
                        let mut iter = name.split('/');
                        let framework = iter.next().expect("inclusion name has framework");
                        if framework == library_name {
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
                            if included != library_name {
                                // The file is often included twice, even
                                // within the same file, so insertion can fail
                                library
                                    .files
                                    .entry(included)
                                    .or_insert_with(|| File::new(&library_name, &context));
                            }
                        }
                    }
                    EntityKind::MacroExpansion if preprocessing => {
                        let location = entity.get_location().expect("macro location");
                        context
                            .macro_invocations
                            .insert(location.get_spelling_location(), entity);
                    }
                    EntityKind::MacroDefinition if preprocessing => {
                        // let name = entity.get_name().expect("macro def name");
                        // entity.is_function_like_macro();
                        // trace!("macrodef", name);
                    }
                    _ => {
                        if preprocessing {
                            info!("done preprocessing");
                        }
                        preprocessing = false;
                        // No more includes / macro expansions after this line
                        let mut maybe_file = library.files.get_mut(&file_name);
                        for stmt in Stmt::parse(&entity, &context) {
                            let file: &mut File = maybe_file.as_mut().expect("file");
                            file.add_stmt(stmt);
                        }
                    }
                }
            } else {
                trace!("library not found");
            }
        }
        EntityVisitResult::Continue
    });

    result
}

fn get_translation_unit<'i: 'tu, 'tu>(
    index: &'i Index<'tu>,
    sdk: &SdkPath,
    llvm_target: &str,
) -> TranslationUnit<'tu> {
    let _span = info_span!("initializing translation unit").entered();

    let target = format!("--target={llvm_target}");

    let tu = index
        .parser(Path::new(env!("CARGO_MANIFEST_DIR")).join("framework-includes.h"))
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
            &target,
            "-Wall",
            "-Wextra",
            "-fobjc-arc",
            "-fobjc-arc-exceptions",
            "-fobjc-abi-version=2", // 3??
            // "-fparse-all-comments",
            // TODO: "-fretain-comments-from-system-headers"
            "-fapinotes",
            "-isysroot",
            sdk.path.to_str().unwrap(),
            // See ClangImporter.cpp and Foundation/NSObjCRuntime.h
            "-D",
            "__SWIFT_ATTR_SUPPORTS_SENDABLE_DECLS=1",
        ])
        .parse()
        .unwrap();

    // dbg!(&tu);
    // dbg!(tu.get_target());
    // dbg!(tu.get_memory_usage());
    // dbg!(tu.get_diagnostics());

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

    tu
}
