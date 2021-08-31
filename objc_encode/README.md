# `objc-encode`

[![Latest version](https://badgen.net/crates/v/objc-encode)](https://crates.io/crates/objc-encode)
[![License](https://badgen.net/badge/license/MIT/blue)](../LICENSE.txt)
[![Documentation](https://docs.rs/objc-encode/badge.svg)](https://docs.rs/objc-encode/)
[![CI Status](https://github.com/madsmtm/objc/workflows/CI/badge.svg)](https://github.com/madsmtm/objc/actions)

Objective-C type encoding creation and parsing in Rust.

The Objective-C compiler encodes types as strings for usage in the runtime.
This crate aims to provide a strongly-typed (rather than stringly-typed) way
to create and describe these type encodings without memory allocation in Rust.


## Implementing Encode

This crate declares an `Encode` trait that can be implemented for types that
the Objective-C compiler can encode. Implementing this trait looks like:

```rust
use objc::{Encode, Encoding};

#[cfg(target_pointer_width = "32")]
type CGFloat = f32;

#[cfg(target_pointer_width = "64")]
type CGFloat = f64;

#[repr(C)]
struct CGPoint {
    x: CGFloat,
    y: CGFloat,
}

unsafe impl Encode for CGPoint {
    const ENCODING: Encoding<'static> =
        Encoding::Struct("CGPoint", &[CGFloat::ENCODING, CGFloat::ENCODING]);
}
```

For an example of how this works with more complex types, like structs
containing structs, see the `core_graphics` example.

## Comparing with encoding strings

An `Encoding` can be compared with an encoding string from the Objective-C
runtime:

```rust
use objc::Encode;

assert!(&i32::ENCODING == "i");
```

## Generating encoding strings

Every `Encoding` implements `Display` as its string representation.
This can be generated conveniently through the `to_string` method:

```rust
use objc::Encode;

assert_eq!(i32::ENCODING.to_string(), "i");
```
