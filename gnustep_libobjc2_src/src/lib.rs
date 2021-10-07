use std::env;
use std::path::{Path, PathBuf};

const NO_SOURCES_MSG: &'static str = r#"
libobjc2 sources not present, please run:

$ git submodule update --init --recursive

If you did not checkout via git, you will
need to fetch the submodule's contents from
https://github.com/gnustep/libobjc2
"#;

pub fn build() -> PathBuf {
    // GNUStep only compiles with clang, so try that first.
    // (But let the user specify a different path if they need to).
    if env::var_os("CC").is_none() {
        env::set_var("CC", "clang");
    }
    if env::var_os("CXX").is_none() {
        env::set_var("CXX", "clang++");
    }

    let source_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("libobjc2");
    if !source_dir.join("objc/objc.h").exists() {
        panic!("{}", NO_SOURCES_MSG);
    }

    cmake::Config::new(source_dir)
        .define("BUILD_STATIC_LIBOBJC", "OFF") // Default
        .define("DEBUG_ARC_COMPAT", "OFF") // Default
        .define("ENABLE_OBJCXX", "ON") // Default (NO_OBJCXX in code)
        .define("ENABLE_TRACING", "OFF") // Default (WITH_TRACING in code)
        .define("LEGACY_COMPAT", "OFF") // Default (NO_LEGACY in code)
        // .define("OLDABI_COMPAT", "?") // Default depends on WIN32
        .define("TESTS", "OFF")
        .define("TYPE_DEPENDENT_DISPATCH", "ON") // Default
        // .always_configure(false) // TODO
        // .static_crt(?)
        .build_target("install")
        .build()
}
