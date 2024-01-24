//! Verify that certain things we don't want to be encode aren't.
use core::cell::{Cell, UnsafeCell};
use core::ffi::c_void;

use block2::Block;
use objc2::encode::Encode;
use objc2::runtime::Sel;

fn is_encode<T: Encode>() {}

fn main() {
    is_encode::<Vec<u32>>();

    is_encode::<()>();
    is_encode::<&()>();
    is_encode::<*const ()>();
    is_encode::<c_void>();

    is_encode::<&Block<dyn Fn((), i32)>>();
    is_encode::<&Block<dyn Fn() -> bool>>();

    is_encode::<fn() -> &'static ()>();
    is_encode::<fn(())>();
    is_encode::<fn(i32, ())>();

    is_encode::<&Sel>();

    // This should compile
    is_encode::<UnsafeCell<&u8>>();
    // But this mustn't
    is_encode::<Option<UnsafeCell<&u8>>>();

    // Same
    is_encode::<Option<Cell<&u8>>>();
}
