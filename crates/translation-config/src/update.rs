use std::fs;
use std::io::{self, Seek, Write};
use std::path::Path;

use crate::{Config, PlatformCfg, VERSION};

/// Update various project metadata.
pub fn update_metadata(config: &Config) {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir.parent().unwrap().parent().unwrap();

    update_root_cargo_toml(workspace_dir, config);
    update_frameworks_list_data(workspace_dir, config).expect("failed updating list");
    update_frameworks_list_unsupported(workspace_dir, config).expect("failed updating list");
    update_test_metadata(workspace_dir, config);
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

/// Update `test-frameworks/Cargo.toml`.
fn update_test_metadata(workspace_dir: &Path, config: &Config) {
    let test_crate_dir = workspace_dir.join("crates").join("test-frameworks");

    let mut f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(test_crate_dir.join("Cargo.toml"))
        .unwrap();
    let mut cargo_toml: toml_edit::DocumentMut = io::read_to_string(&f)
        .unwrap()
        .parse()
        .expect("invalid test toml");

    let mut features = toml_edit::Array::new();
    for (_, lib) in config.to_parse() {
        if lib.located_outside_sdk {
            continue;
        }
        // Add feature per crate.
        //
        // This is required for some reason for `cargo run --example` to work
        // nicely in our workspace.
        let mut crate_features = vec![format!("dep:{}", lib.krate)];

        // Add non-default features.
        let path = workspace_dir
            .join(if lib.is_library {
                "crates"
            } else {
                "framework-crates"
            })
            .join(&lib.krate)
            .join("Cargo.toml");
        let crate_cargo_toml: toml_edit::DocumentMut = fs::read_to_string(path)
            .unwrap()
            .parse()
            .expect("invalid test toml");
        let docs_rs = &crate_cargo_toml["package"]["metadata"]["docs"]["rs"];
        if let Some(non_default) = docs_rs.get("features") {
            let non_default = non_default.as_array().unwrap();
            for item in non_default {
                let item = item.as_str().unwrap();
                crate_features.push(format!("{}?/{item}", lib.krate));
            }
        }

        cargo_toml["features"][&lib.krate] = toml_edit::Array::from_iter(crate_features).into();

        features.push(lib.krate.to_string());
        // Inserting into array removes decor, so set it afterwards
        features
            .get_mut(features.len() - 1)
            .unwrap()
            .decor_mut()
            .set_prefix("\n    ");
    }
    features.set_trailing("\n");
    features.set_trailing_comma(true);
    cargo_toml["features"]["test-frameworks"] = features.into();

    // Reset dependencies
    cargo_toml["dependencies"] = toml_edit::Item::Table(toml_edit::Table::from_iter([
        (
            "block2",
            toml_edit::Value::InlineTable(toml_edit::InlineTable::from_iter([
                ("workspace", toml_edit::Value::from(true)),
                ("default-features", toml_edit::Value::from(true)),
            ])),
        ),
        (
            "objc2",
            toml_edit::Value::InlineTable(toml_edit::InlineTable::from_iter([
                ("workspace", toml_edit::Value::from(true)),
                ("default-features", toml_edit::Value::from(true)),
                // FIXME: Make these not required for tests
                (
                    "features",
                    toml_edit::Value::Array(toml_edit::Array::from_iter(["relax-sign-encoding"])),
                ),
            ])),
        ),
        (
            "libc",
            toml_edit::Value::InlineTable(toml_edit::InlineTable::from_iter([
                ("workspace", toml_edit::Value::from(true)),
                ("default-features", toml_edit::Value::from(true)),
            ])),
        ),
    ]));
    let _ = cargo_toml.remove("target");

    for (_, lib) in config.to_parse() {
        if lib.located_outside_sdk {
            continue;
        }
        let platform_cfg = PlatformCfg::from_config_explicit(lib);

        let dependencies = if let Some(cfgs) = platform_cfg.cfgs() {
            let key = format!("'cfg({cfgs})'").parse().unwrap();
            implicit_table(
                implicit_table(implicit_table(cargo_toml.entry("target")).entry_format(&key))
                    .entry("dependencies"),
            )
        } else {
            cargo_toml["dependencies"].as_table_mut().unwrap()
        };

        dependencies[&lib.krate] = toml_edit::InlineTable::from_iter([
            ("workspace", toml_edit::Value::from(true)),
            ("optional", toml_edit::Value::from(true)),
            ("default-features", toml_edit::Value::from(true)),
        ])
        .into();
    }

    f.set_len(0).unwrap();
    f.seek(io::SeekFrom::Start(0)).unwrap();
    f.write_all(cargo_toml.to_string().as_bytes()).unwrap();
}

fn implicit_table(entry: toml_edit::Entry<'_>) -> &mut toml_edit::Table {
    entry
        .or_insert_with(|| {
            let mut table = toml_edit::Table::new();
            table.set_implicit(true);
            toml_edit::Item::Table(table)
        })
        .as_table_mut()
        .unwrap()
}
