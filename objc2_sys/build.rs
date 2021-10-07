#[cfg(feature = "vendor_gnustep")]
fn main() {
    let mut builder = gnustep_libobjc2_src::Builder::new();

    #[cfg(feature = "static")]
    builder.lib_kind(gnustep_libobjc2_src::LibKind::Static);
    #[cfg(not(feature = "static"))]
    builder.lib_kind(gnustep_libobjc2_src::LibKind::Dynamic);

    let artifacts = builder.build();
    artifacts.print_cargo_metadata();
    artifacts.print_cargo_rerun_if_changed();

    // Add #[cfg(gnustep)] directive
    println!("cargo:rustc-cfg=gnustep");
}

#[cfg(not(feature = "vendor_gnustep"))]
fn main() {
    #[cfg(feature = "static")]
    compile_error!("Can only link statically to libobjc when vendoring is enabled.");

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
