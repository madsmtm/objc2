//! # Bindings to the `WebKit` framework
//!
//!
//! ## Examples
//!
//! ```ignore
#![doc = include_str!("../../examples/browser.rs")]
//! ```
mod fixes;
#[path = "../generated/WebKit/mod.rs"]
mod generated;

#[allow(unreachable_pub)]
pub use self::fixes::*;
pub use self::generated::*;
