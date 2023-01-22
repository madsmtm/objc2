#[path = "../generated/EventKit/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "EventKit", kind = "framework")]
extern "C" {}
