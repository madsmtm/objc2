mod fixes;
#[path = "../generated/MapKit/mod.rs"]
mod generated;

pub use self::fixes::*;
pub use self::generated::*;

#[link(name = "MapKit", kind = "framework")]
extern "C" {}
