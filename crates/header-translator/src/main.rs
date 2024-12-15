use std::collections::{BTreeMap, BTreeSet};
use std::io::{ErrorKind, Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

use apple_sdk::{AppleSdk, DeveloperDirectory, Platform, SdkPath, SimpleSdk};
use clang::{Clang, EntityKind, EntityVisitResult, Index, TranslationUnit};
use semver::VersionReq;
use tracing::{debug_span, error, info, info_span, trace_span};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::{Layer, SubscriberExt};
use tracing_subscriber::registry::Registry;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_tree::HierarchicalLayer;

use header_translator::{
    global_analysis, run_cargo_fmt, Config, Context, Library, LibraryConfig, Location, MacroEntity,
    MacroLocation, Stmt,
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
        let llvm_targets: &[_] = match &sdk.platform {
            Platform::MacOsX => &[
                // "x86_64-apple-macosx10.12.0",
                "arm64-apple-macosx11.0.0",
                // "i686-apple-macosx10.12.0",
            ],
            Platform::IPhoneOs => &[
                "arm64-apple-ios10.0.0",
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

        let mut result: Option<BTreeMap<String, Library>> = None;

        for llvm_target in llvm_targets {
            let _span = info_span!("parsing", platform = ?sdk.platform, llvm_target).entered();

            let curr_result = parse_sdk(&index, &config, &sdk, llvm_target, &tempdir);

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

    let dependency_map: BTreeMap<_, _> = libraries
        .iter()
        .map(|(library_name, library)| (&**library_name, library.dependencies(&config)))
        .collect();

    for (library_name, library) in &libraries {
        let _span = info_span!("writing", library_name).entered();

        let crate_dir = workspace_dir
            .join("framework-crates")
            .join(&library.data.krate);

        // Ensure directories exist
        let generated_dir = workspace_dir.join("generated").join(library_name);
        fs::create_dir_all(generated_dir)?;
        fs::create_dir_all(crate_dir.join("src"))?;

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

        library.output(&crate_dir, &config, &dependency_map)?;
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
    config: &Config,
    sdk: &SdkPath,
    llvm_target: &str,
    tempdir: &Path,
) -> BTreeMap<String, Library> {
    let mut context = Context::new(config);

    config
        .libraries
        .iter()
        .filter(|(_, data)| match &sdk.platform {
            // TODO: Mac Catalyst
            Platform::MacOsX => data.macos.is_some(),
            Platform::IPhoneOs | Platform::IPhoneSimulator => data.ios.is_some(),
            Platform::AppleTvOs | Platform::AppleTvSimulator => data.tvos.is_some(),
            Platform::WatchOs | Platform::WatchSimulator => data.watchos.is_some(),
            Platform::XrOs | Platform::XrOsSimulator => data.visionos.is_some(),
            _ => unimplemented!("unsupported sdk {sdk:?}"),
        })
        .map(|(name, data)| {
            let _span = info_span!("framework", ?name).entered();
            let mut library = Library::new(name, data);
            let tu =
                get_translation_unit(index, sdk, llvm_target, &library.data.framework, tempdir);
            parse_framework(tu, &mut context, &mut library);
            (name.into(), library)
        })
        .collect()
}

fn parse_framework(tu: TranslationUnit<'_>, context: &mut Context<'_>, library: &mut Library) {
    let mut preprocessing = true;
    let mut file_span: Option<(_, _)> = None;

    tu.get_entity().visit_children(|entity, _parent| {
        let location = entity.get_location().expect("entity location");

        let file = location.get_expansion_location().file;
        if file_span.as_ref().map(|(_, l)| l) != Some(&file) {
            // Drop old span
            file_span.take();

            // Enter new span
            let span = if let Some(file) = file {
                if let Some(module) = file.get_module() {
                    debug_span!("module", full_name = module.get_full_name())
                } else {
                    debug_span!("file", path = ?file.get_path())
                }
            } else {
                // System-defined entities (like built-in macros, or
                // inclusion directives generated from the modulemap).
                debug_span!("Clang-defined")
            };
            file_span = Some((span.entered(), file));
        }

        let _span = trace_span!("entity", ?entity).entered();

        match entity.get_kind() {
            EntityKind::InclusionDirective if preprocessing => {
                let file = entity.get_file().expect("inclusion directive has file");
                let location = Location::from_file(file);
                if location.library_name() == library.data.framework {
                    library.add_module(location);
                }
            }
            EntityKind::MacroExpansion if preprocessing => {
                let entity = MacroEntity::from_entity(&entity, context);
                context
                    .macro_invocations
                    .insert(MacroLocation::from_location(&location), entity);
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

                let file = location
                    .get_expansion_location()
                    .file
                    .expect("expanded location file");
                let location = Location::from_file(file);

                let module = library.module_mut(location);
                for stmt in Stmt::parse(&entity, context) {
                    module.add_stmt(stmt);
                }
            }
        }

        EntityVisitResult::Continue
    });
}

fn get_translation_unit<'i: 'c, 'c>(
    index: &'i Index<'c>,
    sdk: &SdkPath,
    llvm_target: &str,
    framework: &str,
    tempdir: &Path,
) -> TranslationUnit<'c> {
    let _span = info_span!("initializing translation unit").entered();

    // "usr/include/TargetConditionals.modulemap"
    // "System/Library/Frameworks/CoreFoundation.framework/Modules/module.modulemap"
    // "usr/include/ObjectiveC.modulemap"
    let path = sdk.path.join(format!(
        "System/Library/Frameworks/{framework}.framework/Modules/module.modulemap"
    ));
    let modulemap = fs::read_to_string(&path).expect("read module map");
    let re = regex::Regex::new(r"(?m)^framework +module +(\w*)").unwrap();

    // Find the top-level framework
    let mut captures = re.captures_iter(&modulemap);
    let module = &captures.next().expect("module name in module map")[1];
    assert_eq!(captures.count(), 0);

    let tu = index
        .parser(path.to_str().unwrap())
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
            "-fexceptions",
            "-fobjc-exceptions",
            "-fobjc-abi-version=2", // 3??
            "-fblocks",
            // "-fparse-all-comments",
            // TODO: "-fretain-comments-from-system-headers"
            "-isysroot",
            sdk.path.to_str().unwrap(),
            // See ClangImporter.cpp and Foundation/NSObjCRuntime.h
            "-D",
            "__SWIFT_ATTR_SUPPORTS_SENDABLE_DECLS=1",
            "-D",
            "__SWIFT_ATTR_SUPPORTS_SENDING=1",
            // "-D",
            // "__swift__=51000",
            // Enable modules. We do this by parsing the `.modulemap` instead
            // of a combined file containing includes, as the Clang AST from
            // dependent modules does not seem possible to access otherwise.
            //
            // The magic here is passing `-emit-module` to the frontend.
            //
            // See:
            // https://clang.llvm.org/docs/Modules.html
            // https://clang.llvm.org/docs/PCHInternals.html
            "-fmodules",
            "-fimplicit-module-maps",
            // "-Xclang",
            // "-fmodule-format=raw",
            &format!("-fmodules-cache-path={}", tempdir.to_str().unwrap()),
            "-Xclang",
            "-emit-module",
            &format!("-fmodule-name={module}"),
            "-fsystem-module",
            // "-fmodules-validate-system-headers",
            // "-fmodules-search-all",
            "-Xclang",
            "-fno-modules-prune-non-affecting-module-map-files",
            // "-Xclang",
            // "-fmodule-feature",
            // "-Xclang",
            // "swift",
            "-disable-objc-default-synthesize-properties",
            // Explicitly enable API notes (implicitly enabled by -fmodules).
            "-fapinotes",
            "-fapinotes-modules",
            // "-fapi-notes-swift-version=6.0",
            // Make AudioToolbox less dependent on CoreServices
            "-DAUDIOCOMPONENT_NOCARBONINSTANCES=1",
        ])
        .parse()
        .unwrap();

    // dbg!(&tu);
    // dbg!(tu.get_entity().get_children());
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
            && !["objc2-app-kit", "objc2-file-provider", "objc2-health-kit", "objc2-photos"].contains(&&*lib.krate)
    })?;
    writer(&mut ci, config, "FRAMEWORKS_MACOS_10_13", |lib| {
        lib.macos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=10.13").unwrap().matches(v))
            // HACK: These depend on `objc2-uniform-type-identifiers`, which
            // is not available on macOS 10.13, but will be enabled by `"all"`
            && !["objc2-app-kit", "objc2-file-provider", "objc2-health-kit", "objc2-photos"].contains(&&*lib.krate)
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
    writer(&mut ci, config, "FRAMEWORKS_MACOS_15", |lib| {
        lib.macos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=15.0").unwrap().matches(v))
    })?;
    writer(&mut ci, config, "FRAMEWORKS_IOS_10", |lib| {
        lib.ios
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=10.0").unwrap().matches(v))
    })?;
    writer(&mut ci, config, "FRAMEWORKS_IOS_17", |lib| {
        lib.ios
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=17.0").unwrap().matches(v))
            // HACK: MLCompute and MetalFX are only available on Aarch64
            && !["objc2-ml-compute", "objc2-metal-fx"].contains(&&*lib.krate)
    })?;
    writer(&mut ci, config, "FRAMEWORKS_TVOS_17", |lib| {
        lib.tvos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=17.0").unwrap().matches(v))
    })?;
    writer(&mut ci, config, "FRAMEWORKS_MAC_CATALYST_17", |lib| {
        lib.maccatalyst
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=17.0").unwrap().matches(v))
    })?;
    writer(&mut ci, config, "FRAMEWORKS_VISIONOS_1", |lib| {
        lib.visionos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=1.0").unwrap().matches(v))
    })?;
    writer(&mut ci, config, "FRAMEWORKS_WATCHOS_10", |lib| {
        lib.watchos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=10.0").unwrap().matches(v))
    })?;
    writer(&mut ci, config, "FRAMEWORKS_GNUSTEP", |lib| {
        // HACK: CoreFoundation uses mach types that GNUStep doesn't support
        lib.gnustep && lib.krate != "objc2-core-foundation"
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
