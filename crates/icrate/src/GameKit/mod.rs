#[path = "../generated/GameKit/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "GameKit", kind = "framework")]
extern "C" {}
