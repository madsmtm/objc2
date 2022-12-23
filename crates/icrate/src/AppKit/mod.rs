mod fixes;
#[path = "../generated/AppKit/mod.rs"]
mod generated;

pub use self::fixes::*;
pub use self::generated::*;

#[cfg_attr(feature = "apple", link(name = "AppKit", kind = "framework"))]
#[cfg_attr(feature = "gnustep-1-7", link(name = "gnustep-gui", kind = "dylib"))]
extern "C" {}
