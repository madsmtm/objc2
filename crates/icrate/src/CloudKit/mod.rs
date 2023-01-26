#[path = "../generated/CloudKit/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "CloudKit", kind = "framework")]
extern "C" {}
