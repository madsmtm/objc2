#![crate_name = "objc_foundation"]
#![crate_type = "lib"]

#![feature(core, hash, std_misc)]

extern crate libc;
#[macro_use]
extern crate objc;

pub use self::array::{
    INSArray, INSMutableArray, INSOwnedArray, INSSharedArray,
    NSArray, NSComparisonResult, NSEnumerator, NSMutableArray, NSRange,
    NSMutableSharedArray, NSSharedArray,
};
pub use self::dictionary::{INSDictionary, NSDictionary};
pub use self::object::{INSObject, NSObject};
pub use self::string::{INSCopying, INSMutableCopying, INSString, NSString};
pub use self::value::{INSValue, NSValue};

#[macro_use]
mod macros;

mod array;
mod dictionary;
mod object;
mod string;
mod value;
