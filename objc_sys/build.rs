use std::env;

fn main() {
    // Only rerun if this file changes; the script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");
    // Link to libobjc
    println!("cargo:rustc-link-lib=dylib=objc");

    let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap();

    // Adds useful #[cfg(apple)] and #[cfg(gnustep)] directives
    if target_vendor == "apple" {
        println!("cargo:rustc-cfg=apple");
    } else {
        // TODO: Are there other possibilities than GNUStep? Airyx? Is it
        // possible to use Apple's open source objc4 on other platforms?
        println!("cargo:rustc-cfg=gnustep");
        // TODO: Should we vendor GNUStep's libobjc2?
        // Using Cargo.toml:
        // [target.'cfg(not(target_vendor = "apple"))'.build-dependencies]
        // cc = "1.0"
    }
}
