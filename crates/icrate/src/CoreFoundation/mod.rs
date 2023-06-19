#[path = "../generated/CoreFoundation/mod.rs"]
mod generated;

pub use self::generated::*;

#[cfg_attr(feature = "apple", link(name = "CoreFoundation", kind = "framework"))]
extern "C" {}
