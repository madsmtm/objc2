use core::ptr;
use std::os::raw::c_char;

use crate::{objc_class, objc_selector, BOOL, IMP};

#[no_mangle]
pub extern "C" fn objc_getClass(name: *const c_char) -> *const objc_class {
    ptr::null()
}

#[no_mangle]
pub extern "C" fn objc_allocateClassPair(
    superclass: *const objc_class,
    name: *const c_char,
    extra_bytes: usize,
) -> *mut objc_class {
    ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn objc_disposeClassPair(cls: *mut objc_class) {}

#[no_mangle]
pub extern "C" fn objc_registerClassPair(cls: *mut objc_class) {}

#[no_mangle]
pub fn class_addIvar(
    cls: *mut objc_class,
    name: *const c_char,
    size: usize,
    alignment: u8,
    types: *const c_char,
) -> BOOL {
    false as BOOL
}

#[no_mangle]
pub fn class_addMethod(
    cls: *mut objc_class,
    name: *const objc_selector,
    imp: IMP,
    types: *const c_char,
) -> BOOL {
    false as BOOL
}
