mod fixes;
#[path = "../generated/AuthenticationServices/mod.rs"]
mod generated;

pub use self::fixes::*;
pub use self::generated::*;

#[link(name = "AuthenticationServices", kind = "framework")]
extern "C" {}
