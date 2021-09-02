# `objc-encode` - Objective-C type-encoding in Rust

[![Latest version](https://badgen.net/crates/v/objc-encode)](https://crates.io/crates/objc-encode)
[![License](https://badgen.net/badge/license/MIT/blue)](../LICENSE.txt)
[![Documentation](https://docs.rs/objc-encode/badge.svg)](https://docs.rs/objc-encode/)
[![CI Status](https://github.com/madsmtm/objc/workflows/CI/badge.svg)](https://github.com/madsmtm/objc/actions)

The Objective-C directive `@encode` encodes types as strings for usage in
various places in the runtime.

This crate provides the `Encoding` type to describe and compare these
type-encodings without memory allocation.

Additionally it provides traits for annotating types that has a corresponding
Objective-C encoding, respectively `Encode` for structs and `RefEncode` for
references (and `EncodeArguments` for function arguments).

These types are exported under the `objc` crate as well, so usually you would
just use that crate.

# Examples

Implementing `Encode` and `RefEncode`:

```rust
use objc_encode::{Encode, Encoding, RefEncode};

#[repr(C)]
struct MyObject {
    a: f32,
    b: bool,
}

unsafe impl Encode for MyObject {
    const ENCODING: Encoding<'static> = Encoding::Struct(
        "MyObject",
        &[f32::ENCODING, bool::ENCODING
    ]);
}

assert_eq!(&MyObject::ENCODING, "{MyObject=fB}");

unsafe impl RefEncode for MyObject {
    const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
}

assert_eq!(&MyObject::ENCODING_REF, "^{MyObject=fB}");
```

An `Encoding` can be compared with an encoding string from the Objective-C
runtime:

```rust
use objc_encode::Encode;
assert!(&i32::ENCODING == "i");
```

`Encoding` implements `Display` as its string representation. This can be
generated conveniently through the `to_string` method:

```rust
use objc_encode::Encode;
assert_eq!(i32::ENCODING.to_string(), "i");
```

See the [`examples`] folder for more complex usage.

# Installation

```toml
[dependencies]
objc-encode = "1.1.0"
```

# License

This project is licensed under the MIT license, see [`../LICENSE.txt`].

Work is in progress to make it dual-licensed under the Apache License
(Version 2.0) as well.

[`examples`]: https://github.com/madsmtm/objc/tree/master/objc_encode/examples
[`../LICENSE.txt`]: https://github.com/madsmtm/objc/blob/master/LICENSE.txt
