mod fixes;
#[path = "../generated/Foundation/mod.rs"]
mod generated;

pub use self::fixes::*;
pub use self::generated::*;

pub use objc2::foundation::*;
pub use objc2::ns_string;
