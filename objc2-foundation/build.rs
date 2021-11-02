use std::env;

fn main() {
    // The script doesn't depend on our code
    println!("cargo:rerun-if-changed=build.rs");

    let runtime = env::var("DEP_OBJC_RUNTIME").unwrap();
    println!("cargo:rustc-cfg={}", runtime);
}
