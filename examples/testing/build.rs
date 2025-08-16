//! Allow linking to XCTest in the fashion that it expects.
use std::env;
use std::process::Command;

fn xcode_select_developer_dir() -> Option<String> {
    let output = Command::new("xcode-select")
        .arg("--print-path")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let mut stdout = output.stdout;
    if let Some(b'\n') = stdout.last() {
        let _ = stdout.pop().unwrap();
    }
    Some(String::from_utf8(stdout).unwrap())
}

fn main() {
    // The script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");

    let developer_dir = xcode_select_developer_dir();
    let developer_dir = developer_dir
        .as_deref()
        .unwrap_or("/Applications/Xcode.app/Contents/Developer");

    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let abi = env::var("CARGO_CFG_TARGET_ABI");
    let abi = abi.as_deref().unwrap_or("");

    let sdk_name = match (&*os, abi) {
        ("macos", "") => "MacOSX",
        ("ios", "") => "iPhoneOS",
        ("ios", "sim") => "iPhoneSimulator",
        // Mac Catalyst uses the macOS SDK
        ("ios", "macabi") => "MacOSX",
        ("tvos", "") => "AppleTVOS",
        ("tvos", "sim") => "AppleTVSimulator",
        ("visionos", "") => "XROS",
        ("visionos", "sim") => "XRSimulator",
        ("watchos", "") => "WatchOS",
        ("watchos", "sim") => "WatchSimulator",
        (os, abi) => unreachable!("invalid os '{os}' / abi '{abi}' combination for Apple target"),
    };

    // XCTest and XCUIAutomation live inside:
    // `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/Library/Frameworks`
    println!("cargo:rustc-link-search=framework={developer_dir}/Platforms/{sdk_name}.platform/Developer/Library/Frameworks");
    println!("cargo:rustc-link-search=native={developer_dir}/Platforms/{sdk_name}.platform/Developer/usr/lib");

    // Configure the test binary as a bundle instead.
    println!("cargo:rustc-link-arg-bins=-bundle");
    // + #![no_main] and #[ctor]
    // + maybe -bundle_loader app_under_test?

    // TODO: Are these necessary?
    println!("cargo:rustc-link-arg-bins=-Xlinker");
    println!("cargo:rustc-link-arg-bins=-debug_variant");
    println!("cargo:rustc-link-arg-bins=-Xlinker");
    println!("cargo:rustc-link-arg-bins=-export_dynamic");
    println!("cargo:rustc-link-arg-bins=-Xlinker");
    println!("cargo:rustc-link-arg-bins=-no_deduplicate");
}
