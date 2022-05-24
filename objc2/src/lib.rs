//! # Objective-C interface and runtime bindings
//!
//! Objective-C is<sup>1</sup> the standard programming language on Apple
//! platforms like macOS, iOS, tvOS and watchOS. It is an object-oriented
//! language centered around sending messages to it's instances, which is for
//! the most part equivalent to a function call.
//!
//! Most of the core libraries and frameworks that are in use on Apple systems
//! are written in Objective-C, and hence we would like the ability to
//! interract with these using Rust; this crate enables you to do that, in
//! as safe a manner as possible.
//!
//! <sup>1: Yes, I know, "was", Swift now exists. All the existing frameworks
//! are written in Objective-C though, so the point still holds.</sup>
//!
//!
//! ## Basic usage
//!
//! This example illustrates major parts of the functionality in this crate:
//!
//! First, we get a reference to the `NSObject`'s [`runtime::Class`] using the
//! [`class!`] macro.
//! Next, we creates a new [`runtime::Object`] pointer, and ensures it is
//! deallocated after we've used it by putting it into an [`rc::Owned`]
//! [`rc::Id`].
//! Now we send messages to the object to our hearts desire using
//! the [`msg_send!`] macro, and lastly, the `Id<Object, _>` goes out of
//! scope, and the object is deallocated.
//!
#![cfg_attr(feature = "apple", doc = "```")]
#![cfg_attr(not(feature = "apple"), doc = "```no_run")]
//! use objc2::{class, msg_send, msg_send_bool};
//! use objc2::ffi::NSUInteger;
//! use objc2::rc::{Id, Owned};
//! use objc2::runtime::Object;
//!
//! // Creation
//! let cls = class!(NSObject);
//! let obj: *mut Object = unsafe { msg_send![cls, new] };
//! let obj: Id<Object, Owned> = unsafe {
//!     Id::new(obj).expect("Failed allocating object")
//! };
//!
//! // Usage
//! let hash: NSUInteger = unsafe { msg_send![obj, hash] };
//! let is_kind = unsafe { msg_send_bool![obj, isKindOfClass: cls] };
//! assert!(is_kind);
//! ```
//!
//! Note that this very simple example contains **a lot** of `unsafe` (which
//! should all ideally be justified with a `// SAFETY` comment). This is
//! required because our compiler can verify very little about the Objective-C
//! invocation, including all argument and return types used in [`msg_send!`];
//! we could have just as easily accidentally made `hash` an `f32`, or any
//! other type, and this would trigger undefined behaviour!
//!
//! Making the ergonomics better is something that is currently being worked
//! on, see e.g. the [`objc2-foundation`] crate for more ergonomic usage of at
//! least the `Foundation` framework.
//!
//! Anyhow, this nicely leads us to another feature that this crate has:
//!
//! [`runtime::Class`]: crate::runtime::Class
//! [`runtime::Object`]: crate::runtime::Object
//! [`rc::Owned`]: crate::rc::Owned
//! [`rc::Id`]: crate::rc::Id
//! [`objc2-foundation`]: https://crates.io/crates/objc2-foundation
//!
//!
//! ## Encodings and message type verification
//!
//! The Objective-C runtime includes encodings for each method that describe
//! the argument and return types. See the [`objc2-encode`] crate for the
//! full overview of what this is.
//!
//! The important part is, to make message sending _safer_ (not fully safe),
//! all arguments and return values for messages must implement [`Encode`].
//! This allows the Rust compiler to prevent you from passing e.g. a [`Box`]
//! into Objective-C, which would both be UB and leak the box.
//!
//! Furthermore, this crate can take advantage of the encodings provided by
//! the runtime to verify that the types used in Rust match the types encoded
//! for the method. This is not a perfect solution for ensuring safety of
//! message sends (some Rust types have the same encoding, but are not
//! equivalent), but it gets us much closer to it!
//!
//! To use this functionality, enable the `"verify_message"` cargo feature
//! while debugging. With this feature enabled, encoding types are checked
//! every time your send a message, and the message send will panic if they
//! are not equivalent.
//!
//! [`objc2-encode`]: objc2_encode
//! [`Box`]: std::boxed::Box
//!
//!
//! ## Crate features
//!
//! This crate exports several optional cargo features, see [`Cargo.toml`] for
//! an overview and description of these.
//!
//! [`Cargo.toml`]: https://github.com/madsmtm/objc2/blob/master/objc2/Cargo.toml
//!
//!
//! ## Support for other Operating Systems
//!
//! The bindings can be used on Linux or *BSD utilizing the
//! [GNUstep Objective-C runtime](https://www.github.com/gnustep/libobjc2),
//! see the [`objc-sys`][`objc_sys`] crate for how to configure this.
//!
//!
//! ## Other features
//!
//! Anyhow, that was a quick introduction, this library also has [support for
//! handling exceptions][exc], [the ability to dynamically declare Objective-C
//! classes][declare], [more advanced reference-counting utilities][rc] and
//! more, peruse the documentation at will!
//!
#![cfg_attr(feature = "exception", doc = "[exc]: crate::exception")]
#![cfg_attr(not(feature = "exception"), doc = "[exc]: #exception-feature-disabled")]
//! [declare]: crate::declare
//! [rc]: crate::rc

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
#![doc(html_root_url = "https://docs.rs/objc2/0.3.0-alpha.6")]

extern crate alloc;
extern crate std;

// The example uses NSObject without doing the __gnustep_hack
#[cfg(all(feature = "apple", doctest))]
#[doc = include_str!("../README.md")]
extern "C" {}

pub use objc2_encode as encode;
pub use objc_sys as ffi;

#[doc(no_inline)]
pub use objc2_encode::{Encode, EncodeArguments, Encoding, RefEncode};

pub use crate::message::{Message, MessageArguments, MessageError, MessageReceiver};

pub use crate::cache::CachedClass as __CachedClass;
pub use crate::cache::CachedSel as __CachedSel;

#[macro_use]
mod macros;

mod bool;
mod cache;
pub mod declare;
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
#[cfg(feature = "gnustep-1-7")]
pub mod __gnustep_hack {
    use super::runtime::Class;

    #[link(name = "gnustep-base", kind = "dylib")]
    // This linking doesn't have to be on the correct `extern` block.
    extern "C" {
        static _OBJC_CLASS_NSObject: Class;
    }

    pub unsafe fn get_class_to_force_linkage() -> &'static Class {
        unsafe { core::ptr::read_volatile(&&_OBJC_CLASS_NSObject) }
    }

    #[test]
    fn ensure_linkage() {
        unsafe { get_class_to_force_linkage() };
    }
}
