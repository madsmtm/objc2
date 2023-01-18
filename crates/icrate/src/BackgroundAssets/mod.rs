#[path = "../generated/BackgroundAssets/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "BackgroundAssets", kind = "framework")]
extern "C" {}
