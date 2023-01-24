#[path = "../generated/Speech/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "Speech", kind = "framework")]
extern "C" {}
