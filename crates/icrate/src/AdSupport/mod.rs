#[path = "../generated/AdSupport/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "AdSupport", kind = "framework")]
extern "C" {}
