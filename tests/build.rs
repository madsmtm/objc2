use std::env;

fn main() {
    println!("cargo:rerun-if-changed=extern/block_utils.c");

    let mut builder = cc::Build::new();
    builder.compiler("clang");
    builder.file("extern/block_utils.c");

    for flag in env::var("DEP_BLOCK_CC_ARGS").unwrap().split(' ') {
        builder.flag(flag);
    }

    builder.compile("libblock_utils.a");
}
