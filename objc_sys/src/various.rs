use core::ffi::c_void;
use std::os::raw::{c_char, c_int, c_uint};

use crate::{objc_AssociationPolicy, objc_ivar, objc_object, BOOL, IMP};

extern "C" {
    pub fn imp_getBlock(anImp: IMP) -> *mut objc_object;
    pub fn imp_implementationWithBlock(block: *mut objc_object) -> IMP;
    pub fn imp_removeBlock(anImp: IMP) -> BOOL;

    pub fn ivar_getName(v: *const objc_ivar) -> *const c_char;
    pub fn ivar_getOffset(v: *const objc_ivar) -> isize;
    pub fn ivar_getTypeEncoding(v: *const objc_ivar) -> *const c_char;

    pub fn objc_copyClassNamesForImage(
        image: *const c_char,
        outCount: *mut c_uint,
    ) -> *mut *const c_char;
    pub fn objc_copyImageNames(outCount: *mut c_uint) -> *mut *const c_char;

    pub fn objc_enumerationMutation(obj: *mut objc_object);

    pub fn objc_getAssociatedObject(
        object: *mut objc_object,
        key: *const c_void,
    ) -> *mut objc_object;
    pub fn objc_setAssociatedObject(
        object: *mut objc_object,
        key: *const c_void,
        value: *mut objc_object,
        policy: objc_AssociationPolicy,
    );
    pub fn objc_removeAssociatedObjects(object: *mut objc_object);

    pub fn objc_setForwardHandler(fwd: *mut c_void, fwd_stret: *mut c_void);
    pub fn objc_sync_enter(obj: *mut objc_object) -> c_int;
    pub fn objc_sync_exit(obj: *mut objc_object) -> c_int;
}
