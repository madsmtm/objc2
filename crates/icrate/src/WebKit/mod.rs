mod fixes;
#[path = "../generated/WebKit/mod.rs"]
mod generated;

#[allow(unreachable_pub)]
pub use self::fixes::*;
pub use self::generated::*;

#[link(name = "WebKit", kind = "framework")]
extern "C" {}
