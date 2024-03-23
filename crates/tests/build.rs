use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut builder = cc::Build::new();
    builder.compiler("clang");
    builder.file("extern/block_utils.c");
    println!("cargo:rerun-if-changed=extern/block_utils.c");

    for flag in env::var("DEP_OBJC_0_3_CC_ARGS").unwrap().split(' ') {
        builder.flag(flag);
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
    builder.file("extern/test_simd_return.m");
    println!("cargo:rerun-if-changed=extern/encode_utils.m");
    println!("cargo:rerun-if-changed=extern/test_object.m");
    println!("cargo:rerun-if-changed=extern/test_simd_return.m");

    builder.flag("-fblocks");

    for flag in env::var("DEP_OBJC_0_3_CC_ARGS").unwrap().split(' ') {
        builder.flag(flag);
    }

    builder.compile("libobjc_utils.a");
}
