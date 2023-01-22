mod fixes;
#[path = "../generated/InputMethodKit/mod.rs"]
mod generated;

pub use self::fixes::*;
pub use self::generated::*;

#[link(name = "InputMethodKit", kind = "framework")]
extern "C" {}
