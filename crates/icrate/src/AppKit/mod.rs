//! # Bindings to the `AppKit` framework
//!
//!
//! ## Examples
//!
//! Implementing `NSApplicationDelegate` for a custom class.
//!
//! ```ignore
#![doc = include_str!("../../examples/delegate.rs")]
//! ```
//!
//! An example showing basic and a bit more advanced usage of `NSPasteboard`.
//!
//! ```ignore
#![doc = include_str!("../../examples/nspasteboard.rs")]
//! ```

mod fixes;
#[path = "../generated/AppKit/mod.rs"]
mod generated;

pub use self::fixes::*;
pub use self::generated::*;
