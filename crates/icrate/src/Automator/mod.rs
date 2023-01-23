#[path = "../generated/Automator/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "Automator", kind = "framework")]
extern "C" {}
