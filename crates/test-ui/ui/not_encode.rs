//! Verify that certain things we don't want to be encode aren't.
use core::ffi::c_void;

use objc2::encode::Encode;
use objc2::runtime::Sel;

fn is_encode<T: Encode>() {}

fn main() {
    is_encode::<Vec<u32>>();

    is_encode::<()>(); // TODO: Make this fail as well
    is_encode::<&()>();
    is_encode::<*const ()>();
    is_encode::<c_void>();
    is_encode::<&c_void>();

    is_encode::<fn() -> &'static ()>();

    is_encode::<&Sel>();
}
