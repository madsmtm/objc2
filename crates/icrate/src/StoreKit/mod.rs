#[path = "../generated/StoreKit/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "StoreKit", kind = "framework")]
extern "C" {}
