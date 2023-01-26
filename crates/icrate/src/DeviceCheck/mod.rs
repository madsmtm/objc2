#[path = "../generated/DeviceCheck/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "DeviceCheck", kind = "framework")]
extern "C" {}
