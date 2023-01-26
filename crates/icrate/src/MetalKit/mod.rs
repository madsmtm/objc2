#[path = "../generated/MetalKit/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "MetalKit", kind = "framework")]
extern "C" {}
