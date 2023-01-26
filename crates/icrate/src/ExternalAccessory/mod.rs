#[path = "../generated/ExternalAccessory/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "ExternalAccessory", kind = "framework")]
extern "C" {}
