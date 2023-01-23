#[path = "../generated/ExtensionKit/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "ExtensionKit", kind = "framework")]
extern "C" {}
