//! ARC functions.
//!
//! All available since macOS `10.7`.
//!
//! Defined in `objc-internal.h`, but is available in Clang's
//! [documentation][ARC] so these are safe to rely on.
//!
//! On GNUStep these are defined in `objc-arc.h`.
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
    pub fn objc_destroyWeak(addr: *mut *mut objc_object);
    pub fn objc_initWeak(addr: *mut *mut objc_object, value: *mut objc_object) -> *mut objc_object;
    // Defined in runtime.h
    pub fn objc_loadWeak(addr: *mut *mut objc_object) -> *mut objc_object;
    pub fn objc_loadWeakRetained(addr: *mut *mut objc_object) -> *mut objc_object;
    pub fn objc_moveWeak(to: *mut *mut objc_object, from: *mut *mut objc_object);

    // Retain / release

    pub fn objc_release(value: *mut objc_object);
    pub fn objc_retain(value: *mut objc_object) -> *mut objc_object;
    pub fn objc_retainAutorelease(value: *mut objc_object) -> *mut objc_object;
    pub fn objc_retainAutoreleaseReturnValue(value: *mut objc_object) -> *mut objc_object;
    pub fn objc_retainAutoreleasedReturnValue(value: *mut objc_object) -> *mut objc_object;
    // Defined in objc-abi.h
    pub fn objc_retainBlock(value: *mut objc_object) -> *mut objc_object;

    // Storing values

    pub fn objc_storeStrong(addr: *mut *mut objc_object, value: *mut objc_object);
    // Defined in runtime.h
    pub fn objc_storeWeak(addr: *mut *mut objc_object, value: *mut objc_object)
        -> *mut objc_object;

    // TODO: Decide about nonstandard extensions like these:
    // #[cfg(any(gnustep, winobjc))]
    // pub fn objc_delete_weak_refs(obj: *mut objc_object) -> BOOL;
}
