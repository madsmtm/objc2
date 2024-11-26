//! Test how `MethodImplementation` is implemented regarding lifetimes in
//! function pointers and whether they're higher-ranked over them.
//!
//! Ideally it should work for all of these, but it can't right now.
//!
//! (`_` can be used to work around this, by letting the compiler choose an
//! appropriate lifetime '0 that the trait is implemented for).
use objc2::runtime::{ClassBuilder, NSObject, Sel};
use objc2::{class, sel};

extern "C" fn my_fn(_this: &NSObject, _cmd: Sel, _x: &NSObject) {}

fn main() {
    let mut builder = ClassBuilder::new(c"SomeTestClass", class!(NSObject)).unwrap();
    unsafe {
        // Works
        builder.add_method(sel!(none:), my_fn as extern "C" fn(_, _, _));

        // Fails
        builder.add_method(sel!(first:), my_fn as extern "C" fn(&NSObject, _, _));
        builder.add_method(sel!(third:), my_fn as extern "C" fn(_, _, &NSObject));
        builder.add_method(sel!(both:), my_fn as extern "C" fn(&NSObject, _, &NSObject));
    }
}
