use core::ffi::c_void;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::ffi;
use crate::runtime::{Class, Sel};

/// Allows storing a [`Sel`] in a static and lazily loading it.
#[doc(hidden)]
pub struct CachedSel {
    ptr: AtomicPtr<c_void>,
}

impl CachedSel {
    /// Constructs a new [`CachedSel`].
    pub const fn new() -> CachedSel {
        CachedSel {
            ptr: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Returns the cached selector. If no selector is yet cached, registers
    /// one with the given name and stores it.
    #[inline]
    #[doc(hidden)]
    pub unsafe fn get(&self, name: &str) -> Sel {
        // `Relaxed` should be fine since `sel_registerName` is thread-safe.
        let ptr = self.ptr.load(Ordering::Relaxed);
        if ptr.is_null() {
            let ptr: *const c_void = unsafe { ffi::sel_registerName(name.as_ptr().cast()).cast() };
            self.ptr.store(ptr as *mut c_void, Ordering::Relaxed);
            unsafe { Sel::from_ptr(ptr) }
        } else {
            unsafe { Sel::from_ptr(ptr) }
        }
    }
}

/// Allows storing a [`Class`] reference in a static and lazily loading it.
#[doc(hidden)]
pub struct CachedClass {
    ptr: AtomicPtr<Class>,
}

impl CachedClass {
    /// Constructs a new [`CachedClass`].
    pub const fn new() -> CachedClass {
        CachedClass {
            ptr: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Returns the cached class. If no class is yet cached, gets one with
    /// the given name and stores it.
    #[inline]
    #[doc(hidden)]
    pub unsafe fn get(&self, name: &str) -> Option<&'static Class> {
        // `Relaxed` should be fine since `objc_getClass` is thread-safe.
        let ptr = self.ptr.load(Ordering::Relaxed);
        if let Some(cls) = unsafe { ptr.as_ref() } {
            Some(cls)
        } else {
            let ptr: *const Class = unsafe { ffi::objc_getClass(name.as_ptr().cast()) }.cast();
            self.ptr.store(ptr as *mut Class, Ordering::Relaxed);
            unsafe { ptr.as_ref() }
        }
    }
}
