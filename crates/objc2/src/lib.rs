//! # Objective-C interface and runtime bindings
//!
//! Objective-C was the standard programming language on Apple platforms like
//! macOS, iOS, iPadOS, tvOS and watchOS. It is an object-oriented language
//! centered around "sending messages" to its instances - this can for the
//! most part be viewed as a simple method call.
//!
//! It has since been superseded by Swift, but most of the core libraries and
//! frameworks that are in use on Apple systems are still written in
//! Objective-C, and hence we would like the ability to interract with these
//! using Rust. This crate enables you to do that, in as safe a manner as
//! possible.
//!
//! See [the document on "Layered Safety"][layered-safety] for a bit of an
//! introduction to how the safety in this crate works, and see [`icrate`] for
//! higher-level bindings to Apple's frameworks.
//!
//! [layered-safety]: https://github.com/madsmtm/objc2/blob/master/LAYERED_SAFETY.md
//! [`icrate`]: https://docs.rs/icrate/latest/icrate/
//!
//!
//! ## Basic usage
//!
//! This example illustrates major parts of the functionality in this crate:
//!
//! First, we allocate a new [`NSObject`] using [`ClassType::alloc`].
//! Next, we initialize this object. It is ensured to be deallocated using
//! [`rc::Id`].
//! Now we're free to send messages to the object to our hearts desire using
//! the [`msg_send!`] or [`msg_send_id!`] macros (depending on the return type
//! of the method).
//! Finally, the `Id` goes out of scope, and the object is released and
//! deallocated.
//!
#![cfg_attr(feature = "apple", doc = "```")]
#![cfg_attr(not(feature = "apple"), doc = "```no_run")]
//! use objc2::{msg_send, msg_send_id, ClassType};
//! use objc2::ffi::NSUInteger;
//! use objc2::rc::Id;
//! use objc2::runtime::{NSObject, NSObjectProtocol};
//!
//! // Creation
//!
//! let obj1: Id<NSObject> = unsafe {
//!     msg_send_id![NSObject::alloc(), init]
//! };
//! // Or we can simply do
//! let obj2 = NSObject::new();
//!
//! // Usage
//!
//! let hash1: NSUInteger = unsafe { msg_send![&obj1, hash] };
//! let hash2: NSUInteger = unsafe { msg_send![&obj2, hash] };
//! assert_ne!(hash1, hash2);
//!
//! let is_kind: bool = unsafe {
//!     msg_send![&obj1, isKindOfClass: NSObject::class()]
//! };
//! assert!(is_kind);
//!
//! let obj1_self: Id<NSObject> = unsafe { msg_send_id![&obj1, self] };
//! assert_eq!(obj1, obj1_self);
//!
//! // Deallocation on drop
//! ```
//!
//! Note that this very simple example contains a lot of `unsafe` (which
//! should all ideally be justified with a `// SAFETY` comment). This is
//! required because our compiler can verify very little about the Objective-C
//! invocation, including all argument and return types used in [`msg_send!`];
//! we could have just as easily accidentally made `hash` an `f32`, or any
//! other type, and this would trigger undefined behaviour!
//!
//! See the `icrate` crate for much more ergonomic usage of the system
//! frameworks like `Foundation`, `AppKit`, `UIKit` and so on.
//!
//! Anyhow, all of this `unsafe` nicely leads us to another feature that this
//! crate has:
//!
//! [`NSObject`]: crate::runtime::NSObject
//! [`rc::Id`]: crate::rc::Id
//!
//!
//! ## Encodings and message type verification
//!
//! The Objective-C runtime includes encodings for each method that describe
//! the argument and return types. See the [`encode`] module for a full
//! overview of what this is.
//!
//! The important part is: To make message sending safer, all arguments and
//! return values for messages must implement [`encode::Encode`]. This allows
//! the Rust compiler to prevent you from passing e.g. a [`Vec`] into
//! Objective-C, which would both be UB and leak the vector.
//!
//! Furthermore, we can take advantage of the encodings provided by the
//! runtime to verify that the types used in Rust actually match the types
//! encoded for the method. This is not a perfect solution for ensuring safety
//! (some Rust types have the same Objective-C encoding, but are not
//! equivalent, such as `&T` and `*const T`), but it gets us much closer to
//! it!
//!
//! When `debug_assertions` are enabled we check the encoding every time you
//! send a message, and the message send will panic if they are not
//! equivalent.
//!
//! To take the example above, if we changed the `hash` method's return type
//! as in the following example, it'll panic if debug assertions are enabled:
//!
#![cfg_attr(all(feature = "apple", debug_assertions), doc = "```should_panic")]
#![cfg_attr(not(all(feature = "apple", debug_assertions)), doc = "```no_run")]
//! # use objc2::{msg_send, ClassType};
//! # use objc2::runtime::NSObject;
//! #
//! # let obj1 = NSObject::new();
//! #
//! // Wrong return type - this is UB!
//! let hash1: f32 = unsafe { msg_send![&obj1, hash] };
//! #
//! # panic!("does not panic in release mode, so for testing we make it!");
//! ```
//!
//! This library contains further such debug checks, most of which are enabled
//! by default. To enable all of them, use the `"verify"` cargo feature.
//!
//! [`Vec`]: std::vec::Vec
//!
//!
//! ## Crate features
//!
//! This crate exports several optional cargo features, see [`Cargo.toml`] for
//! an overview and description of these.
//!
//! [`Cargo.toml`]: https://github.com/madsmtm/objc2/blob/master/crates/objc2/Cargo.toml
//!
//!
//! ## Support for other Operating Systems
//!
//! The bindings can be used on Linux or *BSD utilizing the
//! [GNUstep Objective-C runtime](https://www.github.com/gnustep/libobjc2),
//! see the [`objc-sys`][`objc_sys`] crate for how to configure this.
//!
//!
//! ## Other functionality
//!
//! That was a quick introduction, this library also has [support for handling
//! exceptions][exc], [the ability to dynamically declare Objective-C
//! classes][declare], [advanced reference-counting utilities][rc], and more -
//! peruse the documentation at will!
//!
//! [exc]: crate::exception
//! [declare]: crate::declare
//! [rc]: crate::rc

