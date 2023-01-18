#[path = "../generated/WebKit/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "WebKit", kind = "framework")]
extern "C" {}
