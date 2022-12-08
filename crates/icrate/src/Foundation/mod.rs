mod fixes;
#[allow(unused_imports)]
#[path = "../generated/Foundation/mod.rs"]
mod generated;

pub use objc2::ffi::NSIntegerMax;
pub use objc2::foundation::{CGFloat, CGPoint, CGRect, CGSize, NSZone};
pub use objc2::ns_string;

pub use self::fixes::*;
pub use self::generated::*;
