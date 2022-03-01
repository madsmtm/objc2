//! A helper script for testing the assembly output.
//!
//! Similar to `trybuild` and `compiletest`, except specialized to our setup!

use cargo_metadata::Message;
use std::env;
use std::env::args;
use std::fmt::Write;
use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

fn strip_section(data: &str, section: &str) -> String {
    let mut res = String::with_capacity(data.len());
    let mut in_removed_section = false;
    for line in data.lines() {
        // This only works for the __LLVM sections we're interested in
        if line == "" {
            in_removed_section = false;
        }
        if line.trim().starts_with(".section") {
            in_removed_section = line.contains(section);
        }
        if !in_removed_section {
            res.push_str(line);
        } else {
            write!(res, "; Stripped {section} line").unwrap();
        }
        res.push('\n');
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

    for entry in manifest_dir.join("assembly").read_dir().unwrap() {
        let package_path = entry.unwrap().path();
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
            .arg("--")
            .arg("--emit=asm")
            .arg("-Cllvm-args=--x86-asm-syntax=intel")
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

        let expected_file = package_path.join("expected").join(format!("{target}.s"));

        let actual = read_assembly(&artifact).unwrap();
        if should_overwrite {
            fs::write(expected_file, actual).unwrap();
        } else if let Ok(expected) = read_assembly(expected_file) {
            if expected != actual {
                eprintln!("\n===Expected===\n{}\n===Actual===\n{}", expected, actual);
                panic!("Expected and actual did not match.");
            }
        } else {
            panic!("Missing assembly output for target {}:\n{}", target, actual);
        }
    }
}
