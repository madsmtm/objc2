#[path = "../generated/CoreAudioTypes/mod.rs"]
mod generated;

pub use self::generated::*;

#[cfg_attr(feature = "apple", link(name = "CoreAudioTypes", kind = "framework"))]
extern "C" {}
