//! A helper script for testing the assembly output.
//!
//! Similar to `trybuild` and `compiletest`, except specialized to our setup!
//!
//! Use as:
//! ```
//! TEST_OVERWRITE=1 cargo run --features=run --bin test-assembly -- --target=x86_64-apple-darwin
//! ```
//!
//! Very limited currently, for example we can't stably test things that emits
//! mangled symbols, nor things that are emitted in different crates.

use std::env;
use std::env::args;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

use test_assembly::{get_artifact, read_assembly};

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let should_overwrite = option_env!("TEST_OVERWRITE").is_some();
    let host = env!("TARGET");

    println!("Host {host}");

    for entry in manifest_dir.join("crates").read_dir().unwrap() {
        let entry = entry.unwrap();
        if !entry.file_type().unwrap().is_dir() {
            continue;
        }
        let package_path = entry.path();
        let package = package_path.file_name().unwrap().to_str().unwrap();

        println!("Testing {package}.");

        let result = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
            // .arg("+nightly")
            // .arg("-Zbuild-std")
            // .arg("-vv")
            .arg("rustc")
            .arg(format!("--package={package}"))
            .args(args().skip(1))
            .arg("--release")
            .arg("--message-format=json-render-diagnostics")
            .arg("--features=assembly-features")
            .arg("--")
            .arg("--emit=asm")
            // .arg("-Zplt=no")
            .arg("-Cllvm-args=--x86-asm-syntax=intel")
            .arg("-Csymbol-mangling-version=v0")
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .output()
            .unwrap();

        let artifact = get_artifact(&result.stdout, package);

        println!("{}", artifact.display());

        // Very brittle!
        let target = artifact
            .components()
            .map(|component| component.as_os_str().to_str().unwrap())
            .skip_while(|&component| component != "target")
            .nth(1)
            .unwrap_or(host);

        println!("Target {target}.");
        let mut architecture = target.split_once('-').unwrap().0;
        if matches!(architecture, "i386" | "i686") {
            architecture = "x86";
        };
        if target == "i686-apple-darwin" {
            // Old ABI, we frequently have to do things differently there
            architecture = "old-x86";
        }
        println!("Architecture {architecture}.");

        let extension = artifact.extension().unwrap().to_str().unwrap();

        let expected_file = package_path
            .join("expected")
            .join(format!("apple-{architecture}.{extension}"));

        let actual = read_assembly(&artifact).unwrap();
        if should_overwrite {
            fs::write(expected_file, actual).unwrap();
        } else if let Ok(expected) = fs::read_to_string(expected_file) {
            if expected != actual {
                eprintln!("\n===Expected===\n{}\n===Actual===\n{}", expected, actual);
                panic!("Expected and actual did not match.");
            }
        } else {
            panic!(
                "Missing assembly output for architecture {}:\n{}",
                architecture, actual
            );
        }
    }
}
