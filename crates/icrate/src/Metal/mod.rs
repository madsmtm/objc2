//! # Bindings to the `Metal` framework
//!
//!
//! ## Examples
//!
//! Drawing a rotating triangle.
//!
//! ```ignore
#![doc = include_str!("../../examples/metal.rs")]
//! ```
#![allow(unused_imports)]

mod capture;
mod device;
mod fixes;
#[path = "../generated/Metal/mod.rs"]
mod generated;
#[cfg(feature = "unstable-private")]
mod private;
mod slice;

pub use self::fixes::*;
pub use self::generated::*;
