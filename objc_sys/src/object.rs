use core::ffi::c_void;
use std::os::raw::c_char;

use crate::{objc_class, objc_ivar, OpaqueData};

/// An opaque type that represents an instance of a class.
#[repr(C)]
pub struct objc_object {
    // `isa` field is deprecated, so we don't expose it here.
    // Use `object_getClass` instead.
    _priv: [u8; 0],
    _p: OpaqueData,
}

extern "C" {
    pub fn object_getClass(obj: *mut objc_object) -> *mut objc_class;
    pub fn object_getClassName(obj: *mut objc_object) -> *const c_char;
    pub fn object_getIndexedIvars(obj: *mut objc_object) -> *mut c_void;
    pub fn object_getIvar(obj: *mut objc_object, ivar: *const objc_ivar) -> *mut objc_object;
    pub fn object_setClass(obj: *mut objc_object, cls: *mut objc_class) -> *mut objc_class;
    pub fn object_setIvar(obj: *mut objc_object, ivar: *const objc_ivar, value: *mut objc_object);
}
