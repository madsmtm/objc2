//! Test the implementation of `BlockSignature` not being generic enough.
//!
//! This is a bug, but it is difficult to fix as we have to mark the lifetimes
//! as higher-ranked in the trait implementation, so let's at least track the
//! error message.
use block2::Block;

use objc2::encode::Encode;

fn is_encode<T: Encode>() {}

fn main() {
    is_encode::<&Block<'_, fn(&i8)>>();
    // is_encode::<&Block<for<'a> fn() -> &'a i16>>(); // Unconstrained
    is_encode::<&Block<'_, fn(&i32) -> &i32>>();
    is_encode::<&Block<'_, for<'a> fn(&'a i64, &'a i64) -> &'a i64>>();
}
