use std::collections::HashSet;
use std::fmt::{Display, Write};
use std::path::Path;
use std::process::{Command, Stdio};

mod availability;
mod config;
mod expr;
mod method;
mod objc2_utils;
mod property;
mod rust_type;
mod stmt;
mod unexposed_macro;

pub use self::config::Config;
pub use self::stmt::Stmt;

#[derive(Debug)]
pub struct RustFile {
    declared_types: HashSet<String>,
    stmts: Vec<Stmt>,
}

const INITIAL_IMPORTS: &str = r#"//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
"#;

impl RustFile {
    pub fn new() -> Self {
        Self {
            declared_types: HashSet::new(),
            stmts: Vec::new(),
        }
    }

    pub fn add_stmt(&mut self, stmt: Stmt) {
        match &stmt {
            Stmt::ClassDecl { name, .. } => {
                self.declared_types.insert(name.clone());
            }
            Stmt::CategoryDecl { .. } => {}
            Stmt::ProtocolDecl { name, .. } => {
                self.declared_types.insert(name.clone());
            }
            Stmt::EnumDecl { name, variants, .. } => {
                // Fix weirdness with enums, they're found twice for some reason
                if let Some(Stmt::EnumDecl {
                    name: last_name, ..
                }) = self.stmts.last()
                {
                    if last_name == name {
                        self.stmts.pop();
                    }
                }

                if let Some(name) = name {
                    self.declared_types.insert(name.clone());
                }
                for (name, _) in variants {
                    self.declared_types.insert(name.clone());
                }
            }
            Stmt::AliasDecl { name, .. } => {
                self.declared_types.insert(name.clone());
            }
        }
        self.stmts.push(stmt);
    }

    pub fn finish(self) -> (HashSet<String>, String) {
        let mut tokens = String::new();
        writeln!(tokens, "{}", INITIAL_IMPORTS).unwrap();
        for stmt in self.stmts {
            writeln!(tokens, "{}", stmt).unwrap();
        }

        (self.declared_types, tokens)
    }
}

pub fn run_cargo_fmt(package: &str) {
    let status = Command::new("cargo")
        .args(["fmt", "--package", package])
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap())
        .status()
        .expect("failed running cargo fmt");

    assert!(
        status.success(),
        "failed running cargo fmt with exit code {status}"
    );
}

pub fn run_rustfmt(data: impl Display) -> Vec<u8> {
    use std::io::Write;

    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed running rustfmt");

    let mut stdin = child.stdin.take().expect("failed to open stdin");
    write!(stdin, "{}", data).expect("failed writing");
    drop(stdin);

    let output = child.wait_with_output().expect("failed formatting");

    if !output.status.success() {
        panic!("failed running rustfmt with exit code {}", output.status)
    }

    output.stdout
}
