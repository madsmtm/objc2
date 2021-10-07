use std::env;
use std::path::{Path, PathBuf};

const NO_SOURCES_MSG: &'static str = r#"
libobjc2 sources not present, please run:

$ git submodule update --init --recursive

If you did not checkout via git, you will
need to fetch the submodule's contents from
https://github.com/gnustep/libobjc2
"#;

pub struct Builder {
    lib_kind: LibKind,
    objcxx: bool,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            lib_kind: LibKind::Dynamic,
            objcxx: true,
        }
    }

    /// Set the type of library to be built, and how linking is performed.
    ///
    /// Possible options are [`LibKind::Static`] and [`LibKind::Dynamic`].
    ///
    /// Defaults to [`LibKind::Dynamic`].
    pub fn lib_kind(&mut self, kind: LibKind) -> &mut Self {
        self.lib_kind = kind;
        self
    }

    /// Enable support for Objective-C++.
    ///
    /// Namely interoperability between Objective-C and C++ exceptions.
    ///
    /// Defaults to [`true`].
    pub fn objcxx(&mut self, objcxx: bool) -> &mut Self {
        self.objcxx = objcxx;
        self
    }

    pub fn build(&mut self) -> Artifacts {
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

        let dst = cmake::Config::new(&source_dir)
            // Default to ignoring `gnustep-config` presence, since they
            // usually want to install the libraries globally (which requires
            // root). Users that want systemwide installation should just
            // install it themselves, and shouldn't need to vendor GNUStep.
            .define("GNUSTEP_INSTALL_TYPE", "NONE")
            // Don't bother building tests, we're not gonna run them anyways
            // (and they're not available when packaged, see Cargo.toml).
            .define("TESTS", "OFF")
            // Having this on also builds the dynamic library, but not really
            // anything we can do to change that.
            .define(
                "BUILD_STATIC_LIBOBJC",
                match self.lib_kind {
                    LibKind::Static => "ON",
                    LibKind::Dynamic => "OFF",
                },
            )
            .define("ENABLE_OBJCXX", if self.objcxx { "ON" } else { "OFF" })
            // Various other defaults
            // .define("OLDABI_COMPAT", "ON")
            // .define("DEBUG_ARC_COMPAT", "OFF")
            // .define("ENABLE_TRACING", "OFF")
            // .define("LEGACY_COMPAT", "OFF")
            // .define("LIBOBJC_NAME", "objc")
            // .define("TYPE_DEPENDENT_DISPATCH", "ON")
            // .define("STRICT_APPLE_COMPATIBILITY", "0") // Default none
            // TODO: .static_crt(?)
            .build_target("install")
            .build();

        Artifacts {
            source_dir,
            include_dir: dst.join("include"),
            lib_dir: dst.join("lib"),
            lib_kind: self.lib_kind,
            lib_name: "objc",
        }
    }
}

pub struct Artifacts {
    source_dir: PathBuf,
    include_dir: PathBuf,
    lib_dir: PathBuf,
    lib_kind: LibKind,
    lib_name: &'static str,
}

impl Artifacts {
    pub fn source_dir(&self) -> &Path {
        &self.source_dir
    }

    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    pub fn lib_kind(&self) -> LibKind {
        self.lib_kind
    }

    pub fn lib_name(&self) -> &str {
        &self.lib_name
    }

    pub fn print_cargo_metadata(&self) {
        let kind = match self.lib_kind {
            LibKind::Dynamic => "dylib",
            LibKind::Static => "static",
        };
        println!("cargo:rustc-link-search=native={}", self.lib_dir.display());
        println!("cargo:rustc-link-lib={}={}", kind, self.lib_name);
        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:lib={}", self.lib_dir.display());
    }

    pub fn print_cargo_rerun_if_changed(&self) {
        println!("cargo:rerun-if-env-changed=CC");
        println!("cargo:rerun-if-env-changed=CXX");
        rerun_if(&self.source_dir);
    }
}

fn rerun_if(path: &Path) {
    if path.is_dir() {
        for entry in std::fs::read_dir(path).expect("read_dir") {
            rerun_if(&entry.expect("entry").path());
        }
    } else {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum LibKind {
    Dynamic,
    Static,
    // Framework,
}
