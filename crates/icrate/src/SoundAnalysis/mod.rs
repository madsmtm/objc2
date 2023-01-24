#[path = "../generated/SoundAnalysis/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "SoundAnalysis", kind = "framework")]
extern "C" {}
