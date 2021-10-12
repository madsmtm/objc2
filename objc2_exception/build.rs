use std::env;

fn main() {
    println!("cargo:rerun-if-changed=extern/exception.m");

    let mut builder = cc::Build::new();
    builder.file("extern/exception.m");

    // Assume the C compiler is clang; if it isn't, this is probably going to
    // fail anyways.
    for flag in env::var("DEP_OBJC_CLANG_ARGS").unwrap().split(' ') {
        builder.flag(flag);
    }

    builder.compile("librust_objc_try_catch_exception.a");
}
