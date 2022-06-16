use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut builder = cc::Build::new();
    builder.compiler("clang");
    builder.file("extern/block_utils.c");
    println!("cargo:rerun-if-changed=extern/block_utils.c");

    for flag in env::var("DEP_BLOCK_0_0_CC_ARGS").unwrap().split(' ') {
        builder.flag(flag);
    }

    for flag in env::var("DEP_OBJC_0_2_CC_ARGS").unwrap().split(' ') {
        builder.flag(flag);
    }

    builder.flag("-fno-objc-arc");

    builder.flag("-xobjective-c");

    builder.compile("libblock_utils.a");

    let mut builder = cc::Build::new();
    builder.compiler("clang");
    builder.file("extern/encode_utils.m");
    builder.file("extern/test_object.m");
    println!("cargo:rerun-if-changed=extern/encode_utils.m");
    println!("cargo:rerun-if-changed=extern/test_object.m");

    for flag in env::var("DEP_OBJC_0_2_CC_ARGS").unwrap().split(' ') {
        builder.flag(flag);
    }

    builder.compile("libobjc_utils.a");

    // For assembly tests
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());
}
