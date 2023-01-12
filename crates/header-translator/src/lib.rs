#![cfg(feature = "run")]
#![recursion_limit = "256"]

#[macro_use]
extern crate tracing;

use std::collections::BTreeMap;
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
mod expr;
mod file;
mod id;
mod library;
mod method;
mod objc2_utils;
mod output;
mod rust_type;
mod stmt;
mod unexposed_macro;

pub use self::cache::Cache;
pub use self::config::Config;
pub use self::context::Context;
pub use self::file::File;
pub use self::id::ItemIdentifier;
pub use self::library::Library;
pub use self::output::Output;
pub use self::stmt::Stmt;

pub fn compare_btree<T>(
    data1: &BTreeMap<String, T>,
    data2: &BTreeMap<String, T>,
    f: impl Fn(&str, &T, &T),
) {
    for (key1, item1) in data1 {
        let item2 = data2
            .get(key1)
            .unwrap_or_else(|| panic!("did not find key {key1} in data2"));
        f(key1, item1, item2);
    }
    assert_eq!(data1.len(), data2.len(), "too few items in first map");
}

pub fn compare_slice<T: core::fmt::Debug>(data1: &[T], data2: &[T], f: impl Fn(usize, &T, &T)) {
    let mut iter2 = data1.iter();
    for (i, item1) in data2.iter().enumerate() {
        if let Some(item2) = iter2.next() {
            f(i, item1, item2);
        } else {
            panic!("no more statements in second vec at index {i}");
        }
    }
    let remaining: Vec<_> = iter2.collect();
    if !remaining.is_empty() {
        panic!("remaining statements in second vec: {remaining:#?}");
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
