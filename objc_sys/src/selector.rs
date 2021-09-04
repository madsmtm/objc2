use std::os::raw::c_char;

use crate::{objc_selector, BOOL};

extern "C" {
    pub fn sel_getName(sel: *const objc_selector) -> *const c_char;
    pub fn sel_getUid(str_: *const c_char) -> *const objc_selector;
    pub fn sel_isEqual(lhs: *const objc_selector, rhs: *const objc_selector) -> BOOL;
    pub fn sel_isMapped(sel: *const objc_selector) -> BOOL;
    pub fn sel_registerName(str_: *const c_char) -> *const objc_selector;
}
