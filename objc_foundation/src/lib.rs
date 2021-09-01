#![no_std]
#![crate_name = "objc_foundation"]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc-foundation/0.1.1")]

extern crate alloc;
extern crate std;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

pub use self::array::{
    INSArray, INSMutableArray, NSArray, NSComparisonResult, NSMutableArray, NSMutableSharedArray,
    NSRange, NSSharedArray,
};
pub use self::data::{INSData, INSMutableData, NSData, NSMutableData};
pub use self::dictionary::{INSDictionary, NSDictionary};
pub use self::enumerator::{INSFastEnumeration, NSEnumerator, NSFastEnumerator};
pub use self::object::{INSObject, NSObject};
pub use self::string::{INSCopying, INSMutableCopying, INSString, NSString};
pub use self::value::{INSValue, NSValue};

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[link(name = "Foundation", kind = "framework")]
extern "C" {}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
use objc::runtime::Class;

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
#[link(name = "gnustep-base", kind = "dylib")]
extern "C" {}

// Split up to illustrate that the linking doesn't have to be annotated on the
// correct `extern` block.
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
extern "C" {
    static _OBJC_CLASS_NSObject: Class;
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
#[allow(dead_code)]
unsafe fn get_class_to_force_linkage() -> &'static Class {
    &_OBJC_CLASS_NSObject
}

#[macro_use]
mod macros;

mod array;
mod data;
mod dictionary;
mod enumerator;
mod object;
mod string;
mod value;
