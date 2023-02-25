// This actually lives in the `Photos` framework, but `PhotoKit` is the name that people use to refer to it.
#![doc(alias = "Photos")]

#[path = "../generated/Photos/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "Photos", kind = "framework")]
extern "C" {}
