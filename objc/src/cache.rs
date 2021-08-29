use std::os::raw::c_void;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

use crate::runtime::{self, Class, Sel};

/// Allows storing a `Sel` in a static and lazily loading it.
#[doc(hidden)]
pub struct CachedSel {
    ptr: AtomicPtr<c_void>,
}

impl CachedSel {
    /// Constructs a new `CachedSel`.
    pub const fn new() -> CachedSel {
        CachedSel {
            ptr: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Returns the cached selector. If no selector is yet cached, registers
    /// one with the given name and stores it.
    #[inline(always)]
    pub unsafe fn get(&self, name: &str) -> Sel {
        let ptr = self.ptr.load(Ordering::Relaxed);
        // It should be fine to use `Relaxed` ordering here because `sel_registerName` is
        // thread-safe.
        if ptr.is_null() {
            let sel = runtime::sel_registerName(name.as_ptr() as *const _);
            self.ptr.store(sel.as_ptr() as *mut _, Ordering::Relaxed);
            sel
        } else {
            Sel::from_ptr(ptr)
        }
    }
}

/// Allows storing a `Class` reference in a static and lazily loading it.
#[doc(hidden)]
pub struct CachedClass {
    ptr: AtomicPtr<Class>,
}

impl CachedClass {
    /// Constructs a new `CachedClass`.
    pub const fn new() -> CachedClass {
        CachedClass {
            ptr: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Returns the cached class. If no class is yet cached, gets one with
    /// the given name and stores it.
    #[inline(always)]
    pub unsafe fn get(&self, name: &str) -> Option<&'static Class> {
        // `Relaxed` should be fine since `objc_getClass` is thread-safe.
        let ptr = self.ptr.load(Ordering::Relaxed);
        if ptr.is_null() {
            let cls = runtime::objc_getClass(name.as_ptr() as *const _);
            self.ptr.store(cls as *mut _, Ordering::Relaxed);
            cls.as_ref()
        } else {
            Some(&*ptr)
        }
    }
}
