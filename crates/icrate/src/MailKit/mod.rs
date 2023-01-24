#[path = "../generated/MailKit/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "MailKit", kind = "framework")]
extern "C" {}
