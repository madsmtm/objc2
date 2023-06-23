//! Common selectors.
//!
//! These are put here to deduplicate the cached selector, and when using
//! `unstable-static-sel`, the statics.
//!
//! Note that our assembly tests of `unstable-static-sel-inlined` output a GOT
//! entry for such accesses, but that is just a limitation of our tests - the
//! actual assembly is as one would expect.
use crate::runtime::Sel;
use crate::{__sel_data, __sel_inner};

#[inline]
pub fn alloc_sel() -> Sel {
    __sel_inner!(__sel_data!(alloc), "alloc")
}

#[inline]
pub fn init_sel() -> Sel {
    __sel_inner!(__sel_data!(init), "init")
}

#[inline]
pub fn new_sel() -> Sel {
    __sel_inner!(__sel_data!(new), "new")
}

#[inline]
pub fn dealloc_sel() -> Sel {
    __sel_inner!(__sel_data!(dealloc), "dealloc")
}
