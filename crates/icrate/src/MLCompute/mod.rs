#[path = "../generated/MLCompute/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "MLCompute", kind = "framework")]
extern "C" {}
