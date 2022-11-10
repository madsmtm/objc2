use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());
}
