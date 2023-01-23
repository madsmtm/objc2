#[path = "../generated/AdServices/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "AdServices", kind = "framework")]
extern "C" {}
