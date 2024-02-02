//! Test the implementation of `BlockFn` not being generic enough.
//!
//! This is a bug, but it is difficult to fix as we have to mark the lifetimes
//! as higher-ranked in the trait implementation, so let's at least track the
//! error message.
use block2::Block;

use objc2::encode::Encode;

fn is_encode<T: Encode>() {}

fn main() {
    is_encode::<&Block<dyn Fn(&i8)>>();
    // is_encode::<&Block<dyn for<'a> Fn() -> &'a i16>>();
    is_encode::<&Block<dyn Fn(&i32) -> &i32>>();
    is_encode::<&Block<dyn for<'a> Fn(&'a i64, &'a i64) -> &'a i64>>();
}
