//! Test that `Encode` is not implemented for function pointers that are
//! higher-ranked over lifetimes.
//!
//! Ideally, they should be, but they can't be right now.
//!
//! (Also test that we can use `_` to work around this).
use objc2::Encode;

extern "C" fn my_fn(_x: &i32) {}

fn e<T: Encode>(_x: T) {}

fn main() {
    // Works
    e(my_fn as extern "C" fn(_));
    // Can't be written:
    // let encoding = <extern "C" fn(_) as Encode>::ENCODING;

    // Fails
    e(my_fn as extern "C" fn(&i32));
    // Also fails, properly tested in `fn_ptr_reference_encode2`
    let encoding = <extern "C" fn(&i32) as Encode>::ENCODING;
}
