use std::collections::{BTreeMap, BTreeSet};
use std::io::{ErrorKind, Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

use apple_sdk::{AppleSdk, DeveloperDirectory, Platform, SdkPath, SimpleSdk};
use clang::{Clang, EntityKind, EntityVisitResult, Index, TranslationUnit};
use semver::VersionReq;
use tracing::{debug_span, error, info, info_span, trace, trace_span};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::{Layer, SubscriberExt};
use tracing_subscriber::registry::Registry;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_tree::HierarchicalLayer;

use header_translator::{
    global_analysis, run_cargo_fmt, Config, Context, Library, LibraryConfig, Stmt,
};

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
    let workspace_dir = manifest_dir.parent().unwrap().parent().unwrap();

    let config = load_config(workspace_dir)?;

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

    if sdks.len() != 10 {
        error!("should have one of each platform: {sdks:?}");
    }

    let mut libraries = BTreeMap::new();

    let tempdir = workspace_dir.join("target").join("header-translator");
    fs::create_dir_all(&tempdir)?;

    // TODO: Compare between SDKs
    for sdk in sdks {
        // These are found using the `get_llvm_targets.fish` helper script
        let (llvm_targets, platform_header, platform_config_filter): (
            &[_],
            _,
            fn(&LibraryConfig) -> bool,
        ) = match &sdk.platform {
            Platform::MacOsX => (
                &[
                    // "x86_64-apple-macosx10.12.0",
                    "arm64-apple-macosx11.0.0",
                    // "i686-apple-macosx10.12.0",
                ],
                "macos.h",
                |config| config.macos.is_some(),
            ),
            Platform::IPhoneOs => (
                &[
                    "arm64-apple-ios10.0.0",
                    // "armv7s-apple-ios10.0.0",
                    // "arm64-apple-ios14.0-macabi",
                    // "x86_64-apple-ios13.0-macabi",
                ],
                "ios.h",
                |config| config.ios.is_some(),
            ),
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

        let mut result: Option<BTreeMap<String, Library>> = None;

        let includes = tempdir.join(platform_header);

        let mut includes_file = fs::File::create(&includes).unwrap();
        for lib in config.libraries.values() {
            if !platform_config_filter(lib) {
                continue;
            }
            if let Some(umbrella_header) = &lib.umbrella_header {
                writeln!(
                    &mut includes_file,
                    "#import <{}/{}>",
                    lib.framework, umbrella_header
                )?;
            } else {
                writeln!(
                    &mut includes_file,
                    "#import <{}/{}.h>",
                    lib.framework, lib.framework,
                )?;
            }
        }
        includes_file.flush().unwrap();
        drop(includes_file);

        for llvm_target in llvm_targets {
            let _span = info_span!("parsing", platform = ?sdk.platform, llvm_target).entered();

            let curr_result = parse_sdk(&index, &sdk, llvm_target, &config, &includes);

            if let Some(prev_result) = &result {
                // Ensure that each target produces the same result.
                assert_eq!(*prev_result, curr_result);
            } else {
                result = Some(curr_result);
            }
        }

        let result = result.unwrap();

        // Hacky way to support UIKit
        match sdk.platform {
            Platform::MacOsX => {
                for (name, library) in result {
                    if library.data.macos.is_some() {
                        libraries.insert(name, library);
                    }
                }
            }
            Platform::IPhoneOs => {
                for (name, library) in result {
                    if library.data.macos.is_none() {
                        libraries.insert(name, library);
                    }
                }
            }
            _ => {}
        }
    }

    let span = info_span!("analyzing").entered();
    for (name, library) in &mut libraries {
        let _span = debug_span!("library", name).entered();
        global_analysis(library);
    }
    drop(span);

    for (library_name, library) in &libraries {
        let _span = info_span!("writing", library_name).entered();

        let crate_dir = workspace_dir
            .join("framework-crates")
            .join(&library.data.krate);

        // Ensure directories exist
        let generated_dir = workspace_dir.join("generated").join(library_name);
        fs::create_dir_all(&generated_dir)?;
        fs::create_dir_all(&crate_dir.join("src"))?;

        // Recreate symlink to generated directory
        let symlink_path = crate_dir.join("src").join("generated");
        match fs::remove_file(&symlink_path) {
            Ok(()) => {}
            Err(err) if err.kind() == ErrorKind::NotFound => {}
            Err(err) => Err(err)?,
        }
        #[cfg(unix)]
        let res =
            std::os::unix::fs::symlink(format!("../../../generated/{library_name}"), &symlink_path);
        #[cfg(windows)]
        let res = std::os::windows::fs::symlink_dir(
            format!("..\\..\\..\\generated\\{library_name}"),
            &symlink_path,
        );
        match res {
            Ok(()) => {}
            Err(err) if err.kind() == ErrorKind::AlreadyExists => {}
            Err(err) => Err(err)?,
        }

        library.output(&crate_dir, &config)?;
    }

    let span = info_span!("formatting").entered();
    run_cargo_fmt(libraries.values().map(|library| &library.data.krate));
    drop(span);

    update_ci(workspace_dir, &config)?;

    update_list(workspace_dir, &config)?;

    Ok(())
}

fn load_config(workspace_dir: &Path) -> Result<Config, BoxError> {
    let _span = info_span!("loading configs").entered();

    let mut libraries = BTreeMap::default();

    for dir in fs::read_dir(workspace_dir.join("framework-crates"))? {
        let dir = dir?;
        if !dir.file_type()?.is_dir() {
            continue;
        }
        let path = dir.path().join("translation-config.toml");
        let config =
            LibraryConfig::from_file(&path).unwrap_or_else(|e| panic!("read {path:?} config: {e}"));
        assert_eq!(*config.krate, *dir.file_name());
        libraries.insert(config.framework.to_string(), config);
    }

    let path = workspace_dir
        .join("crates")
        .join("header-translator")
        .join("system-config.toml");
    let system = LibraryConfig::from_file(&path).expect("read system config");

    Ok(Config { libraries, system })
}

fn parse_sdk(
    index: &Index<'_>,
    sdk: &SdkPath,
    llvm_target: &str,
    config: &Config,
    includes: &Path,
) -> BTreeMap<String, Library> {
    let tu = get_translation_unit(index, sdk, llvm_target, includes);

    let mut preprocessing = true;
    let mut libraries: BTreeMap<String, Library> = config
        .libraries
        .iter()
        .map(|(name, data)| (name.into(), Library::new(name, data)))
        .collect();

    let mut library_span: Option<(_, String)> = None;
    let mut file_span: Option<(_, _)> = None;

    let mut context = Context::new(config, sdk);

    tu.get_entity().visit_children(|entity, _parent| {
        let _span = trace_span!("entity", ?entity).entered();
        if let Some(location) = context.get_location(&entity) {
            let library_name = location.library_name();
            if library_span.as_ref().map(|(_, s)| &**s) != Some(library_name) {
                library_span = Some((
                    debug_span!("library", name = library_name).entered(),
                    library_name.to_string(),
                ));
                file_span.take();
            }
            if file_span.as_ref().map(|(_, l)| l) != Some(&location) {
                file_span = Some((debug_span!("file", ?location).entered(), location.clone()));
            }

            if let Some(library) = libraries.get_mut(library_name) {
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
                            if included != *library_name {
                                // The file is often included twice, even
                                // within the same file, so insertion can fail
                                library.add_module(vec![included])
                            }
                        }
                    }
                    EntityKind::MacroExpansion if preprocessing => {
                        let clang_location = entity.get_location().expect("macro location");
                        context
                            .macro_invocations
                            .insert(clang_location.get_spelling_location(), entity);
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
                        for stmt in Stmt::parse(&entity, &context) {
                            let module = library.module_mut(&location);
                            module.add_stmt(stmt);
                        }
                    }
                }
            } else {
                trace!("library not found");
            }
        }
        EntityVisitResult::Continue
    });

    libraries
}

