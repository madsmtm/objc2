#![crate_name = "objc_foundation"]
#![crate_type = "lib"]

#![feature(default_type_params, globs, phase, macro_rules)]

#[phase(plugin, link)]
extern crate objc;

pub use self::array::{
    INSArray, INSMutableArray, INSOwnedArray, INSSharedArray,
    NSArray, NSEnumerator, NSMutableArray, NSRange,
    NSMutableSharedArray, NSSharedArray,
};
pub use self::dictionary::{INSDictionary, NSDictionary};
pub use self::object::{class, ClassName, INSObject, NSObject};
pub use self::string::{INSCopying, INSMutableCopying, INSString, NSString};
pub use self::value::{INSValue, NSValue};

mod macros;

mod array;
mod dictionary;
mod object;
mod string;
mod value;

// Shim to re-export under the objc_foundation:: path for macros
mod objc_foundation {
    pub use super::*;
}
