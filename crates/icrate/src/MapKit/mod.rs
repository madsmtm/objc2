mod fixes;
#[path = "../generated/MapKit/mod.rs"]
mod generated;

#[allow(unreachable_pub)]
pub use self::fixes::*;
pub use self::generated::*;

#[link(name = "MapKit", kind = "framework")]
extern "C" {}
