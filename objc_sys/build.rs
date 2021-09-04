fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=dylib=objc");

    // TODO: Should we vendor GNUStep's libobjc2 on non-apple targets?
    // Check std::env::env::var("CARGO_CFG_TARGET_VENDOR").unwrap()
    // Cargo.toml:
    // [target.'cfg(not(target_vendor = "apple"))'.build-dependencies]
    // cc = "1.0"
}
