#[path = "../generated/FileProviderUI/mod.rs"]
mod generated;

pub use self::generated::*;

#[link(name = "FileProviderUI", kind = "framework")]
extern "C" {}
