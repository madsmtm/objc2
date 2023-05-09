# `icrate`

[![Latest version](https://badgen.net/crates/v/icrate)](https://crates.io/crates/icrate)
[![License](https://badgen.net/badge/license/MIT/blue)](../LICENSE.txt)
[![Documentation](https://docs.rs/icrate/badge.svg)](https://docs.rs/icrate/)
[![CI](https://github.com/madsmtm/objc2/actions/workflows/ci.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/ci.yml)

Rust bindings to Apple's frameworks..

This crate is part of the [`objc2` project](https://github.com/madsmtm/objc2),
see that for related crates, or see [the docs](https://docs.rs/icrate/) for
more details.


## Supported versions

These bindings are automatically generated from the SDKs in Xcode 14.2 (will
be periodically updated).

Currently supports:
- macOS: `10.7-13.1`
- iOS/iPadOS: `7.0-16.2` (WIP)
- tvOS: `9.0-16.1` (WIP)
- watchOS: `1.0-9.1` (WIP)


## Example

```rust
use icrate::Foundation::{NSCopying, NSArray};
use icrate::ns_string;

let string = ns_string!("world");
println!("hello {string}");

let array = NSArray::from_id_slice(&[string.copy()]);
println!("{array:?}");
```

More examples are [available in the repository][examples].

[examples]: https://github.com/madsmtm/objc2/tree/master/crates/icrate/examples
