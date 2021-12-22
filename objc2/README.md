# `objc2`

[![Latest version](https://badgen.net/crates/v/objc2)](https://crates.io/crates/objc2)
[![License](https://badgen.net/badge/license/MIT/blue)](../LICENSE.txt)
[![Documentation](https://docs.rs/objc2/badge.svg)](https://docs.rs/objc2/)
[![Apple CI](https://github.com/madsmtm/objc2/actions/workflows/apple.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/apple.yml)
[![GNUStep CI](https://github.com/madsmtm/objc2/actions/workflows/gnustep.yml/badge.svg)](https://github.com/madsmtm/objc2/actions/workflows/gnustep.yml)

Objective-C runtime bindings and interface for Rust.

## Messaging objects

Objective-C objects can be messaged using the `msg_send!` macro:

```rust , no_run
use objc2::{class, msg_send};
use objc2::runtime::{Bool, Object};

let cls = class!(NSObject);
unsafe {
    let obj: *mut Object = msg_send![cls, new];
    let hash: usize = msg_send![obj, hash];
    let is_kind: Bool = msg_send![obj, isKindOfClass: cls];
    // Even void methods must have their return type annotated
    let _: () = msg_send![obj, release];
}
```

## Reference counting

The utilities of the `rc` module provide ARC-like semantics for working with
Objective-C's reference counted objects in Rust.

An `Id` retains an object and releases the object when dropped.
A `WeakId` will not retain the object, but can be upgraded to an `Id` and
safely fails if the object has been deallocated.

```rust , no_run
use objc2::{class, msg_send};
use objc2::rc::{autoreleasepool, Id, Shared, WeakId};
use objc2::runtime::Object;

// Id will release the object when dropped
let obj: Id<Object, Shared> = unsafe {
    Id::new(msg_send![class!(NSObject), new])
};

// Cloning retains the object an additional time
let cloned = obj.clone();
autoreleasepool(|pool| {
    // Autorelease consumes the Id, but won't
    // actually release until the end of an autoreleasepool
    let obj_ref: &Object = cloned.autorelease(pool);
});

// Weak references won't retain the object
let weak = WeakId::new(&obj);
drop(obj);
assert!(weak.load().is_none());
```

## Declaring classes

Classes can be declared using the `ClassDecl` struct. Instance variables and
methods can then be added before the class is ultimately registered.

The following example demonstrates declaring a class named `MyNumber` that has
one ivar, a `u32` named `_number` and a `number` method that returns it:

```rust , no_run
use objc2::{class, sel};
use objc2::declare::ClassDecl;
use objc2::runtime::{Object, Sel};

let superclass = class!(NSObject);
let mut decl = ClassDecl::new("MyNumber", superclass).unwrap();

// Add an instance variable
decl.add_ivar::<u32>("_number");

// Add an ObjC method for getting the number
extern fn my_number_get(this: &Object, _cmd: Sel) -> u32 {
    unsafe { *this.ivar("_number") }
}
unsafe {
    decl.add_method(sel!(number),
        my_number_get as extern fn(&Object, Sel) -> u32);
}

decl.register();
```

## Exceptions

By default, if the `msg_send!` macro causes an exception to be thrown, this
will unwind into Rust resulting in unsafe, undefined behavior.
However, this crate has an `"catch_all"` feature which, when enabled, wraps
each `msg_send!` in a `@try`/`@catch` and panics if an exception is caught,
preventing Objective-C from unwinding into Rust.

## Message type verification

The Objective-C runtime includes encodings for each method that describe the
argument and return types. This crate can take advantage of these encodings to
verify that the types used in Rust match the types encoded for the method.

To use this functionality, enable the `"verify_message"` feature.
With this feature enabled, type checking is performed for every message send,
which also requires that all arguments and return values for all messages
implement `Encode`.

If this requirement is burdensome or you'd rather just verify specific messages,
you can call the `Message::verify_message` method for specific selectors.

## Support for other Operating Systems

The bindings can be used on Linux or *BSD utilizing the
[GNUstep Objective-C runtime](https://www.github.com/gnustep/libobjc2).
