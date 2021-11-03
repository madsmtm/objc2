/*!
Objective-C Runtime bindings and wrapper for Rust.

# Messaging objects

Objective-C objects can be messaged using the [`msg_send!`](macro.msg_send!.html) macro:

``` no_run
# use objc2::{class, msg_send};
# use objc2::runtime::{Bool, Class, Object};
# unsafe {
let cls = class!(NSObject);
let obj: *mut Object = msg_send![cls, new];
let hash: usize = msg_send![obj, hash];
let is_kind: Bool = msg_send![obj, isKindOfClass: cls];
// Even void methods must have their return type annotated
let _: () = msg_send![obj, release];
# }
```

# Reference counting

Utilities for reference counting Objective-C objects are provided in the
[`rc`](rc/index.html) module.

# Declaring classes

Objective-C classes can even be declared from Rust using the functionality of
the [`declare`](declare/index.html) module.

# Exceptions

By default, if the `msg_send!` macro causes an exception to be thrown, this
will unwind into Rust resulting in unsafe, undefined behavior.
However, this crate has an `"catch_all"` feature which, when enabled, wraps
each `msg_send!` in a `@try`/`@catch` and panics if an exception is caught,
preventing Objective-C from unwinding into Rust.

# Message type verification

The Objective-C runtime includes encodings for each method that describe the
argument and return types. This crate can take advantage of these encodings to
verify that the types used in Rust match the types encoded for the method.

To use this functionality, enable the `"verify_message"` feature.
With this feature enabled, type checking is performed for every message send,
which also requires that all arguments and return values for all messages
implement `Encode`.

If this requirement is burdensome or you'd rather
just verify specific messages, you can call the
[`Message::verify_message`](trait.Message.html#method.verify_message) method
for specific selectors.

# Support for other Operating Systems

The bindings can be used on Linux or *BSD utilizing the
[GNUstep Objective-C runtime](https://www.github.com/gnustep/libobjc2).
*/

#![no_std]
#![cfg_attr(
    feature = "unstable_autoreleasesafe",
    feature(negative_impls, auto_traits)
)]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2/0.2.7")]

extern crate alloc;
extern crate std;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

pub use objc2_encode::{Encode, EncodeArguments, Encoding, RefEncode};

pub use crate::message::{Message, MessageArguments, MessageError, MessageReceiver};

pub use crate::cache::CachedClass as __CachedClass;
pub use crate::cache::CachedSel as __CachedSel;

#[macro_use]
mod macros;

mod bool;
mod cache;
pub mod declare;
mod encode;
#[cfg(feature = "exception")]
pub mod exception;
mod message;
pub mod rc;
pub mod runtime;

#[cfg(test)]
mod test_utils;

/// Hacky way to make GNUStep link properly to Foundation while testing.
///
/// This is a temporary solution to make our CI work for now!
#[doc(hidden)]
#[cfg(gnustep)]
pub mod __gnustep_hack {
    use super::runtime::Class;

    #[link(name = "gnustep-base", kind = "dylib")]
    // This linking doesn't have to be on the correct `extern` block.
    extern "C" {
        static _OBJC_CLASS_NSObject: Class;
    }

    pub unsafe fn get_class_to_force_linkage() -> &'static Class {
        unsafe { &_OBJC_CLASS_NSObject }
    }

    #[test]
    fn ensure_linkage() {
        unsafe { get_class_to_force_linkage() };
    }
}
