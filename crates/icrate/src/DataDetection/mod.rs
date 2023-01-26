#[path = "../generated/DataDetection/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "DataDetection", kind = "framework")]
extern "C" {}
