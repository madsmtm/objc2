#[path = "../generated/HealthKit/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "HealthKit", kind = "framework")]
extern "C" {}
