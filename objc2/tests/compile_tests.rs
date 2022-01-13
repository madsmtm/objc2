use std::env;
use std::path::PathBuf;

fn run_mode(mode: &'static str) {
    let mut config = compiletest_rs::Config::default();

    config.mode = mode.parse().expect("Invalid mode");
    config.src_base = PathBuf::from(format!("tests/{}", mode));
    config.verbose = true;
    config.target_rustcflags = Some(format!(
        "-L ../target/{profile} -L ../target/{profile}/deps --edition=2018 --extern objc2",
        profile = env!("BUILD_PROFILE")
    ));
    config.clean_rmeta();

    compiletest_rs::run_tests(&config);
}

#[test]
fn compile_test() {
}
