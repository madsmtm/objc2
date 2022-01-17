//! Procedural macros for the [`objc2` project](https://github.com/madsmtm/objc2).
//!
//! You should not need to use this crate directly, all its public items are
//! exported in other crates.

#![warn(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::cargo)]
#![warn(clippy::ptr_as_ptr)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-proc-macros/0.0.0")]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

use proc_macro::TokenStream;

/// TODO
#[proc_macro_derive(Encode)]
pub fn encode_derive(input: TokenStream) -> TokenStream {
    dbg!(input);
    todo!()
}

/// TODO
#[proc_macro_derive(RefEncode)]
pub fn ref_encode_derive(input: TokenStream) -> TokenStream {
    dbg!(input);
    todo!()
}
