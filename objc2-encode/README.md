# `objc2-encode`

[![Latest version](https://badgen.net/crates/v/objc2-encode)](https://crates.io/crates/objc2-encode)
[![License](https://badgen.net/badge/license/MIT/blue)](../LICENSE.txt)
[![Documentation](https://docs.rs/objc2-encode/badge.svg)](https://docs.rs/objc2-encode/)
[![Apple CI](https://github.com/madsmtm/objc2/actions/workflows/apple.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/apple.yml)
[![GNUStep CI](https://github.com/madsmtm/objc2/actions/workflows/gnustep.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/gnustep.yml)

Objective-C type-encoding in Rust.

The Objective-C directive `@encode` encodes types as strings for usage in
various places in the runtime.

This crate provides the `Encoding` type to describe and compare these
type-encodings without memory allocation.

Additionally it provides traits for annotating types that has a corresponding
Objective-C encoding, respectively `Encode` for structs and `RefEncode` for
references (and `EncodeArguments` for function arguments).

These types are exported under the `objc2` crate as well, so usually you would
just use that.


## Examples

Implementing `Encode` and `RefEncode`:

```rust
use objc2_encode::{Encode, Encoding, RefEncode};

#[repr(C)]
struct MyObject {
    a: f32,
    b: i16,
}

unsafe impl Encode for MyObject {
    const ENCODING: Encoding<'static> = Encoding::Struct(
        "MyObject",
        &[f32::ENCODING, i16::ENCODING],
    );
}

assert_eq!(&MyObject::ENCODING, "{MyObject=fs}");

unsafe impl RefEncode for MyObject {
    const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
}

assert_eq!(&MyObject::ENCODING_REF, "^{MyObject=fs}");
```

An `Encoding` can be compared with an encoding string from the Objective-C
runtime:

```rust
use objc2_encode::Encode;
assert!(&i32::ENCODING == "i");
```

`Encoding` implements `Display` as its string representation. This can be
generated conveniently through the `to_string` method:

```rust
use objc2_encode::Encode;
assert_eq!(i32::ENCODING.to_string(), "i");
```

See the [`examples`] folder for more complex usage.

[`examples`]: https://github.com/madsmtm/objc2/tree/master/objc2-encode/examples


## Installation

```toml
[dependencies]
objc2-encode = "1.1.0"
```
