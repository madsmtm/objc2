use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Read, Seek, Write};
use std::path::Path;

use semver::VersionReq;

use crate::{Config, LibraryConfig, VERSION};

/// Update various project metadata.
///
/// FIXME: Doesn't update `test-frameworks` metadata yet.
pub fn update_metadata(config: &Config) {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir.parent().unwrap().parent().unwrap();

    update_root_cargo_toml(workspace_dir, config);
    update_frameworks_list_data(workspace_dir, config).expect("failed updating list");
    update_frameworks_list_unsupported(workspace_dir, config).expect("failed updating list");
    update_ci(workspace_dir, config).unwrap();
}

/// Update root `Cargo.toml`.
fn update_root_cargo_toml(workspace_dir: &Path, config: &Config) {
    let mut f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(workspace_dir.join("Cargo.toml"))
        .unwrap();
    let mut cargo_toml: toml_edit::DocumentMut = io::read_to_string(&f)
        .unwrap()
        .parse()
        .expect("invalid Cargo.toml");

    let dependencies = cargo_toml["workspace"]["dependencies"]
        .as_table_mut()
        .unwrap();

    // Delete all framework crate entries.
    dependencies.retain(|key, _| !key.starts_with("objc2-"));

    // And add them again.
    for (i, data) in config.frameworks().enumerate() {
        let table = toml_edit::InlineTable::from_iter([
            (
                "path",
                toml_edit::Value::from(format!("framework-crates/{}", data.krate)),
            ),
            ("version", toml_edit::Value::from(VERSION)),
            ("default-features", toml_edit::Value::from(false)),
        ]);
        dependencies[&data.krate] = table.into();

        if i == 0 {
            dependencies
                .key_mut(&data.krate)
                .unwrap()
                .leaf_decor_mut()
                .set_prefix("\n##\n## AUTO-GENERATED BELOW\n##\n\n")
        }
    }

    let patch_crates_io = cargo_toml["patch"]["crates-io"].as_table_mut().unwrap();

    // Delete all framework crate entries.
    patch_crates_io.retain(|key, _| !key.starts_with("objc2-"));

    // And add them again.
    for (i, data) in config.frameworks().enumerate() {
        let path = format!("framework-crates/{}", data.krate);

        patch_crates_io[&data.krate] =
            toml_edit::InlineTable::from_iter([("path", toml_edit::Value::from(&path))]).into();

        if i == 0 {
            patch_crates_io
                .key_mut(&data.krate)
                .unwrap()
                .leaf_decor_mut()
                .set_prefix("\n##\n## AUTO-GENERATED BELOW\n##\n\n")
        }
    }

    f.set_len(0).unwrap();
    f.seek(io::SeekFrom::Start(0)).unwrap();
    f.write_all(cargo_toml.to_string().as_bytes()).unwrap();
}

/// Update `frameworks_list_data.md`.
fn update_frameworks_list_data(workspace_dir: &Path, config: &Config) -> io::Result<()> {
    let path = workspace_dir.join("crates/objc2/src/topics/frameworks_list_data.md");
    let mut f = fs::File::create(path)?;

    writeln!(f, "| Framework | Crate | Docs.rs |")?;
    writeln!(f, "| --- | --- | --- |")?;

    for data in config.frameworks() {
        let name = &data.framework;
        let package = &data.krate;
        writeln!(f, "| `{name}` | [`{package}`](https://crates.io/crates/{package}) | [![docs.rs](https://docs.rs/{package}/badge.svg)](https://docs.rs/{package}/) |")?;
    }

    Ok(())
}

/// Update `frameworks_list_unsupported.md`.
fn update_frameworks_list_unsupported(workspace_dir: &Path, config: &Config) -> io::Result<()> {
    let path = workspace_dir.join("crates/objc2/src/topics/frameworks_list_unsupported.md");
    let mut f = fs::File::create(path)?;

    writeln!(f, "| Framework | Why is this unsupported? |")?;
    writeln!(f, "| --- | --- |")?;

    for (framework, why) in &config.skipped {
        writeln!(f, "| `{framework}` | {why}. |")?;
    }

    Ok(())
}

fn update_ci(workspace_dir: &Path, config: &Config) -> io::Result<()> {
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
        for data in config.frameworks() {
            if data.located_outside_sdk {
                continue; // Cannot easily link to these.
            }
            if check(data) {
                frameworks.insert(&*data.krate);
            }
        }
        write!(ci, "  {env_name}:")?;
        for framework in frameworks {
            write!(ci, " --package={}", framework)?;
        }
        writeln!(ci)?;

        Ok(())
    }

    // HACK: Linking `objc2-avf-audio` on older systems is not possible
    // without an SDK that's new enough.
    let uses_avf_audio = |lib: &LibraryConfig| {
        matches!(
            &*lib.krate,
            "objc2-avf-audio"
                | "objc2-av-foundation"
                | "objc2-av-kit"
                | "objc2-media-player"
                | "objc2-photos"
                | "objc2-photos-ui"
                | "objc2-sprite-kit"
                | "objc2-scene-kit"
        )
    };
    // HACK: Cinematic, MediaSetup, etc. aren't available in the simulator.
    // MLCompute and MetalFX are also only available on Aarch64
    let not_on_simulator = |lib: &LibraryConfig| {
        matches!(
            &*lib.krate,
            "objc2-cinematic"
                | "objc2-media-setup"
                | "objc2-thread-network"
                | "objc2-ml-compute"
                | "objc2-metal-fx"
        )
    };

    writer(&mut ci, config, "FRAMEWORKS_MACOS_10_12", |lib| {
        lib.macos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=10.12").unwrap().matches(v))
            && !uses_avf_audio(lib)
            // HACK: PDFKit requires linking Quartz on older systems.
            && !["objc2-pdf-kit"].contains(&&*lib.krate)
            // HACK: iTunesLibrary has a different install name on older systems.
            && !["objc2-itunes-library"].contains(&&*lib.krate)
    })?;
    writer(&mut ci, config, "FRAMEWORKS_MACOS_10_13", |lib| {
        lib.macos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=10.13").unwrap().matches(v))
            && !uses_avf_audio(lib)
            // HACK: PDFKit requires linking Quartz on older systems.
            && !["objc2-pdf-kit"].contains(&&*lib.krate)
            // HACK: iTunesLibrary has a different install name on older systems.
            && !["objc2-itunes-library"].contains(&&*lib.krate)
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
            && !not_on_simulator(lib)
    })?;
    writer(&mut ci, config, "FRAMEWORKS_TVOS_17", |lib| {
        lib.tvos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=17.0").unwrap().matches(v))
            // HACK: MetalPerformanceShadersGraph is not available on tvOS simulator
            && !["objc2-metal-performance-shaders-graph"].contains(&&*lib.krate)
            && !not_on_simulator(lib)
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
            && !not_on_simulator(lib)
    })?;
    writer(&mut ci, config, "FRAMEWORKS_WATCHOS_10", |lib| {
        lib.watchos
            .as_ref()
            .is_some_and(|v| VersionReq::parse("<=10.0").unwrap().matches(v))
            && !not_on_simulator(lib)
    })?;
    writer(&mut ci, config, "FRAMEWORKS_GNUSTEP", |lib| {
        // HACK: CoreFoundation uses mach types that GNUStep doesn't support
        lib.gnustep && lib.krate != "objc2-core-foundation"
    })?;

    write!(&mut ci, "  # END AUTOMATICALLY GENERATED{after}")?;

    Ok(())
}
