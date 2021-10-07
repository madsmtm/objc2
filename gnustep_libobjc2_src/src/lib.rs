use std::env;
use std::path::{Path, PathBuf};

const NO_SOURCES_MSG: &'static str = r#"
libobjc2 sources not present, please run:

$ git submodule update --init --recursive

If you did not checkout via git, you will
need to fetch the submodule's contents from
https://github.com/gnustep/libobjc2
"#;

#[non_exhaustive]
pub enum LibKind {
    Dynamic,
    Static,
    // Framework,
}

pub struct Artifacts {
    include_dir: PathBuf,
    lib_dir: PathBuf,
    lib_kind: LibKind,
    lib_name: &'static str,
}

pub fn build() -> Artifacts {
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

    let dst = cmake::Config::new(source_dir)
        // Default to ignoring `gnustep-config` presence, since they usually
        // want to install the libraries globally (which requires root).
        // Users that want systemwide installation should just install it
        // themselves, and shouldn't need to vendor GNUStep.
        .define("GNUSTEP_INSTALL_TYPE", "NONE")
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
        .build();

    Artifacts {
        include_dir: dst.join("include"),
        lib_dir: dst.join("lib"),
        lib_kind: LibKind::Dynamic,
        lib_name: "objc",
    }
}

impl Artifacts {
    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    pub fn lib_kind(&self) -> &LibKind {
        &self.lib_kind
    }

    pub fn lib_name(&self) -> &str {
        &self.lib_name
    }

    pub fn print_cargo_metadata(&self) {
        let kind = match self.lib_kind {
            LibKind::Dynamic => "dynamic",
            LibKind::Static => "static",
        };
        println!("cargo:rustc-link-search=native={}", self.lib_dir.display());
        println!("cargo:rustc-link-lib={}={}", kind, self.lib_name);
        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:lib={}", self.lib_dir.display());
    }
}
