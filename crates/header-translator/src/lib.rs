#![allow(clippy::collapsible_else_if)]

#[macro_use]
extern crate tracing;

mod availability;
mod clang_utils;
mod command;
mod context;
mod display_helper;
pub mod documentation;
mod expr;
mod global_analysis;
mod id;
mod library;
mod method;
mod module;
mod name_translation;
mod objc2_utils;
mod protocol;
mod rust_type;
mod stmt;
mod thread_safety;
mod unexposed_attr;

pub use self::availability::HOST_MACOS;
pub use self::clang_utils::immediate_children;
pub use self::command::{run_cargo_fmt, run_rustfmt};
pub use self::context::{Context, MacroEntity, MacroLocation};
pub use self::documentation::EXTRA_BLOCK_COMMANDS;
pub use self::global_analysis::global_analysis;
pub use self::id::{ItemIdentifier, Location};
pub use self::library::Library;
pub use self::module::Module;
pub use self::stmt::Stmt;
