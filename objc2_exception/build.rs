use std::env;

fn main() {
    println!("cargo:rerun-if-changed=extern/exception.m");

    let mut builder = cc::Build::new();
    builder.file("extern/exception.m");

    for flag in env::var("DEP_OBJC_CC_ARGS").unwrap().split(' ') {
        builder.flag(flag);
    }

    builder.compile("librust_objc_try_catch_exception.a");
}
