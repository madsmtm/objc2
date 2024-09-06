#![allow(dead_code)]
use core::ptr::NonNull;

use objc2::rc::Retained;

pub(crate) fn retained_ptr_cast<T: ?Sized>(objects: *mut Retained<T>) -> *mut NonNull<T> {
    // SAFETY: `Retained<T>` has the same memory layout as `NonNull<T>`, and
    // stronger guarantees.
    objects.cast()
}

pub(crate) fn ref_ptr_cast_const<T: ?Sized>(objects: *const &T) -> *mut NonNull<T> {
    // SAFETY: `&T` has the same memory layout as `NonNull<T>`, and stronger
    // guarantees.
    (objects as *mut &T).cast()
}

pub(crate) fn retained_ptr_cast_const<T: ?Sized>(objects: *const Retained<T>) -> *mut NonNull<T> {
    retained_ptr_cast(objects as *mut Retained<T>)
}
