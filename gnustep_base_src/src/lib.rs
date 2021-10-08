use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const NO_SOURCES_MSG: &'static str = r#"
GNUStep Base sources not present, please run:

$ git submodule update --init --recursive

If you did not checkout via git, you will
need to fetch the submodule's contents from
https://github.com/gnustep/libs-base
"#;

pub struct Builder {}

impl Builder {
    pub fn new() -> Self {
        Self {}
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

        let source_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("libs-base");
        if !source_dir.join("Headers/Foundation/Foundation.h").exists() {
            panic!("{}", NO_SOURCES_MSG);
        }

        let mut configure = Command::new(source_dir.join("configure"));
        // .arg("--with-config-file=")
        // .arg("--enable-libffi")
        // .arg("--with-default-config=")
        // .arg("--prefix") // Or GNUSTEP_SYSTEM_ROOT ?

        // run `make` and `make install`

        let dst: PathBuf = todo!();

        Artifacts {
            source_dir,
            include_dir: dst.join("include"),
            lib_dir: dst.join("lib"),
            lib_name: "gnustep-base",
        }
    }
}

pub struct Artifacts {
    source_dir: PathBuf,
    include_dir: PathBuf,
    lib_dir: PathBuf,
    lib_name: &'static str,
}

impl Artifacts {
    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    pub fn lib_name(&self) -> &str {
        &self.lib_name
    }

    pub fn print_cargo_metadata(&self) {
        println!("cargo:rustc-link-search=native={}", self.lib_dir.display());
        println!("cargo:rustc-link-lib=dylib={}", self.lib_name);
        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:lib={}", self.lib_dir.display());
    }

    pub fn print_cargo_rerun_if_changed(&self) {
        println!("cargo:rerun-if-env-changed=CC");
        println!("cargo:rerun-if-env-changed=CXX");
        rerun_if(&self.source_dir);
    }
}

// https://github.com/rust-lang/git2-rs/blob/0.13.23/libgit2-sys/build.rs#L228-L236
fn rerun_if(path: &Path) {
    if path.is_dir() {
        for entry in std::fs::read_dir(path).expect("read_dir") {
            rerun_if(&entry.expect("entry").path());
        }
    } else {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}
