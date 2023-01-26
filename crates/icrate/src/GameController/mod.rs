mod fixes;
#[path = "../generated/GameController/mod.rs"]
mod generated;

pub use self::fixes::*;
pub use self::generated::*;

#[link(name = "GameController", kind = "framework")]
extern "C" {}
