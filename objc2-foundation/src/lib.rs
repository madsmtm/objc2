//! # Bindings to the Objective-C Cocoa `Foundation` framework
//!
//! The [`std`] equivalent for Objective-C, containing essential data types,
//! collections, and operating-system services.
//!
//! See [Apple's documentation](https://developer.apple.com/documentation/foundation?language=objc).
//!
//! **NOTICE: This library is very much in progress, consider using the more
//! battle-tested [`cocoa-foundation`] crate in the meantime.**
//!
//! [`cocoa-foundation`]: https://crates.io/crates/cocoa-foundation
//!
//! ## Philosophy
//!
//! The `Foundation` framework is _huge_! If we aspired to map every API it
//! exposes (a lot of it is just helper methods to make Objective-C more
//! ergonomic), this library would never be finished. Instead, our focus lies
//! on conversion methods, to allow easily using them from Rust.
//!
//! If you find some API that an object doesn't expose (but should), we gladly
//! accept [pull requests]. Anyhow, if it is something that is out of scope,
//! these objects implement the [`objc2::Message`] trait, so you can always
//! just manually implement call a method using the [`objc2::msg_send!`]
//! macro.
//!
//! [pull requests]: https://github.com/madsmtm/objc2/pulls

#![no_std]
#![warn(elided_lifetimes_in_paths)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
// TODO: #![warn(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(clippy::missing_safety_doc)] // TODO: Remove this
#![warn(clippy::cargo)]
#![warn(clippy::ptr_as_ptr)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-foundation/0.2.0-alpha.6")]
#![cfg_attr(feature = "unstable-docsrs", feature(doc_auto_cfg))]

extern crate alloc;
extern crate std;

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

pub use self::array::NSArray;
pub use self::attributed_string::{NSAttributedString, NSAttributedStringKey};
pub use self::comparison_result::NSComparisonResult;
pub use self::copying::{NSCopying, NSMutableCopying};
pub use self::data::NSData;
pub use self::dictionary::NSDictionary;
pub use self::enumerator::{NSEnumerator, NSFastEnumeration, NSFastEnumerator};
pub use self::exception::NSException;
pub use self::geometry::{CGFloat, NSPoint, NSRect, NSSize};
pub use self::mutable_array::NSMutableArray;
pub use self::mutable_attributed_string::NSMutableAttributedString;
pub use self::mutable_data::NSMutableData;
pub use self::mutable_string::NSMutableString;
pub use self::object::NSObject;
pub use self::process_info::NSProcessInfo;
pub use self::range::NSRange;
pub use self::string::NSString;
pub use self::thread::{is_main_thread, is_multi_threaded, MainThreadMarker, NSThread};
#[cfg(not(macos_10_7))] // Temporary
pub use self::uuid::NSUUID;
pub use self::value::NSValue;
pub use self::zone::NSZone;

// Available under Foundation, so makes sense here as well:
// https://developer.apple.com/documentation/foundation/numbers_data_and_basic_values?language=objc
#[doc(no_inline)]
pub use objc2::ffi::{NSInteger, NSUInteger};

// For macros
#[doc(hidden)]
pub use core as __core;
#[doc(hidden)]
pub extern crate std as __std;

// Expose the version of objc2 that this crate uses
pub use objc2;

#[cfg(feature = "apple")]
#[link(name = "Foundation", kind = "framework")]
extern "C" {}

#[cfg(feature = "gnustep-1-7")]
#[link(name = "gnustep-base", kind = "dylib")]
extern "C" {}

mod array;
mod attributed_string;
mod comparison_result;
mod copying;
mod data;
mod declare_macro;
mod dictionary;
mod enumerator;
mod exception;
mod geometry;
mod macros;
mod mutable_array;
mod mutable_attributed_string;
mod mutable_data;
mod mutable_string;
mod object;
mod process_info;
mod range;
mod string;
mod thread;
// Temporarily disable testing UUID on macOS 10.7 until
#[cfg(not(macos_10_7))]
mod uuid;
mod value;
mod zone;

#[cfg(test)]
mod tests {
    use core::panic::{RefUnwindSafe, UnwindSafe};
    use objc2::rc::{Id, Owned, Shared};

    use super::*;

    // We expect most Foundation types to be UnwindSafe and RefUnwindSafe,
    // since they follow Rust's usual mutability rules (&T = immutable).
    //
    // A _lot_ of Objective-C code out there would be subtly broken if e.g.
    // `NSString` wasn't exception safe!
    // As an example: -[NSArray objectAtIndex:] can throw, but it is still
    // perfectly valid to access the array after that!
    //
    // Note that e.g. `&mut NSMutableString` is still not exception safe, but
    // that is the entire idea of `UnwindSafe` (that if the object could have
    // been mutated, it is not exception safe).
    //
    // Also note that this is still just a speed bump, not actually part of
    // any unsafe contract; we really protect against it if something is not
    // exception safe, since `UnwindSafe` is a safe trait.
    fn assert_unwindsafe<T: UnwindSafe + RefUnwindSafe>() {}

    fn assert_auto_traits<T: Send + Sync + UnwindSafe + RefUnwindSafe>() {
        assert_unwindsafe::<T>();
    }

    #[test]
    fn send_sync_unwindsafe() {
        assert_auto_traits::<NSArray<NSString, Shared>>();
        assert_auto_traits::<NSArray<NSString, Owned>>();
        assert_auto_traits::<Id<NSArray<NSString, Shared>, Shared>>();
        assert_auto_traits::<Id<NSArray<NSString, Owned>, Shared>>();
        assert_auto_traits::<Id<NSArray<NSString, Shared>, Owned>>();
        assert_auto_traits::<Id<NSArray<NSString, Owned>, Owned>>();

        assert_auto_traits::<NSAttributedString>();
        assert_auto_traits::<NSComparisonResult>();
        assert_auto_traits::<NSData>();
        assert_auto_traits::<NSDictionary<NSString, Shared>>();
        // TODO: Figure out if Send + Sync is safe?
        // assert_auto_traits::<NSEnumerator<NSString>>();
        // assert_auto_traits::<NSFastEnumerator<NSArray<NSString, Shared>>>();
        assert_auto_traits::<NSException>();
        assert_auto_traits::<CGFloat>();
        assert_auto_traits::<NSPoint>();
        assert_auto_traits::<NSRect>();
        assert_auto_traits::<NSSize>();
        assert_auto_traits::<NSMutableArray<NSString, Shared>>();
        assert_auto_traits::<NSMutableAttributedString>();
        assert_auto_traits::<NSMutableData>();
        assert_auto_traits::<NSMutableString>();
        // assert_auto_traits::<NSObject>(); // Intentional
        assert_auto_traits::<NSProcessInfo>();
        assert_auto_traits::<NSRange>();
        assert_auto_traits::<NSString>();
        assert_unwindsafe::<MainThreadMarker>(); // Intentional
        assert_auto_traits::<NSThread>();
        #[cfg(not(macos_10_7))]
        assert_auto_traits::<NSUUID>();
        assert_auto_traits::<NSValue<i32>>();
        assert_unwindsafe::<NSZone>(); // Intentional
    }
}
