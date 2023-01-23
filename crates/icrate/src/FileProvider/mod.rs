#[path = "../generated/FileProvider/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "FileProvider", kind = "framework")]
extern "C" {}
