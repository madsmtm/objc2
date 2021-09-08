//! # Objective-C type-encoding

#![no_std]
#![warn(missing_docs)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2_encode/1.1.0")]

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
