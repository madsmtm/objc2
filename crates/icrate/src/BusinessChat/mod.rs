#[path = "../generated/BusinessChat/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "BusinessChat", kind = "framework")]
extern "C" {}
