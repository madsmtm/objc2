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
#![allow(non_snake_case)]

#[doc(hidden)]
pub mod __macro_helpers;
#[macro_use]
mod iter;
#[cfg(feature = "Foundation_NSArray")]
pub mod array;
#[cfg(feature = "Foundation_NSAttributedString")]
mod attributed_string;
#[cfg(feature = "Foundation_NSBundle")]
mod bundle;
#[cfg(feature = "Foundation_NSObjCRuntime")]
mod comparison_result;
#[cfg(feature = "Foundation_NSObject")]
mod copying;
#[cfg(feature = "Foundation_NSData")]
mod data;
#[cfg(feature = "Foundation_NSDecimal")]
mod decimal;
#[cfg(feature = "Foundation_NSDictionary")]
pub mod dictionary;
#[cfg(feature = "Foundation_NSEnumerator")]
pub mod enumerator;
#[cfg(feature = "Foundation_NSError")]
mod error;
#[cfg(feature = "Foundation_NSException")]
mod exception;
#[cfg(feature = "Foundation_NSEnumerator")]
mod fast_enumeration_state;
mod generics;
#[cfg(feature = "Foundation_NSGeometry")]
mod geometry;
mod macros;
mod ns_consumed;
#[cfg(feature = "Foundation_NSValue")]
mod number;
#[cfg(feature = "Foundation_NSProcessInfo")]
mod process_info;
#[cfg(feature = "Foundation_NSRange")]
mod range;
#[cfg(feature = "Foundation_NSSet")]
pub mod set;
#[cfg(feature = "Foundation_NSString")]
mod string;
#[cfg(test)]
mod tests;
mod thread;
#[cfg(feature = "Foundation_NSObject")]
mod to_owned;
mod util;
#[cfg(feature = "Foundation_NSUUID")]
mod uuid;
#[cfg(feature = "Foundation_NSValue")]
mod value;

pub use crate::generated::Foundation::*;

#[cfg(feature = "Foundation_NSObjCRuntime")]
pub use self::comparison_result::NSComparisonResult;
#[cfg(feature = "Foundation_NSObject")]
pub use self::copying::{NSCopying, NSMutableCopying};
#[cfg(feature = "Foundation_NSDecimal")]
pub use self::decimal::NSDecimal;
#[cfg(feature = "Foundation_NSEnumerator")]
pub use self::fast_enumeration_state::NSFastEnumerationState;
pub use self::generics::*;
#[cfg(feature = "Foundation_NSGeometry")]
pub use self::geometry::{CGFloat, CGPoint, CGRect, CGSize, NSPoint, NSRect, NSRectEdge, NSSize};
#[cfg(feature = "Foundation_NSMapTable")]
pub use self::ns_consumed::NSFreeMapTable;
#[cfg(feature = "Foundation_NSRange")]
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
#[cfg(feature = "Foundation_NSZone")]
pub use objc2::runtime::NSZone;
#[doc(inline)]
#[cfg(feature = "Foundation_NSProxy")]
pub use objc2::runtime::__NSProxy as NSProxy;
pub use objc2::runtime::{NSObject, NSObjectProtocol};

#[cfg_attr(feature = "gnustep-1-7", link(name = "gnustep-base", kind = "dylib"))]
extern "C" {}

// MacTypes.h
#[allow(unused)]
pub(crate) type Boolean = u8; // unsigned char
#[allow(unused)]
pub(crate) type FourCharCode = u32;
#[allow(unused)]
pub(crate) type OSType = FourCharCode;
#[allow(unused)]
pub(crate) type UTF32Char = u32; // Or maybe Rust's char?
