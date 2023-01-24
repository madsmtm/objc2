#[path = "../generated/LocalAuthenticationEmbeddedUI/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "LocalAuthenticationEmbeddedUI", kind = "framework")]
extern "C" {}
