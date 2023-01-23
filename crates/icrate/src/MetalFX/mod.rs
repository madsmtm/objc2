#[path = "../generated/MetalFX/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "MetalFX", kind = "framework")]
extern "C" {}
