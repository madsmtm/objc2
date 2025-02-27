use std::{env, fs, iter, path::Path, process::Command};

#[track_caller]
fn check(filename: &str) {
    // Normalize actual file
    let actual = Path::new(env!("CARGO_TARGET_TMPDIR")).join(filename);
    let actual_contents = fs::read_to_string(actual).unwrap();
    let actual_contents = if filename.ends_with("ll") {
        actual_contents
            .lines()
            // Skip header
            .skip(5)
            // Skip tags, we're only really interested in the function bodies,
            // and these tend to change often.
            .take_while(|line| !line.contains("attributes #0"))
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        // Skip often-changing assembly directives.
        actual_contents
            .lines()
            .filter(|line| !line.contains(".build_version"))
            .chain(iter::once(""))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let expected = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("memory")
        .join(filename);
    let expected_contents = fs::read_to_string(&expected).unwrap();

    // Overwrite test data with:
    // TEST_OVERWRITE=1 cargo test -pobjc2-browser-engine-core --test memory
    let should_overwrite = env::var("TEST_OVERWRITE")
        .map(|var| var == "1")
        .unwrap_or(false);
    if should_overwrite {
        fs::write(expected, actual_contents).unwrap();
    } else if expected_contents != actual_contents {
        eprintln!("\n===Expected===\n{expected_contents}\n===Actual===\n{actual_contents}");
        panic!("expected and actual did not match.");
    }
}

#[test]
#[cfg_attr(
    not(all(target_os = "macos", target_arch = "aarch64")),
    ignore = "test designed for host macOS Aarch64"
)]
fn test_llvm_and_assembly() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests_dir = manifest_dir.join("tests").join("memory");
    let tmp_dir = Path::new(env!("CARGO_TARGET_TMPDIR"));
    let rustc = env::var("RUSTC");
    let rustc = rustc.as_deref().unwrap_or("rustc");
    let clang = env::var("CC");
    let clang = clang.as_deref().unwrap_or("clang");

    // Compile library into libobjc2_browser_engine_core.rlib.
    let res = Command::new(rustc)
        .env("MACOSX_DEPLOYMENT_TARGET", "14.3")
        .current_dir(&tests_dir)
        .arg("--edition=2021")
        .arg("--crate-type=lib")
        .arg("--crate-name=objc2_browser_engine_core")
        .arg(r#"--cfg=feature="BEMemory""#)
        .arg(manifest_dir.join("src").join("lib.rs"))
        .arg("-o")
        .arg(tmp_dir.join("libobjc2_browser_engine_core.rlib"))
        .status()
        .unwrap();
    assert!(res.success());

    // Compile memory/use_functions.rs to LLVM IR.
    let res = Command::new(rustc)
        .env("MACOSX_DEPLOYMENT_TARGET", "14.3")
        .current_dir(&tests_dir)
        .arg("--edition=2021")
        .arg("--crate-type=lib")
        .arg("-L")
        .arg(tmp_dir)
        .arg("--extern=objc2_browser_engine_core")
        .arg("--emit=llvm-ir")
        .arg("use_rs.rs")
        .arg("-o")
        .arg(tmp_dir.join("use_rs.ll"))
        .status()
        .unwrap();
    assert!(res.success());
    check("use_rs.ll");

    // Compile memory/use_functions.rs to ASM.
    let res = Command::new(rustc)
        .env("MACOSX_DEPLOYMENT_TARGET", "14.3")
        .current_dir(&tests_dir)
        .arg("--edition=2021")
        .arg("--crate-type=lib")
        .arg("-L")
        .arg(tmp_dir)
        .arg("--extern=objc2_browser_engine_core")
        .arg("--emit=asm")
        .arg("use_rs.rs")
        .arg("-o")
        .arg(tmp_dir.join("use_rs.s"))
        .status()
        .unwrap();
    assert!(res.success());
    check("use_rs.s");

    // Compile memory/use_functions.c to LLVM IR.
    let res = Command::new(clang)
        .env("MACOSX_DEPLOYMENT_TARGET", "14.3")
        .current_dir(&tests_dir)
        .arg("-emit-llvm")
        .arg("-S")
        .arg("use_c.c")
        .arg("-o")
        .arg(tmp_dir.join("use_c.ll"))
        .status()
        .unwrap();
    assert!(res.success());
    check("use_c.ll");

    // Compile memory/use_functions.c to ASM.
    let res = Command::new(clang)
        .env("MACOSX_DEPLOYMENT_TARGET", "14.3")
        .current_dir(&tests_dir)
        .arg("-S")
        .arg("use_c.c")
        .arg("-o")
        .arg(tmp_dir.join("use_c.s"))
        .status()
        .unwrap();
    assert!(res.success());
    check("use_c.s");
}

#[test]
#[cfg(all(target_os = "ios", target_arch = "aarch64"))]
fn run() {
    assert!(objc2_browser_engine_core::be_memory_inline_jit_restrict_with_witness_supported());
    objc2_browser_engine_core::be_memory_inline_jit_restrict_rwx_to_rw_with_witness();
    objc2_browser_engine_core::be_memory_inline_jit_restrict_rwx_to_rx_with_witness();
}
