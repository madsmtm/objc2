/*!
Objective-C type encoding creation and parsing in Rust.

The Objective-C compiler encodes types as strings for usage in the runtime.
This crate aims to provide a strongly-typed (rather than stringly-typed) way
to create and describe these type encodings without memory allocation in Rust.

# Implementing Encode

This crate declares an [`Encode`] trait that can be implemented for types that
the Objective-C compiler can encode. Implementing this trait looks like:

```ignore
unsafe impl Encode for CGPoint {
    const ENCODING: Encoding<'static> =
        Encoding::Struct("CGPoint", &[CGFloat::ENCODING, CGFLOAT::ENCODING]);
}
```

For an example of how this works with more complex types, like structs
containing structs, see the `core_graphics` example.

# Comparing with encoding strings

An [`Encoding`] can be compared with an encoding string from the Objective-C
runtime:

```
# use objc_encode::Encode;
assert!(&i32::ENCODING == "i");
```

# Generating encoding strings

Every [`Encoding`] implements [`Display`][`core::fmt::Display`] as its string
representation. This can be generated conveniently through the
[`to_string`][`alloc::string::ToString::to_string`] method:

```
# use objc_encode::Encode;
assert_eq!(i32::ENCODING.to_string(), "i");
```
*/

#![no_std]
#![warn(missing_docs)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc-encode/1.1.0")]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
extern "C" {}

#[cfg(any(test, doc))]
extern crate alloc;

mod encode;
mod encoding;
mod parse;

pub use crate::encode::Encode;
pub use crate::encoding::Encoding;
