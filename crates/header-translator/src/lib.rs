#![recursion_limit = "256"]

#[macro_use]
extern crate tracing;

use std::fmt;
use std::path::Path;
use std::process::{Command, Stdio};

use clang::{Entity, EntityVisitResult};
use tracing::span::EnteredSpan;

mod availability;
mod cache;
mod config;
mod context;
mod data;
mod display_helper;
mod expr;
mod feature;
mod file;
mod id;
mod library;
mod method;
mod objc2_utils;
mod output;
mod rust_type;
mod stmt;
mod unexposed_attr;

pub use self::cache::Cache;
pub use self::config::Config;
pub use self::context::Context;
pub use self::file::File;
pub use self::id::ItemIdentifier;
pub use self::library::Library;
pub use self::output::Output;
pub use self::stmt::{Mutability, Stmt};

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

pub fn run_rustfmt(data: impl fmt::Display) -> Vec<u8> {
    use std::io::Write;

    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed running rustfmt");

    let mut stdin = child.stdin.take().expect("failed to open stdin");
    write!(stdin, "{data}").expect("failed writing");
    drop(stdin);

    let output = child.wait_with_output().expect("failed formatting");

    if !output.status.success() {
        panic!("failed running rustfmt with exit code {}", output.status)
    }

    output.stdout
}

fn immediate_children<'tu>(
    entity: &Entity<'tu>,
    mut closure: impl FnMut(Entity<'tu>, EnteredSpan),
) {
    entity.visit_children(|entity, _parent| {
        let span = debug_span!(
            "child",
            kind = ?entity.get_kind(),
            dbg = entity.get_name(),
        )
        .entered();

        closure(entity, span);

        EntityVisitResult::Continue
    });
}

pub(crate) fn to_snake_case(input: impl AsRef<str>) -> String {
    let input = input.as_ref();
    if input == "_" {
        String::from("_")
    } else {
        heck::ToSnakeCase::to_snake_case(input)
    }
}