#![no_std]
#![cfg_attr(
    feature = "unstable-autoreleasesafe",
    feature(negative_impls, auto_traits)
)]
#![cfg_attr(feature = "unstable-c-unwind", feature(c_unwind))]
#![cfg_attr(feature = "unstable-docsrs", feature(doc_cfg, doc_auto_cfg))]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::cargo)]
#![warn(clippy::ptr_as_ptr)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2/0.3.0-beta.5")]

#[cfg(not(feature = "alloc"))]
compile_error!("The `alloc` feature currently must be enabled.");

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

extern crate alloc;
extern crate std;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

#[doc(no_inline)]
pub use objc_sys as ffi;

pub use self::class_type::ClassType;
#[doc(no_inline)]
pub use self::encode::{Encode, Encoding, RefEncode};
pub use self::message::{Message, MessageArguments, MessageReceiver};
pub use self::protocol_type::ProtocolType;

#[cfg(feature = "objc2-proc-macros")]
#[doc(hidden)]
pub use objc2_proc_macros::__hash_idents;

#[cfg(not(feature = "objc2-proc-macros"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __hash_idents {
    // Noop; used to make our other macros a bit easier to read
    ($($x:tt)*) => {
        ()
    };
}

#[doc(hidden)]
pub mod __macro_helpers;
mod class_type;
pub mod declare;
pub mod encode;
pub mod exception;
mod macros;
mod message;
pub mod mutability;
mod protocol_type;
pub mod rc;
pub mod runtime;
#[cfg(test)]
mod test_utils;
mod verify;

// Link to Foundation to make NSObject work
#[cfg_attr(
    all(feature = "apple", not(feature = "unstable-compiler-rt")),
    link(name = "Foundation", kind = "framework")
)]
#[cfg_attr(feature = "gnustep-1-7", link(name = "gnustep-base", kind = "dylib"))]
extern "C" {}
