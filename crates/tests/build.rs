use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let clang_objc_runtime = if env::var_os("CARGO_FEATURE_GNUSTEP_2_1").is_some() {
        Some("-fobjc-runtime=gnustep-2.1")
    } else if env::var_os("CARGO_FEATURE_GNUSTEP_2_0").is_some() {
        Some("-fobjc-runtime=gnustep-2.0")
    } else if env::var_os("CARGO_FEATURE_GNUSTEP_1_9").is_some() {
        Some("-fobjc-runtime=gnustep-1.9")
    } else if env::var_os("CARGO_FEATURE_GNUSTEP_1_8").is_some() {
        Some("-fobjc-runtime=gnustep-1.8")
    } else if env::var_os("CARGO_FEATURE_GNUSTEP_1_7").is_some() {
        Some("-fobjc-runtime=gnustep-1.7")
    } else if env::var_os("CARGO_FEATURE_UNSTABLE_OBJFW").is_some() {
        Some("-fobjc-runtime=objfw-0.8")
    } else if env::var("CARGO_CFG_TARGET_VENDOR").unwrap() == "apple" {
        // Default to `clang`'s own heuristics.
        //
        // Note that the `cc` crate forwards the correct deployment target to clang as well.
        None
    } else {
        panic!("Must specify the desired runtime using Cargo features on non-Apple platforms")
    };

    let mut builder = cc::Build::new();
    builder.compiler("clang");
    builder.file("extern/block_utils.c");
    println!("cargo:rerun-if-changed=extern/block_utils.c");

    builder.flag("-fobjc-exceptions");

    if let Some(clang_objc_runtime) = clang_objc_runtime {
        builder.flag(clang_objc_runtime);
    }

    if cfg!(feature = "gnustep-1-7") && !cfg!(feature = "gnustep-2-0") {
        builder.include("compat-headers/gnustep-pre-2-0");
    }

    if cfg!(feature = "unstable-objfw") {
        builder.include("compat-headers/objfw");
    }

    builder.flag("-fblocks");
    builder.flag("-fno-objc-arc");

    builder.flag("-xobjective-c");

    builder.compile("libblock_utils.a");

    let mut builder = cc::Build::new();
    builder.compiler("clang");
    builder.file("extern/encode_utils.m");
    builder.file("extern/test_object.m");
    #[cfg(feature = "unstable-simd")]
    builder.file("extern/test_simd_return.m");
    println!("cargo:rerun-if-changed=extern/encode_utils.m");
    println!("cargo:rerun-if-changed=extern/test_object.m");
    println!("cargo:rerun-if-changed=extern/test_simd_return.m");

    builder.flag("-fblocks");

    if let Some(clang_objc_runtime) = clang_objc_runtime {
        builder.flag(clang_objc_runtime);
    }

    builder.flag("-fobjc-exceptions");
    builder.flag("-fobjc-arc");
    builder.flag("-fobjc-arc-exceptions");

    builder.compile("libobjc_utils.a");
}
