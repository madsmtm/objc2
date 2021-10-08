#[cfg(feature = "vendor_gnustep")]
fn main() {
    let artifacts = gnustep_libobjc2_src::build();
    artifacts.print_cargo_metadata();
    println!("cargo:rustc-cfg=gnustep");
}

#[cfg(not(feature = "vendor_gnustep"))]
fn main() {
    // Only rerun if this file changes; the script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");

    let target_vendor = std::env::var("CARGO_CFG_TARGET_VENDOR").unwrap();

    if target_vendor == "apple" {
        // Add #[cfg(apple)] directive
        println!("cargo:rustc-cfg=apple");
    } else {
        // TODO: Are there other possibilities than GNUStep? Airyx? Is it
        // possible to use Apple's open source objc4 on other platforms?

        // Add #[cfg(gnustep)] directive
        println!("cargo:rustc-cfg=gnustep");
    }

    // Link to libobjc
    println!("cargo:rustc-link-lib=dylib=objc");
}
