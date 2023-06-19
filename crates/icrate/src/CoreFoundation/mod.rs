mod fixes;
#[path = "../generated/CoreFoundation/mod.rs"]
mod generated;

#[allow(unreachable_pub)]
pub use self::fixes::*;
pub use self::generated::*;

#[cfg_attr(feature = "apple", link(name = "CoreFoundation", kind = "framework"))]
extern "C" {}
