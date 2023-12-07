#[doc(hidden)]
pub mod __macro_helpers;
#[macro_use]
mod iter;
pub mod array;
mod attributed_string;
mod bundle;
mod comparison_result;
mod data;
pub mod dictionary;
pub mod enumerator;
mod error;
mod exception;
mod geometry;
mod macros;
mod number;
mod process_info;
mod range;
pub mod set;
mod string;
#[cfg(test)]
mod tests;
mod thread;
mod util;
mod uuid;
mod value;

pub use self::comparison_result::NSComparisonResult;
pub use self::geometry::{
    CGFloat, CGPoint, CGRect, CGSize, NSMaxXEdge, NSMaxYEdge, NSMinXEdge, NSMinYEdge, NSPoint,
    NSRect, NSRectEdge, NSRectEdgeMaxX, NSRectEdgeMaxY, NSRectEdgeMinX, NSRectEdgeMinY, NSSize,
};
pub use self::range::NSRange;
#[cfg(feature = "Foundation_NSThread")]
#[cfg(feature = "dispatch")]
pub use self::thread::MainThreadBound;
pub use self::thread::MainThreadMarker;
#[cfg(feature = "Foundation_NSThread")]
pub use self::thread::{is_main_thread, is_multi_threaded};

#[cfg(feature = "Foundation_NSString")]
#[doc(inline)]
pub use crate::__ns_string as ns_string;

// Available under Foundation, so makes sense here as well:
// https://developer.apple.com/documentation/foundation/numbers_data_and_basic_values?language=objc
pub use objc2::ffi::{NSInteger, NSUInteger};

// Special types that are stored in `objc2`, but really belong here
#[doc(inline)]
#[cfg(feature = "Foundation_NSProxy")]
pub use objc2::runtime::__NSProxy as NSProxy;
pub use objc2::runtime::{NSObject, NSObjectProtocol, NSZone};
