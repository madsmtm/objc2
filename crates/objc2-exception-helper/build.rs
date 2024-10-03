use std::env;

fn main() {
    // The script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");

    if std::env::var("DOCS_RS").is_ok() {
        // docs.rs doesn't have clang, so skip building this. The
        // documentation will still work since it doesn't need to link.
        //
        // This is independent of the `docsrs` cfg; we never want to try
        // invoking clang on docs.rs, whether we're the crate being
        // documented currently, or a dependency of another crate.
        return;
    }

    // Note: Intentionally compile without ARC, we want to handle memory
    // management ourselves.
    let mut builder = cc::Build::new();
    builder.flag("-xobjective-c");
    builder.flag("-fobjc-exceptions");
    builder.file("src/try_catch.m");

    // Set Objective-C runtime. We assume the compiler is Clang, if it isn't,
    // this is probably going to fail anyways, since we're using newer
    // runtimes than GCC supports.
    //
    // TODO: ObjFW via `-fobjc-runtime=objfw-VERSION`. Clang defaults to 0.8
    if env::var_os("CARGO_FEATURE_GNUSTEP_2_1").is_some() {
        builder.flag("-fobjc-runtime=gnustep-2.1");
    } else if env::var_os("CARGO_FEATURE_GNUSTEP_2_0").is_some() {
        builder.flag("-fobjc-runtime=gnustep-2.0");
    } else if env::var_os("CARGO_FEATURE_GNUSTEP_1_9").is_some() {
        builder.flag("-fobjc-runtime=gnustep-1.9");
    } else if env::var_os("CARGO_FEATURE_GNUSTEP_1_8").is_some() {
        builder.flag("-fobjc-runtime=gnustep-1.8");
    } else if env::var_os("CARGO_FEATURE_GNUSTEP_1_7").is_some() {
        builder.flag("-fobjc-runtime=gnustep-1.7");
    } else {
        // Let the compiler choose a sensible default
    };

    println!("cargo:rerun-if-changed=src/try_catch.m");
    builder.compile("libobjc2_exception_helper_0_1.a");
}
