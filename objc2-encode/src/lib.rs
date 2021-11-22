//! # Objective-C type-encoding
//!
//! This is re-exported into the top level of `objc2`.
//!
//! Further resources:
//! - <https://dmaclach.medium.com/objective-c-encoding-and-you-866624cc02de>
//! - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html>
//! - <https://dmaclach.medium.com/objective-c-encoding-and-you-866624cc02de>

#![no_std]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-encode/2.0.0-alpha.1")]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

#[cfg(any(test, doc))]
extern crate alloc;

mod encode;
mod encoding;
mod parse;

pub use self::encode::{Encode, EncodeArguments, RefEncode};
pub use self::encoding::Encoding;
