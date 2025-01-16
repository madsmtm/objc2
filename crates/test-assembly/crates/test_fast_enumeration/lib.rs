//! Test that fast enumeration is handled efficiently.
use core::hint::black_box;

use objc2_foundation::array::IterUnchecked;
use objc2_foundation::{NSArray, NSObject};

// Should ideally be a fast zero-initialization.
#[no_mangle]
fn iter_create(array: &NSArray<NSObject>) -> IterUnchecked<'_, NSObject> {
    unsafe { array.iter_unchecked() }
}

#[no_mangle]
fn iter_once<'a>(iter: &mut IterUnchecked<'a, NSObject>) -> Option<&'a NSObject> {
    iter.next()
}

#[inline(never)]
#[no_mangle]
fn use_obj(obj: &NSObject) {
    black_box(obj);
}

#[no_mangle]
fn iter(array: &NSArray<NSObject>) {
    for obj in unsafe { array.iter_unchecked() } {
        use_obj(obj);
    }
}

#[no_mangle]
fn iter_noop(array: &NSArray<NSObject>) {
    for _ in unsafe { array.iter_unchecked() } {}
}

#[no_mangle]
fn iter_retained(array: &NSArray<NSObject>) {
    for obj in array {
        use_obj(&obj);
    }
}
