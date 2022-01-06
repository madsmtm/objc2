//! # Bindings to the Objective-C Cocoa `Foundation` framework
//!
//! This library is very much in progress, consider using the more
//! battle-tested [`cocoa-foundation`] crate in the meantime.
//!
//! [`cocoa-foundation`]: https://crates.io/crates/cocoa-foundation

#![no_std]
#![warn(elided_lifetimes_in_paths)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
// TODO: #![warn(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-foundation/0.2.0-alpha.4")]

extern crate alloc;
extern crate std;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

pub use self::array::{NSArray, NSMutableArray};
pub use self::comparison_result::NSComparisonResult;
pub use self::copying::{INSCopying, INSMutableCopying};
pub use self::data::{NSData, NSMutableData};
pub use self::dictionary::{INSDictionary, NSDictionary};
pub use self::enumerator::{INSFastEnumeration, NSEnumerator, NSFastEnumerator};
pub use self::object::{INSObject, NSObject};
pub use self::range::NSRange;
pub use self::string::{INSString, NSString};
pub use self::value::{INSValue, NSValue};

#[cfg(apple)]
#[link(name = "Foundation", kind = "framework")]
extern "C" {}

#[cfg(gnustep)]
#[link(name = "gnustep-base", kind = "dylib")]
extern "C" {}

#[macro_use]
mod macros;

mod array;
mod comparison_result;
mod copying;
mod data;
mod dictionary;
mod enumerator;
mod object;
mod range;
mod string;
mod value;
