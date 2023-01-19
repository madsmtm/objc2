#[path = "../generated/Accessibility/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "Accessibility", kind = "framework")]
extern "C" {}
