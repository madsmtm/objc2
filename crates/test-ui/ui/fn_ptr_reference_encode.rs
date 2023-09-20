//! Test that `Encode` is not implemented for function pointers that are
//! higher-ranked over lifetimes.
//!
//! Ideally, it should be, but it can't right now.
use objc2::encode::Encode;

extern "C" fn my_fn(_x: &i32) {}

fn impls_encode<T: Encode>(_x: T) {}

fn main() {
    // Works
    impls_encode(my_fn as extern "C" fn(_));
    // Can't be written:
    // let encoding = <extern "C" fn(_) as Encode>::ENCODING;

    // Fails
    impls_encode(my_fn as extern "C" fn(&i32));
    let _encoding = <extern "C" fn(&i32) as Encode>::ENCODING;
}
