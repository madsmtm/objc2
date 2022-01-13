use std::env;

fn main() {
    // The script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");

    let runtime = env::var("DEP_OBJC_RUNTIME").unwrap();
    println!("cargo:rustc-cfg={}", runtime);

    #[cfg(feature = "exception")]
    {
        if std::env::var("DOCS_RS").is_ok() {
            // docs.rs doesn't have clang, so skip building this. The
            // documentation will still work since it doesn't need to link.
            return;
        }
        println!("cargo:rerun-if-changed=extern/exception.m");

        let mut builder = cc::Build::new();
        builder.file("extern/exception.m");

        for flag in env::var("DEP_OBJC_CC_ARGS").unwrap().split(' ') {
            builder.flag(flag);
        }

        builder.compile("librust_objc_try_catch_exception.a");
    }

    // For compile tests
    println!(
        "cargo:rustc-env=BUILD_PROFILE={}",
        env::var("PROFILE").unwrap()
    );
    println!(
        "cargo:rustc-env=BUILD_TARGET={}",
        env::var("TARGET").unwrap()
    );
}
