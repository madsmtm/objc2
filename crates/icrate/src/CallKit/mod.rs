#[path = "../generated/CallKit/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "CallKit", kind = "framework")]
extern "C" {}
