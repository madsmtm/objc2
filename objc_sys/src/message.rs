//! The `objc_msgSend` familiy of functions.
//!
//! Most of these are `cfg`-gated, these configs are semver-stable.
use super::{objc_class, objc_object};

/// Specifies data used when sending messages to superclasses.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
// TODO: Does this belong in this file or in types.rs?
pub struct objc_super {
    /// The object / instance to send a message to.
    pub receiver: *mut objc_object,
    /// The particular superclass of the instance to message.
    ///
    /// Named `class` in GNUStep and in older Objective-C versions.
    pub super_class: *const objc_class,
}

extern "C" {
    pub fn objc_msgSend();
    pub fn objc_msgSendSuper();
    pub fn method_invoke();
    pub fn _objc_msgForward();
    pub fn class_getMethodImplementation();
}

#[cfg(not(target_arch = "aarch64"))] // __arm64__
extern "C" {
    /// Not available on `target_arch = "aarch64"`
    pub fn objc_msgSend_stret();
    /// Not available on `target_arch = "aarch64"`
    pub fn objc_msgSendSuper_stret();
    /// Not available on `target_arch = "aarch64"`
    pub fn method_invoke_stret();
    /// Not available on `target_arch = "aarch64"`
    pub fn _objc_msgForward_stret();
    /// Not available on `target_arch = "aarch64"`
    pub fn class_getMethodImplementation_stret();
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))] // __x86_64__ and __i386__
extern "C" {
    /// Only available on `target_arch = "x86_64"` or `target_arch = "x86"`
    pub fn objc_msgSend_fpret();
}

#[cfg(target_arch = "x86_64")] // __x86_64__
extern "C" {
    /// Only available on `target_arch = "x86_64"`
    pub fn objc_msgSend_fp2ret();
}
