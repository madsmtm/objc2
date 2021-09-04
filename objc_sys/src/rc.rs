//! ARC functions.
//!
//! All available since macOS `10.7`.
//!
//! Defined in `objc-internal.h`, but is available in Clang's
//! [documentation][ARC] so these are safe to rely on.
//!
//! [ARC]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html#runtime-support>
use core::ffi::c_void;

use crate::objc_object;

extern "C" {
    // Autorelease

    pub fn objc_autorelease(value: *mut objc_object) -> *mut objc_object;
    pub fn objc_autoreleasePoolPop(pool: *mut c_void);
    pub fn objc_autoreleasePoolPush() -> *mut c_void;
    pub fn objc_autoreleaseReturnValue(value: *mut objc_object) -> *mut objc_object;

    // Weak pointers

    pub fn objc_copyWeak(to: *mut *mut objc_object, from: *mut *mut objc_object);
    pub fn objc_destroyWeak(location: *mut *mut objc_object);
    pub fn objc_initWeak(
        location: *mut *mut objc_object,
        value: *mut objc_object,
    ) -> *mut objc_object;
    // Defined in runtime.h
    pub fn objc_loadWeak(location: *mut *mut objc_object) -> *mut objc_object;
    pub fn objc_loadWeakRetained(location: *mut *mut objc_object) -> *mut objc_object;
    pub fn objc_moveWeak(to: *mut *mut objc_object, from: *mut *mut objc_object);

    // Retain / release

    pub fn objc_release(value: *mut objc_object);
    pub fn objc_retain(value: *mut objc_object) -> *mut objc_object;
    pub fn objc_retainAutorelease(value: *mut objc_object) -> *mut objc_object;
    pub fn objc_retainAutoreleasedReturnValue(value: *mut objc_object) -> *mut objc_object;
    // Defined in objc-abi.h
    pub fn objc_retainBlock(value: *mut objc_object) -> *mut objc_object;

    // Storing values

    pub fn objc_storeStrong(location: *mut *mut objc_object, value: *mut objc_object);
    // Defined in runtime.h
    pub fn objc_storeWeak(
        location: *mut *mut objc_object,
        value: *mut objc_object,
    ) -> *mut objc_object;
}
