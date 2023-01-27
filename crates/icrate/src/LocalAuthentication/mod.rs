mod fixes;
#[path = "../generated/LocalAuthentication/mod.rs"]
mod generated;

pub use self::fixes::*;
pub use self::generated::*;

#[link(name = "LocalAuthentication", kind = "framework")]
extern "C" {}
