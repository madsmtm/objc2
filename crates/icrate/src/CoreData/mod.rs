mod fixes;
#[path = "../generated/CoreData/mod.rs"]
mod generated;

#[allow(unreachable_pub)]
pub use self::fixes::*;
pub use self::generated::*;

#[link(name = "CoreData", kind = "framework")]
extern "C" {}
