//! Test that fast enumeration is handled efficiently.
use core::hint::black_box;

use objc2_foundation::array::IterUnchecked;
use objc2_foundation::{NSArray, NSObject};

// Should ideally be a fast zero-initialization.
#[export_name = "fn1_iter_create"]
fn iter_create(array: &NSArray<NSObject>) -> IterUnchecked<'_, NSObject> {
    unsafe { array.iter_unchecked() }
}

#[export_name = "fn2_iter_once"]
fn iter_once<'a>(iter: &mut IterUnchecked<'a, NSObject>) -> Option<&'a NSObject> {
    iter.next()
}

#[inline(never)]
#[export_name = "fn3_use_obj"]
fn use_obj(obj: &NSObject) {
    black_box(obj);
}

#[export_name = "fn4_iter"]
fn iter(array: &NSArray<NSObject>) {
    for obj in unsafe { array.iter_unchecked() } {
        use_obj(obj);
    }
}

#[export_name = "fn5_iter_noop"]
fn iter_noop(array: &NSArray<NSObject>) {
    for _ in unsafe { array.iter_unchecked() } {}
}

#[export_name = "fn6_iter_retained"]
fn iter_retained(array: &NSArray<NSObject>) {
    for obj in array {
        use_obj(&obj);
    }
}
