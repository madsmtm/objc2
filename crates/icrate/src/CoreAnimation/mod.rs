//! # Core Animation
//!
//! This actually lives in the `QuartzCore` framework, but `CoreAnimation` is
//! the name that people use to refer to it.
#![doc(alias = "QuartzCore")]

mod fixes;
#[path = "../generated/QuartzCore/mod.rs"]
mod generated;

pub use self::fixes::*;
pub use self::generated::*;

#[link(name = "QuartzCore", kind = "framework")]
extern "C" {}
