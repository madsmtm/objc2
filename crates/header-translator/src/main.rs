use std::path::{Path, PathBuf};

use apple_sdk::{AppleSdk, DeveloperDirectory, Platform, SdkPath, SimpleSdk};
use clang::{Clang, EntityKind, EntityVisitResult, Index, TranslationUnit};
use tracing::{debug_span, info, info_span, trace, trace_span};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::{Layer, SubscriberExt};
use tracing_subscriber::registry::Registry;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_tree::HierarchicalLayer;

use header_translator::{run_cargo_fmt, Cache, Config, Context, File, Output, Stmt};

fn main() {
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
        .with(
            HierarchicalLayer::new(2)
                .with_targets(false)
                .with_indent_lines(true)
                // Note: Change this to DEBUG if you want to see more info
                .with_filter(LevelFilter::INFO),
        )
        .init();

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir.parent().unwrap();
    let crate_src = workspace_dir.join("icrate/src");

    let config = load_config(manifest_dir);

    let clang = Clang::new().unwrap();
    let index = Index::new(&clang, true, true);

    let developer_dir = DeveloperDirectory::from(PathBuf::from(
        std::env::args_os()
            .nth(1)
            .expect("must specify developer directory as first argument"),
    ));

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

    assert_eq!(sdks.len(), 8, "should have one of each platform: {sdks:?}");

    let mut final_result = None;

    // TODO: Compare between SDKs
    for sdk in sdks {
        // These are found using the `get_llvm_targets.fish` helper script
        let llvm_targets: &[_] = match &sdk.platform {
            Platform::MacOsX => &[
                "x86_64-apple-macosx10.7.0",
                // "arm64-apple-macosx11.0.0",
                // "i686-apple-macosx10.7.0",
            ],
            Platform::IPhoneOs => &[
                // "arm64-apple-ios7.0.0",
                // "armv7-apple-ios7.0.0",
                // "armv7s-apple-ios",
                // "arm64-apple-ios14.0-macabi",
                // "x86_64-apple-ios13.0-macabi",
            ],
            // Platform::IPhoneSimulator => &[
            //     "arm64-apple-ios7.0.0-simulator",
            //     "x86_64-apple-ios7.0.0-simulator",
            //     "i386-apple-ios7.0.0-simulator",
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

    for (library_name, files) in final_result.libraries {
        let _span = info_span!("writing", library_name).entered();
        let output_path = crate_src.join("generated").join(&library_name);
        files.output(&output_path).unwrap();
    }

    let _span = info_span!("formatting").entered();
    run_cargo_fmt("icrate");
}

fn load_config(manifest_dir: &Path) -> Config {
    let _span = info_span!("loading config").entered();

    Config::from_file(&manifest_dir.join("translation-config.toml")).expect("read config")
}

fn parse_sdk(index: &Index<'_>, sdk: &SdkPath, llvm_target: &str, config: &Config) -> Output {
    let tu = get_translation_unit(index, sdk, llvm_target);

    let mut preprocessing = true;
    let mut result = Output::from_libraries(config.libraries.keys());

    let mut library_span = None;
    let mut library_span_name = String::new();
    let mut file_span = None;
    let mut file_span_name = String::new();

    let mut context = Context::new(config, sdk);

    tu.get_entity().visit_children(|entity, _parent| {
        let _span = trace_span!("entity", ?entity).entered();
        if let Some((library_name, file_name)) = context.get_library_and_file_name(&entity) {
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
                        let name = entity.get_name().expect("macro name");
                        let location = entity.get_location().expect("macro location");
                        context
                            .macro_invocations
                            .insert(location.get_spelling_location(), name);
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
                        let file = library.files.get_mut(&file_name).expect("file");
                        for stmt in Stmt::parse(&entity, &context) {
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
            "-fapinotes",
            "-isysroot",
            sdk.path.to_str().unwrap(),
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
