#[path = "../generated/OSAKit/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "OSAKit", kind = "framework")]
extern "C" {}
