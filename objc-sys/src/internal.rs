//! Defined in `objc-internal.h`
#[allow(unused)]
use crate::{objc_class, objc_object};
use objc2_proc_macros::cfg_available;

extern_c_unwind! {
    #[cfg(apple)]
    #[cfg_available(macOS(10.9), iOS(7.0), tvOS(9.0), watchOS(1.0))]
    pub fn objc_alloc(cls: *const objc_class) -> *mut objc_object;

    #[cfg(apple)]
    #[cfg_available(macOS(10.14.4), iOS(12.2), tvOS(12.2), watchOS(5.2))]
    pub fn objc_alloc_init(cls: *const objc_class) -> *mut objc_object;

    #[cfg(apple)]
    #[cfg_available(macOS(10.15), iOS(13.0), tvOS(13.0), watchOS(6.0))]
    pub fn objc_opt_new(cls: *const objc_class) -> *mut objc_object;
}
