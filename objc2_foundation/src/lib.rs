#![no_std]
#![crate_name = "objc2_foundation"]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2_foundation/0.1.1")]

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

#[cfg(target_vendor = "apple")]
#[link(name = "Foundation", kind = "framework")]
extern "C" {}

#[cfg(not(target_vendor = "apple"))]
#[link(name = "gnustep-base", kind = "dylib")]
extern "C" {}

#[macro_use]
mod macros;

mod array;
mod data;
mod dictionary;
mod enumerator;
mod object;
mod string;
mod value;
