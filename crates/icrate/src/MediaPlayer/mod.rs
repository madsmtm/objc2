#[path = "../generated/MediaPlayer/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "MediaPlayer", kind = "framework")]
extern "C" {}
