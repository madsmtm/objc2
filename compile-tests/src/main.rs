use cargo_metadata::camino::Utf8PathBuf;
use cargo_metadata::Message;
use std::env;
use std::io;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

use compiletest_rs::{common::Mode, run_tests, Config};

fn get_rlib<'a>(filenames: impl IntoIterator<Item = &'a Utf8PathBuf>) -> &'a Utf8PathBuf {
    filenames
        .into_iter()
        .find(|name| name.extension() == Some("rlib"))
        .expect("An rlib")
}

fn main() -> io::Result<()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    // Build objc2
    let result = Command::new("cargo")
        .current_dir(&manifest_dir.parent().unwrap())
        .arg("build")
        .arg("-pobjc2")
        .arg("--message-format=json-render-diagnostics")
        .args(std::env::args().skip(1))
        .stdout(Stdio::piped())
        .output()?;

    // Extract metadata from build
    let artifacts: Vec<_> = cargo_metadata::Message::parse_stream(&*result.stdout)
        .filter_map(|message| {
            if let Message::CompilerArtifact(artifact) = message.unwrap() {
                if artifact.target.kind == ["lib"] && !artifact.profile.test {
                    return Some(artifact);
                }
            }
            None
        })
        .collect();
    let dep_dir = get_rlib(&artifacts[0].filenames).parent().unwrap();
    let flags = artifacts
        .iter()
        .map(|artifact| {
            format!(
                " --extern {name}={rlib}",
                name = artifact.target.name,
                rlib = get_rlib(&artifact.filenames),
            )
        })
        .collect::<String>();

    let mut config = Config::default();
    config.target_rustcflags = Some(format!(
        "-L dependency={dep}{flags}",
        dep = dep_dir,
        flags = flags,
    ));
    config.llvm_filecheck = Some(
        env::var("FILECHECK")
            .unwrap_or("FileCheck".to_string())
            .into(),
    );
    config.edition = Some("2018".into());
    config.verbose = matches!(
        std::env::var("CARGO_TERM_VERBOSE").as_deref(),
        Ok("true" | "1")
    ) || std::env::args().any(|val| val == "--verbose" || val == "-v");

    // Run UI tests
    config.src_base = manifest_dir.join("ui");
    config.mode = Mode::Ui;
    run_tests(&config);
    config.mode = Mode::CompileFail;
    run_tests(&config);

    // Run Codegen tests
    config.src_base = manifest_dir.join("assembly");
    config.mode = Mode::Assembly;
    run_tests(&config);

    Ok(())
}
