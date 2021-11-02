//! # Objective-C type-encoding
//!
//! This is re-exported into the top level of `objc2`.

#![no_std]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-encode/1.1.0")]

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
