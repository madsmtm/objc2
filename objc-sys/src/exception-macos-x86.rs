//! `objc-exception.h` is conditional on __OBJC2__, which is set for all
//! platforms except 32-bit macOS; these are the functions that are defined
//! when !__OBJC2__.
use core::ffi::c_void;

use crate::objc_object;

// objc_exception_functions_t

extern_c! {
    pub fn objc_exception_throw(exception: *mut objc_object) -> !;
    pub fn objc_exception_try_enter(exception_data: *const c_void);
    pub fn objc_exception_try_exit(exception_data: *const c_void);
    // objc_exception_extract
    // objc_exception_match
    // objc_exception_get_functions
    // objc_exception_set_functions
}
