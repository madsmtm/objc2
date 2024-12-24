//! Smart pointer definitions used by libdispatch.

use core::{fmt, ops::Deref, ptr::NonNull};

use super::ffi::*;

/// Smart pointer based on libdispatch reference counting system.
#[repr(transparent)]
pub(crate) struct Retained<T: ?Sized> {
    ptr: NonNull<T>,
}

impl<T> Retained<T> {
    /// Create new smart pointer assuming the ownership over the object.
    /// The retain count will stay the same.
    pub(crate) unsafe fn from_raw(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    /// Create new smart pointer with shared ownership.
    /// Increments reference counter by 1.
    #[allow(unused)]
    pub(crate) unsafe fn retain(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| {
            // Safety: upheld by the caller
            unsafe { dispatch_retain(ptr.as_ptr().cast()) };
            Self { ptr }
        })
    }

    #[inline]
    pub(crate) fn as_ptr(this: &Self) -> *const T {
        this.ptr.as_ptr()
    }
}

impl<T: ?Sized> Drop for Retained<T> {
    fn drop(&mut self) {
        // Safety: the pointer must be valid.
        unsafe { dispatch_release(self.ptr.as_ptr().cast()) };
    }
}

impl<T: ?Sized> Clone for Retained<T> {
    /// Retain the object, increasing its reference count.
    #[inline]
    fn clone(&self) -> Self {
        // Safety: upheld by the caller.
        unsafe { dispatch_retain(self.ptr.as_ptr().cast()) };
        Self { ptr: self.ptr }
    }
}

impl<T: ?Sized> fmt::Pointer for Retained<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr.as_ptr(), f)
    }
}

impl<T: ?Sized> Deref for Retained<T> {
    type Target = T;

    /// Obtain an immutable reference to the object.
    // Box doesn't inline, but that's because it's a compiler built-in
    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: The pointer's validity is verified when the type is
        // created.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> fmt::Debug for Retained<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ptr.as_ptr().fmt(f)
    }
}

// Safety: inherently safe to move between threads.
unsafe impl<T> Send for Retained<T> {}

#[cfg(feature = "objc2")]
impl<T> From<Retained<T>> for objc2::rc::Retained<T>
where
    T: objc2::Message,
{
    fn from(value: Retained<T>) -> Self {
        // Safety: upheld by the caller
        unsafe {
            objc2::rc::Retained::retain(Retained::as_ptr(&value).cast_mut()).expect("cannot be nil")
        }
    }
}
