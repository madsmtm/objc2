# `objc2`

[![Latest version](https://badgen.net/crates/v/objc2)](https://crates.io/crates/objc2)
[![License](https://badgen.net/badge/license/MIT/blue)](../LICENSE.txt)
[![Documentation](https://docs.rs/objc2/badge.svg)](https://docs.rs/objc2/)
[![CI](https://github.com/madsmtm/objc2/actions/workflows/ci.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/ci.yml)

Objective-C interface and runtime bindings for Rust.

Most of the core libraries and frameworks that are in use on Apple systems are
written in Objective-C; this crate enables you to interract with those.

## Example

```rust
use std::ptr::NonNull;
use objc2::{class, msg_send};
use objc2::rc::{Id, Owned};
use objc2::runtime::{Class, Object, ObjectType};

let cls = class!(NSObject);
let obj: *mut Object = unsafe { msg_send![cls, new] };
let obj: Id<Object, Owned> = unsafe { Id::new(NonNull::new(obj).unwrap()) };

// TODO
// let isa = unsafe { obj.ivar::<Class>("isa") };
// assert_eq!(cls, isa);
```

See [the docs](https://docs.rs/objc2/) for a more thorough overview, or jump
right into the [examples].

This crate is part of the [`objc2` project](https://github.com/madsmtm/objc2),
see that for related crates.

[examples]: https://github.com/madsmtm/objc2/tree/master/objc2/examples
