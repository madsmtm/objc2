use std::env;
use std::io;
use std::path::{Path, PathBuf};

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

fn run_mode(mode: &'static str) {
    let mut config = compiletest_rs::Config::default();

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

    config.mode = mode.parse().expect("Invalid mode");
    config.src_base = PathBuf::from(format!("tests/{}", mode));
    config.target_rustcflags = Some(format!(
        "-L dependency={} --edition=2018 --extern objc2={}",
        deps_dir.display(),
        dir.display()
    ));

    compiletest_rs::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("ui");
}
