#![allow(clippy::collapsible_else_if)]

#[macro_use]
extern crate tracing;

mod availability;
mod clang_utils;
mod command;
mod context;
mod display_helper;
pub mod documentation;
mod expr;
mod global_analysis;
mod id;
mod library;
mod method;
mod module;
mod name_translation;
mod objc2_utils;
mod protocol;
mod rust_type;
mod stmt;
mod thread_safety;
mod unexposed_attr;

use std::fmt::Write as _;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use apple_sdk::{AppleSdk, DeveloperDirectory, Platform, SdkPath, SimpleSdk};
use clang::diagnostic::Severity;
use clang::{Clang, EntityKind, EntityVisitResult, Index, TranslationUnit};
use clap::Parser;
use tracing::level_filters::LevelFilter;
use tracing::{debug_span, error, info, info_span, trace, trace_span, warn};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::layer::{Layer, SubscriberExt};
use tracing_subscriber::registry::Registry;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_tree::HierarchicalLayer;
use translation_config::{Config, LibraryConfig, PlatformCfg};

use self::availability::HOST_MACOS;
use self::clang_utils::immediate_children;
use self::command::run_cargo_fmt;
use self::context::{Context, MacroEntity, MacroLocation};
use self::documentation::EXTRA_BLOCK_COMMANDS;
use self::global_analysis::global_analysis;
use self::id::{ItemIdentifier, Location};
use self::library::Library;
use self::stmt::Stmt;

type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The framework/library to output.
    ///
    /// The special value "all" means to parse and emit all frameworks.
    framework: String,
}

fn main() -> Result<(), BoxError> {
    // Run with `RUST_LOG=debug RUST_LOG_NO_DEFERRED=1` to get more context
    // for errors.
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    Registry::default()
        .with(
            HierarchicalLayer::new(2)
                .with_targets(false)
                .with_indent_lines(true)
                .with_deferred_spans(std::env::var_os("RUST_LOG_NO_DEFERRED").is_some())
                .with_filter(filter),
        )
        .init();

    let cli = Cli::parse();

    // Normalize framework name
    let framework = cli
        .framework
        .to_lowercase()
        .replace("-", "")
        .replace("_", "");
    let framework = if framework == "all" {
        None
    } else {
        Some(framework.strip_prefix("objc2").unwrap_or(&framework))
    };

    let _span = info_span!("running").entered();

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir.parent().unwrap().parent().unwrap();

    let span = info_span!("loading configs").entered();
    let config = Config::load()?;
    drop(span);

    clang_sys::load()?;
    info!(clang_version = clang::get_version());

    let clang = Clang::new()?;
    let index = Index::new(&clang, true, true);

    let developer_dir = DeveloperDirectory::from_env()
        .unwrap()
        .or_else(|| DeveloperDirectory::from_xcode_select_paths().unwrap())
        .or_else(DeveloperDirectory::default_xcode)
        .expect("could not find developer directory. Pass DEVELOPER_DIR=...");

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
                panic!("found multiple sdks {sdks:?} in {:?}", *platform);
            }
            sdks[0].sdk_path()
        })
        .collect();

    if sdks.len() != 10 {
        error!("should have one of each platform: {sdks:?}");
    }

    let tempdir = workspace_dir.join("target").join("header-translator");
    fs::create_dir_all(&tempdir)?;

    let span = info_span!("updating various project metadata").entered();
    translation_config::update_metadata(&config);
    drop(span);

    let mut found = false;
    for (name, data) in config.to_parse() {
        if let Some(framework) = &framework {
            if *framework != name.to_lowercase() {
                continue; // Skip if filter requested
            }
        }
        found = true;

        let _span = info_span!("framework", name).entered();
        let library = parse_library(&index, &config, data, name, &sdks, &tempdir);
        output_library(workspace_dir, name, &library, &config).unwrap();
    }
    if !found {
        panic!("failed finding framework {}", cli.framework);
    }

    let span = info_span!("updating test-frameworks").entered();
    update_test_imports(workspace_dir, &config);
    run_cargo_fmt(["test-frameworks"]);
    drop(span);

    Ok(())
}

