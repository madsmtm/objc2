//! # Objective-C type-encoding
//!
//! This is re-exported into the top level of `objc2`.
//!
//! The Objective-C directive `@encode` encodes types as strings, and this is
//! used in various places in the runtime.
//!
//! This crate provides the [`Encoding`] type to efficiently describe and
//! compare these type-encodings.
//!
//! Additionally it provides traits for annotating types that has an
//! Objective-C encoding: Specifically [`Encode`] for structs, [`RefEncode`]
//! for references and [`EncodeArguments`] for function arguments.
//!
//! These types are exported under the [`objc2`] crate as well, so usually you
//! would just use them from there.
//!
//! ## Example
//!
//! Implementing [`Encode`] and [`RefEncode`] for a custom type:
//!
//! ```
//! use objc2_encode::{Encode, Encoding, RefEncode};
//! // or from objc2:
//! // use objc2::{Encode, Encoding, RefEncode};
//!
//! #[repr(C)]
//! struct MyStruct {
//!     a: f32, // float
//!     b: i16, // int16_t
//! }
//!
//! unsafe impl Encode for MyStruct {
//!     const ENCODING: Encoding<'static> = Encoding::Struct(
//!         "MyStruct", // Must use the same name as defined in C header files
//!         &[
//!             f32::ENCODING, // Same as Encoding::Float
//!             i16::ENCODING, // Same as Encoding::Short
//!         ],
//!     );
//! }
//!
//! // @encode(MyStruct) -> "{MyStruct=fs}"
//! assert!(MyStruct::ENCODING.equivalent_to_str("{MyStruct=fs}"));
//!
//! unsafe impl RefEncode for MyStruct {
//!     // Note that if `MyStruct` is an Objective-C instance, this should
//!     // be `Encoding::Object`.
//!     const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
//! }
//!
//! // @encode(MyStruct*) -> "^{MyStruct=fs}"
//! assert!(MyStruct::ENCODING_REF.equivalent_to_str("^{MyStruct=fs}"));
//! ```
//!
//! See the [`examples`] folder for more complex usage.
//!
//! [`examples`]: https://github.com/madsmtm/objc2/tree/master/objc2-encode/examples
//!
//! Further resources:
//! - [Objective-C, Encoding and You](https://dmaclach.medium.com/objective-c-encoding-and-you-866624cc02de).
//! - [Apple's documentation on Type Encodings](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html).
//! - [How are the digits in ObjC method type encoding calculated?](https://stackoverflow.com/a/11527925)
//! - [`clang`'s source code for generating `@encode`](https://github.com/llvm/llvm-project/blob/fae0dfa6421ea6c02f86ba7292fa782e1e2b69d1/clang/lib/AST/ASTContext.cpp#L7500-L7850).
//!
//! [`objc2`]: https://crates.io/crates/objc2

#![no_std]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-encode/2.0.0-beta.1")]

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