fn get_translation_unit<'i: 'tu, 'tu>(
    index: &'i Index<'tu>,
    sdk: &SdkPath,
    llvm_target: &str,
    includes: &Path,
) -> TranslationUnit<'tu> {
    let _span = info_span!("initializing translation unit").entered();

    let tu = index
        .parser(includes)
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
            "-target",
            llvm_target,
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

fn update_ci(workspace_dir: &Path, config: &Config) -> io::Result<()> {
    let _span = info_span!("updating ci.yml").entered();
    let mut ci = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(workspace_dir.join(".github/workflows/ci.yml"))?;
    // find the features section
    let mut text = String::new();
    ci.read_to_string(&mut text)?;
    let (before, after) = text
        .split_once("BEGIN AUTOMATICALLY GENERATED")
        .expect("begin section not found in ci.yml");
    let (_, after) = after
        .split_once("  # END AUTOMATICALLY GENERATED")
        .expect("end section not found in ci.yml");

    // Clear file
    ci.set_len(0)?;
    ci.seek(io::SeekFrom::Start(0))?;

    writeln!(ci, "{before}BEGIN AUTOMATICALLY GENERATED")?;

    fn writer(
        mut ci: impl Write,
        config: &Config,
        env_name: &str,
        check: impl Fn(&LibraryConfig) -> bool,
    ) -> io::Result<()> {
        // Use a BTreeSet to sort the libraries
        let mut frameworks = BTreeSet::new();
        for library in config.libraries.values() {
            if check(library) {
                frameworks.insert(&*library.krate);
            }
        }
        write!(ci, "  {env_name}:")?;
        for framework in frameworks {
            write!(ci, " --package={}", framework)?;
        }
        writeln!(ci)?;

        Ok(())
    }

    writer(&mut ci, config, "FRAMEWORKS_MACOS_10_12", |lib| {
        lib.macos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=10.12").unwrap().matches(v))
            // HACK: These depend on `objc2-uniform-type-identifiers`, which
            // is not available on macOS 10.12, but will be enabled by `"all"`
            && !["objc2-file-provider", "objc2-health-kit", "objc2-photos"].contains(&&*lib.krate)
    })?;
    writer(&mut ci, config, "FRAMEWORKS_MACOS_10_13", |lib| {
        lib.macos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=10.13").unwrap().matches(v))
            // HACK: These depend on `objc2-uniform-type-identifiers`, which
            // is not available on macOS 10.13, but will be enabled by `"all"`
            && !["objc2-file-provider", "objc2-health-kit", "objc2-photos"].contains(&&*lib.krate)
    })?;
    writer(&mut ci, config, "FRAMEWORKS_MACOS_11", |lib| {
        lib.macos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=11.0").unwrap().matches(v))
    })?;
    writer(&mut ci, config, "FRAMEWORKS_MACOS_12", |lib| {
        lib.macos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=12.0").unwrap().matches(v))
    })?;
    writer(&mut ci, config, "FRAMEWORKS_MACOS_13", |lib| {
        lib.macos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=13.0").unwrap().matches(v))
    })?;
    writer(&mut ci, config, "FRAMEWORKS_MACOS_14", |lib| {
        lib.macos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=14.0").unwrap().matches(v))
    })?;
    writer(&mut ci, config, "FRAMEWORKS_IOS_10", |lib| {
        // HACK: We can't test iOS frameworks with the `"all"` feature, as
        // that enables `objc2-app-kit` as well.
        matches!(
            &*lib.krate,
            "objc2-foundation" | "objc2-metal" | "objc2-ui-kit"
        )
    })?;
    writer(&mut ci, config, "FRAMEWORKS_GNUSTEP", |lib| {
        lib.gnustep
            && ["objc2-app-kit", "objc2-foundation", "objc2-core-data"].contains(&&*lib.krate)
    })?;

    write!(&mut ci, "  # END AUTOMATICALLY GENERATED{after}")?;

    Ok(())
}

fn update_list(workspace_dir: &Path, config: &Config) -> io::Result<()> {
    let _span = info_span!("updating list_data.md").entered();

    let mut f = fs::File::create(
        workspace_dir.join("crates/objc2/src/topics/about_generated/list_data.md"),
    )?;

    writeln!(f, "| Framework | Crate | Documentation |")?;
    writeln!(f, "| --- | --- | --- |")?;

    for (name, library) in &config.libraries {
        let package = &library.krate;
        writeln!(
            f,
            "| `{name}` | [![`{package}`](https://badgen.net/crates/v/{package})](https://crates.io/crates/{package}) | [![docs.rs](https://docs.rs/{package}/badge.svg)](https://docs.rs/{package}/) |",
        )?;
    }

    Ok(())
}
