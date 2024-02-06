//! # Bindings to the `Foundation` framework
//!
//! This is the [`std`] equivalent for Objective-C, containing essential data
//! types, collections, and operating-system services.
//!
//! See [Apple's documentation](https://developer.apple.com/documentation/foundation?language=objc).
//!
//!
//! ## Examples
//!
//! Basic usage of a few Foundation types.
//!
//! ```ignore
#![doc = include_str!("../../../examples/basic_usage.rs")]
//! ```
//!
//! An example showing how to define your own interfaces to parts that may be missing in `icrate`.
//!
//! ```ignore
#![doc = include_str!("../../../examples/speech_synthesis.rs")]
//! ```

#[doc(hidden)]
pub mod __macro_helpers;
#[macro_use]
mod iter;
pub mod array;
mod attributed_string;
mod bundle;
mod comparison_result;
mod copying;
mod data;
mod debug;
mod decimal;
pub mod dictionary;
pub mod enumerator;
mod error;
mod exception;
mod fast_enumeration_state;
mod generics;
mod geometry;
mod macros;
mod not_found;
mod ns_consumed;
mod number;
mod process_info;
mod range;
pub mod set;
mod string;
#[cfg(test)]
mod tests;
mod thread;
mod to_owned;
mod util;
mod uuid;
mod value;

pub use crate::generated::Foundation::*;

pub use self::comparison_result::NSComparisonResult;
pub use self::copying::{NSCopying, NSMutableCopying};
pub use self::decimal::NSDecimal;
pub use self::fast_enumeration_state::NSFastEnumerationState;
pub use self::generics::*;
pub use self::geometry::{
    CGFloat, CGPoint, CGRect, CGSize, NSMaxXEdge, NSMaxYEdge, NSMinXEdge, NSMinYEdge, NSPoint,
    NSRect, NSRectEdge, NSRectEdgeMaxX, NSRectEdgeMaxY, NSRectEdgeMinX, NSRectEdgeMinY, NSSize,
};
pub use self::not_found::NSNotFound;
#[cfg(feature = "Foundation_NSMapTable")]
pub use self::ns_consumed::NSFreeMapTable;
pub use self::range::NSRange;
pub use self::thread::MainThreadMarker;
#[cfg(feature = "Foundation_NSThread")]
pub use self::thread::{is_main_thread, is_multi_threaded};
#[cfg(feature = "Foundation_NSThread")]
#[cfg(feature = "dispatch")]
pub use self::thread::{run_on_main, MainThreadBound};

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

#[cfg_attr(feature = "gnustep-1-7", link(name = "gnustep-base", kind = "dylib"))]
extern "C" {}
