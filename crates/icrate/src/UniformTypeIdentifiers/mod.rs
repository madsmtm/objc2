#[path = "../generated/UniformTypeIdentifiers/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "UniformTypeIdentifiers", kind = "framework")]
extern "C" {}
