use std::env;
use std::io;
use std::path::{Path, PathBuf};

use compiletest_rs::{common::Mode, run_tests, Config};

// Environment variables created in build script
const TARGET: &'static str = env!("BUILD_TARGET");
const PROFILE: &'static str = env!("BUILD_PROFILE");

fn find_dependency(deps_dir: &Path, dep: &str) -> io::Result<PathBuf> {
    // Find lib[dep]-XYZ.rlib
    // Yes. This is ugly.
    let mut rlib = None;
    for dir in deps_dir.read_dir()? {
        let path = dir?.path();
        if path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with(&format!("lib{}-", dep))
            && path.extension().map_or(false, |ext| ext == "rlib")
        {
            if let Some(rlib) = rlib {
                panic!("Found multiple rlibs: {:?}, {:?}", rlib, path);
            } else {
                rlib = Some(path);
            }
        }
    }
    Ok(rlib.expect("Found no rlib"))
}

fn run(src: &'static str, mode: Mode) {
    let mut config = Config::default();

    // ../target
    let target_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("target");

    // [/TARGET]/debug/deps
    let deps_dir = if target_dir.join(TARGET).exists() {
        target_dir.join(TARGET)
    } else {
        target_dir
    }
    .join(PROFILE)
    .join("deps");

    let dir = find_dependency(&deps_dir, "objc2").unwrap();

    config.mode = mode;
    config.src_base = PathBuf::from(format!("tests/{}", src));
    config.target_rustcflags = Some(format!(
        "-L dependency={} --extern objc2={}",
        deps_dir.display(),
        dir.display()
    ));
    config.llvm_filecheck = Some(
        env::var("FILECHECK")
            .unwrap_or("FileCheck".to_string())
            .into(),
    );
    config.edition = Some("2018".into());
    config.verbose = true;

    run_tests(&config);
}

#[test]
fn test_ui() {
    run("ui", Mode::Ui);
}

#[test]
fn test_ui_compile_fail() {
    run("ui", Mode::CompileFail);
}

#[test]
fn test_codegen() {
    run("codegen", Mode::Codegen);
}

#[test]
fn test_codegen_pass() {
    run("codegen", Mode::RunPass);
}
