#[path = "../generated/Contacts/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "Contacts", kind = "framework")]
extern "C" {}
