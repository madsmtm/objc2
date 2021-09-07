//! The `objc_msgSend` familiy of functions.
//!
//! Most of these are `cfg`-gated, these configs are semver-stable.
//!
//! TODO: Some of these are only supported on _some_ GNUStep targets!
use crate::{objc_class, objc_object};
#[cfg(gnustep)]
use crate::{objc_selector, IMP};

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

#[cfg(gnustep)]
extern "C" {
    pub fn objc_msg_lookup(receiver: *mut objc_object, sel: *const objc_selector) -> IMP;
    pub fn objc_msg_lookup_super(sup: *const objc_super, sel: *const objc_selector) -> IMP;
    // objc_msg_lookup_sender

    // objc_msgLookup family available in macOS >= 10.12
}

extern "C" {
    // objc_msgSend_noarg

    pub fn objc_msgSend();
    // objc_msgSend_debug

    #[cfg(apple)]
    pub fn objc_msgSendSuper();
    // objc_msgSendSuper2
    // objc_msgSendSuper2_debug

    #[cfg(apple)]
    pub fn method_invoke();
    #[cfg(apple)]
    pub fn _objc_msgForward();
    pub fn class_getMethodImplementation();
}

#[cfg(not(target_arch = "aarch64"))] // __arm64__
extern "C" {
    /// Not available on `target_arch = "aarch64"`
    pub fn objc_msgSend_stret();
    // objc_msgSend_stret_debug

    /// Not available on `target_arch = "aarch64"`
    #[cfg(apple)]
    pub fn objc_msgSendSuper_stret();
    // objc_msgSendSuper2_stret
    // objc_msgSendSuper2_stret_debug

    /// Not available on `target_arch = "aarch64"`
    #[cfg(apple)]
    pub fn method_invoke_stret();
    /// Not available on `target_arch = "aarch64"`
    #[cfg(apple)]
    pub fn _objc_msgForward_stret();
    /// Not available on `target_arch = "aarch64"`
    pub fn class_getMethodImplementation_stret();
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))] // __x86_64__ and __i386__
extern "C" {
    /// Only available on `target_arch = "x86_64"` or `target_arch = "x86"`
    pub fn objc_msgSend_fpret();
    // objc_msgSend_fpret_debug
}

#[cfg(target_arch = "x86_64")] // __x86_64__
extern "C" {
    /// Only available on `target_arch = "x86_64"`
    #[cfg(apple)]
    pub fn objc_msgSend_fp2ret();
    // objc_msgSend_fp2ret_debug
}
