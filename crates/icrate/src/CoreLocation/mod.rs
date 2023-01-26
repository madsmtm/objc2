#[path = "../generated/CoreLocation/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "CoreLocation", kind = "framework")]
extern "C" {}
