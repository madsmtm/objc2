#[path = "../generated/LinkPresentation/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "LinkPresentation", kind = "framework")]
extern "C" {}