fn parse_library(
    index: &Index<'_>,
    config: &Config,
    data: &LibraryConfig,
    name: &str,
    sdks: &[SdkPath],
    tempdir: &Path,
) -> Library {
    let mut result = None;

    // Find preferred SDK, to hackily support UIKit. For speed, we currently
    // only parse each module once in total (though in the future we'll have
    // to parse it multiple times, and compare the result).
    let sdk = sdks.iter().find(|&sdk| {
        let platform = &sdk.platform;
        // Order of preference
        if data.macos.is_some() {
            *platform == Platform::MacOsX
        } else if data.ios.is_some() {
            *platform == Platform::IPhoneOs
        } else if data.maccatalyst.is_some() {
            *platform == Platform::MacOsX
        } else if data.tvos.is_some() {
            *platform == Platform::AppleTvOs
        } else if data.watchos.is_some() {
            *platform == Platform::WatchOs
        } else if data.visionos.is_some() {
            *platform == Platform::XrOs
        } else {
            panic!("no supported SDK: {sdk:?}")
        }
    });
    let sdk = sdk.expect("find SDK");

    let llvm_targets: &[_] = match &sdk.platform {
        Platform::MacOsX => {
            if data.macos.is_some() {
                &[
                    "arm64-apple-macosx10.12.0",
                    // "arm64-apple-macosx11.0.0",
                    // "i386-apple-macosx10.12.0",
                ]
            } else {
                &["arm64-apple-ios13.1.0-macabi"]
            }
        }
        Platform::IPhoneOs => &[
            "arm64-apple-ios10.0.0",
            // "armv7s-apple-ios10.0.0",
        ],
        Platform::AppleTvOs => &[
            "arm64-apple-tvos",
            // "x86_64-apple-tvos",
        ],
        Platform::WatchOs => &[
            "arm64-apple-watchos",
            // "arm64_32-apple-watchos",
            // "armv7k-apple-watchos",
        ],
        Platform::XrOs => &["arm64-apple-xros"],
        _ => unimplemented!("SDK platform {sdk:?}"),
    };

    for llvm_target in llvm_targets {
        let _span = info_span!("target", platform = ?sdk.platform, llvm_target).entered();

        let mut context = Context::new(config, name);
        let mut library = Library::new(name, data);
        let tu = get_translation_unit(index, sdk, llvm_target, data, tempdir);
        parse_translation_unit(tu, &mut context, &mut library);
        global_analysis(&mut library, config);

        if let Some(prev_result) = &result {
            // Ensure that each target produces the same result.
            assert_eq!(*prev_result, library);
        } else {
            result = Some(library);
        }
    }

    result.unwrap()
}

fn output_library(
    workspace_dir: &Path,
    library_name: &str,
    library: &Library,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let span = info_span!("writing").entered();

    let crate_dir = workspace_dir
        .join(if library.data.is_library {
            "crates"
        } else {
            "framework-crates"
        })
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

    let test_crate_dir = workspace_dir.join("crates").join("test-frameworks");
    library.output(&crate_dir, &test_crate_dir, config)?;

    drop(span);

    let _span = info_span!("formatting").entered();
    run_cargo_fmt([&library.data.krate]);

    Ok(())
}

