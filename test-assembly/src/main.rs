//! A helper script for testing the assembly output.
//!
//! Similar to `trybuild` and `compiletest`, except specialized to our setup!
//!
//! Use as:
//! ```
//! TEST_OVERWRITE=1 cargo run --bin test-assembly --target=x86_64-apple-darwin
//! ```
//!
//! Very limited currently, for example we can't stably test things that emits
//! mangled symbols, nor things that are emitted in different crates.

use cargo_metadata::Message;
use std::env;
use std::env::args;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

fn strip_lines(data: &str, starts_with: &str) -> String {
    data.lines()
        .filter(|line| !line.trim_start().starts_with(starts_with))
        .collect::<Vec<_>>()
        .join("\n")
}

fn strip_section(data: &str, section: &str) -> String {
    let mut res = String::with_capacity(data.len());
    let mut in_removed_section = false;
    for line in data.lines() {
        // This only works for the __LLVM sections we're interested in
        if line.trim().starts_with(".section") {
            if line.contains(section) {
                in_removed_section = true;
                println!("Stripped {section} section");
            } else {
                in_removed_section = false;
            }
        }
        if !in_removed_section {
            res.push_str(line);
            res.push('\n');
        }
        if line == "" {
            in_removed_section = false;
        }
    }
    res
}

fn read_assembly<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let s = fs::read_to_string(path)?;
    let workspace_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap();
    let s = s.replace(workspace_dir, "$WORKSPACE");
    // HACK: Replace Objective-C image info for simulator targets
    let s = s.replace(
        ".asciz\t\"\\000\\000\\000\\000`\\000\\000\"",
        ".asciz\t\"\\000\\000\\000\\000@\\000\\000\"",
    );
    // Strip various uninteresting directives
    let s = strip_lines(&s, ".cfi_");
    let s = strip_lines(&s, ".macosx_version_");
    let s = strip_lines(&s, ".ios_version_");
    let s = strip_lines(&s, ".build_version");
    // We remove the __LLVM,__bitcode and __LLVM,__cmdline sections because
    // they're uninteresting for out use-case.
    //
    // See https://github.com/rust-lang/rust/blob/1.59.0/compiler/rustc_codegen_llvm/src/back/write.rs#L978-L1074
    Ok(strip_section(&s, "__LLVM"))
}

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let should_overwrite = option_env!("TEST_OVERWRITE").is_some();
    let host = env!("TARGET");

    for entry in manifest_dir.join("crates").read_dir().unwrap() {
        let entry = entry.unwrap();
        if !entry.file_type().unwrap().is_dir() {
            continue;
        }
        let package_path = entry.path();
        let package = package_path.file_name().unwrap().to_str().unwrap();

        println!("Testing {package}.");

        let result = Command::new(std::env::var("CARGO").unwrap_or("cargo".into()))
            // .arg("+nightly")
            // .arg("-Zbuild-std")
            // .arg("-vv")
            .arg("rustc")
            .arg(format!("--package={package}"))
            .args(args().skip(2))
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

        let artifact = Message::parse_stream(&*result.stdout)
            .find_map(|message| {
                if let Message::CompilerArtifact(artifact) = message.unwrap() {
                    // Brittle!
                    if artifact.target.name == package && artifact.filenames.len() == 2 {
                        let path = artifact.filenames[1].clone();
                        let stem = path.file_stem().unwrap().strip_prefix("lib").unwrap();
                        return Some(path.with_file_name(format!("{stem}.s")));
                    }
                }
                None
            })
            .unwrap_or_else(|| {
                panic!(
                    "Could not find package data:\n{}",
                    String::from_utf8_lossy(&result.stdout)
                )
            });

        // Very brittle!
        let target = artifact
            .components()
            .map(|component| component.as_str())
            .skip_while(|&component| component != "target")
            .skip(1)
            .next()
            .unwrap_or(host);

        println!("Target {target}.");
        let mut architecture = target.split_once("-").unwrap().0;
        if matches!(architecture, "i386" | "i686") {
            architecture = "x86";
        };
        if target == "i686-apple-darwin" {
            // Old ABI, we frequently have to do things differently there
            architecture = "old-x86";
        }
        println!("Architecture {architecture}.");

        let expected_file = package_path
            .join("expected")
            .join(format!("apple-{architecture}.s"));

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
