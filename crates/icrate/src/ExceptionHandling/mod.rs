#[path = "../generated/ExceptionHandling/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "ExceptionHandling", kind = "framework")]
extern "C" {}
