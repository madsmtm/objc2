use std::os::raw::c_char;

use crate::{OpaqueData, BOOL};

/// An opaque type that represents a method selector.
///
/// Selectors are immutable.
#[repr(C)]
pub struct objc_selector {
    _priv: [u8; 0],
    _p: OpaqueData,
}

extern "C" {
    pub fn sel_getName(sel: *const objc_selector) -> *const c_char;
    pub fn sel_getUid(name: *const c_char) -> *const objc_selector;
    pub fn sel_isEqual(lhs: *const objc_selector, rhs: *const objc_selector) -> BOOL;
    #[cfg(apple)]
    pub fn sel_isMapped(sel: *const objc_selector) -> BOOL;
    pub fn sel_registerName(name: *const c_char) -> *const objc_selector;
}
