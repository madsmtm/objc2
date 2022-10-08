use std::collections::HashSet;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use proc_macro2::TokenStream;
use quote::quote;

mod availability;
mod config;
mod method;
mod objc2_utils;
mod property;
mod rust_type;
mod stmt;

pub use self::config::Config;
pub use self::stmt::Stmt;

#[derive(Debug)]
pub struct RustFile {
    declared_types: HashSet<String>,
    stmts: Vec<Stmt>,
}

impl RustFile {
    pub fn new() -> Self {
        Self {
            declared_types: HashSet::new(),
            stmts: Vec::new(),
        }
    }

    pub fn add_stmt(&mut self, stmt: Stmt) {
        match &stmt {
            Stmt::FileImport { .. } => {}
            Stmt::ItemImport { .. } => {}
            Stmt::ClassDecl { name, .. } => {
                self.declared_types.insert(name.clone());
            }
            Stmt::CategoryDecl { .. } => {}
            Stmt::ProtocolDecl { name, .. } => {
                self.declared_types.insert(name.clone());
            }
            Stmt::AliasDecl { name, .. } => {
                self.declared_types.insert(name.clone());
            }
        }
        self.stmts.push(stmt);
    }

    pub fn finish(self) -> (HashSet<String>, TokenStream) {
        let iter = self.stmts.iter().filter(|stmt| match stmt {
            Stmt::ItemImport { name } => !self.declared_types.contains(name),
            _ => true,
        });

        let tokens = quote! {
            #[allow(unused_imports)]
            use objc2::{ClassType, extern_class, extern_methods, msg_send, msg_send_id};
            #[allow(unused_imports)]
            use objc2::rc::{Id, Shared};

            #(#iter)*
        };

        (self.declared_types, tokens)
    }
}

pub fn run_cargo_fmt() {
    let status = Command::new("cargo")
        .args(["fmt", "--package=icrate"])
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap())
        .status()
        .expect("failed running cargo fmt");

    assert!(
        status.success(),
        "failed running cargo fmt with exit code {status}"
    );
}

pub fn run_rustfmt(tokens: TokenStream) -> Vec<u8> {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed running rustfmt");

    let mut stdin = child.stdin.take().expect("failed to open stdin");
    write!(stdin, "{}", tokens).expect("failed writing");
    drop(stdin);

    let output = child.wait_with_output().expect("failed formatting");

    if !output.status.success() {
        panic!("failed running rustfmt with exit code {}", output.status)
    }

    output.stdout
}
