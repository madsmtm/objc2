//! Various tools for parsing and handling `translation-config.toml`.

mod config;
mod update;

pub use self::config::*;
pub use self::update::*;

/// The version of the framework crates.
pub const VERSION: &str = "0.3.2";
