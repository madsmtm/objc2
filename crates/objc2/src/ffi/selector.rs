use std::os::raw::c_char;

use crate::{ffi::BOOL, runtime::Sel};

extern_c! {
    pub fn sel_getName(sel: Sel) -> *const c_char;
    pub fn sel_isEqual(lhs: Sel, rhs: Sel) -> BOOL;
    pub fn sel_registerName(name: *const c_char) -> Option<Sel>;

    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn sel_getUid(name: *const c_char) -> Option<Sel>;

    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn sel_isMapped(sel: Sel) -> BOOL;
}
