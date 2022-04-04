//! The `objc_msgSend` familiy of functions.
//!
//! Most of these are `cfg`-gated, these configs are semver-stable.
//!
//! TODO: Some of these are only supported on _some_ GNUStep targets!
use crate::{objc_class, objc_object};
#[cfg(any(gnustep, objfw))]
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
    /// Named `class` in older Objective-C versions.
    pub super_class: *const objc_class,
}

extern_c! {
    #[cfg(any(gnustep, objfw))]
    pub fn objc_msg_lookup(receiver: *mut objc_object, sel: *const objc_selector) -> IMP;
    #[cfg(objfw)]
    pub fn objc_msg_lookup_stret(receiver: *mut objc_object, sel: *const objc_selector) -> IMP;
    #[cfg(any(gnustep, objfw))]
    pub fn objc_msg_lookup_super(sup: *const objc_super, sel: *const objc_selector) -> IMP;
    #[cfg(objfw)]
    pub fn objc_msg_lookup_super_stret(sup: *const objc_super, sel: *const objc_selector) -> IMP;
    // #[cfg(gnustep)]
    // objc_msg_lookup_sender
    // objc_msgLookup family available in macOS >= 10.12

    // objc_msgSend_noarg

    #[cfg(not(objfw))]
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

    // Struct return. Not available on __arm64__:

    /// Not available on `target_arch = "aarch64"`
    #[cfg(all(not(objfw), not(target_arch = "aarch64")))]
    pub fn objc_msgSend_stret();
    // objc_msgSend_stret_debug

    /// Not available on `target_arch = "aarch64"`
    #[cfg(all(apple, not(target_arch = "aarch64")))]
    pub fn objc_msgSendSuper_stret();
    // objc_msgSendSuper2_stret
    // objc_msgSendSuper2_stret_debug

    /// Not available on `target_arch = "aarch64"`
    #[cfg(all(apple, not(target_arch = "aarch64")))]
    pub fn method_invoke_stret();
    /// Not available on `target_arch = "aarch64"`
    #[cfg(all(apple, not(target_arch = "aarch64")))]
    pub fn _objc_msgForward_stret();
    /// Not available on `target_arch = "aarch64"`, always available on ObjFW
    #[cfg(any(objfw, not(target_arch = "aarch64")))]
    pub fn class_getMethodImplementation_stret();

    // __x86_64__ and __i386__

    /// Only available on `target_arch = "x86_64"` or `target_arch = "x86"`
    #[cfg(all(not(objfw), any(target_arch = "x86_64", target_arch = "x86")))]
    pub fn objc_msgSend_fpret();
    // objc_msgSend_fpret_debug

    // __x86_64__

    /// Only available on `target_arch = "x86_64"`
    #[cfg(all(apple, target_arch = "x86_64"))]
    pub fn objc_msgSend_fp2ret();
    // objc_msgSend_fp2ret_debug
}
