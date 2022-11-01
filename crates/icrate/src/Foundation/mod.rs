#[path = "../generated/Foundation/mod.rs"]
pub(crate) mod generated;

pub use objc2::ffi::NSIntegerMax;
pub use objc2::foundation::{
    CGFloat, CGPoint, CGRect, CGSize, NSObject, NSRange, NSTimeInterval, NSZone,
};
pub use objc2::ns_string;

// TODO
pub type NSRangePointer = *const NSRange;

pub use self::generated::__exported::*;