fn parse_translation_unit(
    tu: TranslationUnit<'_>,
    context: &mut Context<'_>,
    library: &mut Library,
) {
    let _span = info_span!("parsing").entered();
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
                let location = Location::from_file(file, context);
                if context.module_configs(&location).any(|c| c.skipped) {
                    return EntityVisitResult::Continue;
                }
                if location.library_name() == library.data.framework {
                    library.add_module(location);
                }
            }
            EntityKind::MacroExpansion if preprocessing => {
                let entity = MacroEntity::from_entity(&entity, context, false);
                context
                    .macro_invocations
                    .insert(MacroLocation::from_location(&location), entity);
            }
            EntityKind::MacroDefinition if preprocessing => {
                let macro_entity = MacroEntity::from_entity(&entity, context, true);
                context
                    .macro_invocations
                    .insert(MacroLocation::from_location(&location), macro_entity);

                let Some(file) = location.get_expansion_location().file else {
                    // Ignore built-in macro definitions.
                    return EntityVisitResult::Continue;
                };

                let location = Location::from_file(file, context);

                // Only parse if the module is the current one and is not skipped.
                if location.library_name() != library.data.framework
                    || context.module_configs(&location).any(|c| c.skipped)
                {
                    return EntityVisitResult::Continue;
                }

                let module = library.module.submodule_mut(location);
                if let Some(stmt) = Stmt::parse_macro_definition(&entity, context) {
                    context.ident_mapping.extend(stmt.get_ident_mapping());
                    module.add_stmt(stmt);
                }
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
                let location = Location::from_file(file, context);

                // Don't try to parse if the entire module, or supermodule, is skipped.
                if context.module_configs(&location).any(|c| c.skipped) {
                    return EntityVisitResult::Continue;
                }

                let module = library.module.submodule_mut(location);
                for stmt in Stmt::parse(&entity, context, &library.data) {
                    context.ident_mapping.extend(stmt.get_ident_mapping());
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
    data: &LibraryConfig,
    tempdir: &Path,
) -> TranslationUnit<'c> {
    let _span = info_span!("initializing translation unit").entered();

    // Example values:
    // "usr/include/TargetConditionals.modulemap"
    // "System/Library/Frameworks/CoreFoundation.framework/Modules/module.modulemap"
    // "usr/include/ObjectiveC.modulemap"
    // "usr/include/dispatch.modulemap"
    let modulemap = data.modulemap.clone().unwrap_or_else(|| {
        format!(
            "System/Library/Frameworks/{}.framework/Modules/module.modulemap",
            data.framework
        )
    });

    // On Mac Catalyst, we need to try to load from System/iOSSupport first.
    let mut path = sdk.path.join(&modulemap);
    if llvm_target.contains("macabi") {
        let ios_path = sdk.path.join("System/iOSSupport").join(&modulemap);
        if ios_path.exists() {
            path = ios_path;
        }
    }

    // Certain developer frameworks like XCTest are found in the platform
    // path instead of the SDK path.
    let platform_framework_path = sdk
        .path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("Library/Frameworks");

    let platform_path = platform_framework_path.join(format!(
        "{}.framework/Modules/module.modulemap",
        data.framework
    ));
    if data.located_outside_sdk {
        path = platform_path;
    }

    // Find the framework module name
    let module = if data.modulemap.is_none() {
        let re = regex::Regex::new(r"(?m)^framework +module +(\w*)").unwrap();
        let contents = fs::read_to_string(&path).expect("read module map");
        let mut captures = re.captures_iter(&contents);
        let module = &captures.next().expect("module name in module map")[1];
        assert_eq!(captures.count(), 0);
        module.to_string()
    } else {
        // Assume the name is the same as the "framework" name.
        // (dispatch.modulemap has both Dispatch and DispatchIntrospection).
        data.framework.clone()
    };

    let comment_block_commands = format!(
        "-fcomment-block-commands={}",
        EXTRA_BLOCK_COMMANDS.join(",")
    );

    let cache_path = format!("-fmodules-cache-path={}", tempdir.to_str().unwrap());
    let module_name = format!("-fmodule-name={module}");
    let mut arguments = vec![
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
        // We're parsing system headers, but still want comments from there.
        //
        // See: https://clang.llvm.org/docs/UsersManual.html#comment-parsing-options
        "-fretain-comments-from-system-headers",
        // Make Clang a bit better at parsing Apple's documentation.
        &comment_block_commands,
        // Tell Clang to parse non-doc comments too.
        // "-fparse-all-comments",
        // Explicitly pass the sysroot (we aren't invoked through
        // `/usr/bin/clang` which is what usually passes it).
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
        &cache_path,
        "-Xclang",
        "-emit-module",
        &module_name,
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
        // Make properties that are nonatomic on iOS only be nonatomic
        // everywhere; this is the safe default, we can consider `cfg`-gating
        // these in the future.
        "-D",
        "NS_NONATOMIC_IOSONLY=nonatomic",
    ];

    // Add include paths for Mac Catalyst
    let ios_include = sdk.path.join("System/iOSSupport/usr/include");
    let ios_frameworks = sdk.path.join("System/iOSSupport/System/Library/Frameworks");
    if llvm_target.contains("macabi") {
        arguments.extend(&[
            "-isystem",
            ios_include.to_str().unwrap(),
            "-iframework",
            ios_frameworks.to_str().unwrap(),
        ]);
    }

    if data.located_outside_sdk {
        arguments.extend(&[
            "-iframework",
            platform_framework_path.as_os_str().to_str().unwrap(),
        ]);
    }

    arguments.extend(data.flags.iter().map(|flag| &**flag));

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
        .arguments(&arguments)
        .parse()
        .unwrap();

    for diag in tu.get_diagnostics() {
        let location = diag.get_location().get_spelling_location();
        let location = format!(
            "{}:{}:{}",
            location
                .file
                .map(|f| f.get_path())
                .unwrap_or_else(|| "unknown".into())
                .to_string_lossy(),
            location.line,
            location.column
        );
        let text = diag.get_text();
        match diag.get_severity() {
            Severity::Ignored => trace!("{location}: {text}"),
            Severity::Note => info!("{location}: {text}"),
            Severity::Warning => warn!("{location}: {text}"),
            Severity::Error | Severity::Fatal => error!("{location}: {text}"),
        }
    }

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

/// Update `test-frameworks/src/imports.rs`.
fn update_test_imports(workspace_dir: &Path, config: &Config) {
    let test_crate_dir = workspace_dir.join("crates").join("test-frameworks");

    // Write imports
    let mut s = String::new();
    for (_, lib) in config.to_parse() {
        if lib.located_outside_sdk {
            continue;
        }
        if let Some(macos) = &lib.macos {
            if (HOST_MACOS as u64) < macos.major {
                // Skip library if not available on current host.
                continue;
            }
        }
        let platform_cfg = PlatformCfg::from_config_explicit(lib);
        if let Some(cfgs) = platform_cfg.cfgs() {
            writeln!(&mut s, "#[cfg({cfgs})]",).unwrap();
        }
        writeln!(&mut s, "pub use {}::*;", lib.krate.replace('-', "_")).unwrap();
    }
    fs::write(test_crate_dir.join("src").join("imports.rs"), s).unwrap();
}
