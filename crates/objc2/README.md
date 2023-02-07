# `objc2`

[![Latest version](https://badgen.net/crates/v/objc2)](https://crates.io/crates/objc2)
[![License](https://badgen.net/badge/license/MIT/blue)](https://github.com/madsmtm/objc2/blob/master/LICENSE.txt)
[![Documentation](https://docs.rs/objc2/badge.svg)](https://docs.rs/objc2/)
[![CI](https://github.com/madsmtm/objc2/actions/workflows/ci.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/ci.yml)

Objective-C interface and runtime bindings in Rust.

Most of the core libraries and frameworks that are in use on Apple systems are
written in Objective-C; this crate enables you to interract with those.

This crate is part of the [`objc2` project](https://github.com/madsmtm/objc2),
see that for related crates, or see [the docs](https://docs.rs/objc2/) for a
more thorough overview.


## Example

```rust
use objc2::ffi::NSUInteger;
use objc2::rc::Id;
use objc2::runtime::NSObject;
use objc2::{msg_send, msg_send_id, ClassType};

let obj: Id<NSObject> = unsafe { msg_send_id![NSObject::class(), new] };

let hash: NSUInteger = unsafe { msg_send![&obj, hash] };
println!("NSObject hash: {}", hash);
```

More examples are [available in the repository][examples].

[examples]: https://github.com/madsmtm/objc2/tree/master/crates/objc2/examples
