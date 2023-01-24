#[path = "../generated/UserNotifications/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "UserNotifications", kind = "framework")]
extern "C" {}
