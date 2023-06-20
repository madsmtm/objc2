//! # Core Animation
//!
//! This actually lives in the `QuartzCore` framework, but `CoreAnimation` is
//! the name that people use to refer to it.
#![doc(alias = "QuartzCore")]

#[path = "../generated/QuartzCore/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "QuartzCore", kind = "framework")]
extern "C" {}
