#[path = "../generated/ClassKit/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "ClassKit", kind = "framework")]
extern "C" {}
