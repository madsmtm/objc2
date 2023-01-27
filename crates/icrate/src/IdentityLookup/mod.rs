#[path = "../generated/IdentityLookup/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "IdentityLookup", kind = "framework")]
extern "C" {}
