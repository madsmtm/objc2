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
            use objc2::{ClassType, extern_class, extern_methods};
            #[allow(unused_imports)]
            use objc2::rc::{Id, Shared};

            #(#iter)*
        };

        (self.declared_types, tokens)
    }
}

pub fn format_method_macro(code: &[u8]) -> Vec<u8> {
    use regex::bytes::{Captures, Regex};
    use std::str;

    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"# ?\[ ?(method_id|method) ?\((([a-zA-Z_]+ ?: ?)+)\) ?\]").unwrap();
    }

    RE.replace_all(code, |caps: &Captures| {
        let method = str::from_utf8(caps.get(1).unwrap().as_bytes())
            .unwrap()
            .replace(" ", "");
        let selector = str::from_utf8(caps.get(2).unwrap().as_bytes())
            .unwrap()
            .replace(" ", "");
        format!("#[{method}({selector})]")
    })
    .to_vec()
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

    format_method_macro(&output.stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_method_macro() {
        fn assert_returns(input: &str, expected_output: &str) {
            let output = format_method_macro(input.as_bytes());
            assert_eq!(output, expected_output.as_bytes());

            // Check that running it through twice doesn't change the output
            let output = format_method_macro(&output);
            assert_eq!(output, expected_output.as_bytes());
        }

        assert_returns(
            "# [method_id (descriptorWithDescriptorType : bytes : length :)]",
            "#[method_id(descriptorWithDescriptorType:bytes:length:)]",
        );
        assert_returns(
            "# [method (insertDescriptor : atIndex :)]",
            "#[method(insertDescriptor:atIndex:)]",
        );
        assert_returns(
            "# [method_id (descriptorAtIndex :)]",
            "#[method_id(descriptorAtIndex:)]",
        );
    }
}
