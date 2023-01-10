#![allow(unused_imports)]

mod capture;
mod device;
mod fixes;
#[path = "../generated/Metal/mod.rs"]
mod generated;
#[cfg(feature = "unstable-private")]
mod private;
mod slice;

pub use self::fixes::*;
pub use self::generated::*;

#[cfg_attr(feature = "apple", link(name = "Metal", kind = "framework"))]
extern "C" {}
