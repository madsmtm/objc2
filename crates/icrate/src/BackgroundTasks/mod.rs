#[path = "../generated/BackgroundTasks/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "BackgroundTasks", kind = "framework")]
extern "C" {}
