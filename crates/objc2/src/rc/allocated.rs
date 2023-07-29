use core::fmt;
use core::marker::PhantomData;
use core::mem::{self, ManuallyDrop};
use core::ptr::NonNull;

use crate::runtime::objc_release_fast;
use crate::Message;

/// A marker type that can be used to indicate that the object has been
/// allocated but not initialized.
///
/// The reason we use `Allocated<T>` / `Option<Allocated<T>>` instead of
/// `*mut T` is:
/// - To allow releasing allocated objects, e.g. in the face of panics.
/// - To safely know the object is valid (albeit uninitialized).
//
// Note that there is no way to get something mutable out of `Allocated`
// unless you're defining the object.
//
// For example, `+[NSMutableString alloc]` is allowed to return a non-unique
// object as an optimization, and then only figure out afterwards whether it
// needs to allocate, or if it can store an `NSString` internally.
// Similarly, while e.g. `+[NSData alloc]` may return a unique object,
// calling `-[NSData init]` afterwards could return a shared empty `NSData`
// instance.
#[repr(transparent)]
#[derive(Debug)]
pub struct Allocated<T: ?Sized> {
    /// The yet-to-be initialized object.
    ///
    /// We don't use `Id` here, since that has different auto-trait impls, and
    /// requires in it's safety contract that the object is initialized (which
    /// makes it difficult to ensure correctness if such things are split
    /// across different files).
    ///
    /// Variance is same as `Id`.
    ptr: NonNull<T>,
    /// Necessary for dropck, as with `Id`.
    p: PhantomData<T>,
}

// We _could_ probably implement auto traits `Send` and `Sync` here, but to be
// safe, we won't.

// Explicitly don't implement `Deref`, `Message` nor `RefEncode`!

impl<T: ?Sized + Message> Allocated<T> {
    /// # Safety
    ///
    /// The caller must ensure the given object has +1 retain count, and that
    /// the object behind the pointer has been allocated (but not yet
    /// initialized).
    #[inline]
    pub(crate) unsafe fn new(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self {
            ptr,
            p: PhantomData,
        })
    }

    #[inline]
    pub(crate) fn option_into_ptr(this: Option<Self>) -> *mut T {
        let ptr = this.map(|this| ManuallyDrop::new(this).ptr);

        // Difficult to write this in an ergonomic way with `?Sized`, so we
        // hack it with transmute.
        //
        // SAFETY: `Option<NonNull<T>>` has the same size as `*mut T`.
        unsafe { mem::transmute::<Option<NonNull<T>>, *mut T>(ptr) }
    }
}

impl<T: ?Sized> Drop for Allocated<T> {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: Allocated objects can always safely be released, since
        // destructors are written to take into account that the object may
        // not have been initialized.
        //
        // Rest is same as `Id`'s `Drop`.
        unsafe { objc_release_fast(self.ptr.as_ptr().cast()) };
    }
}

impl<T: ?Sized> fmt::Pointer for Allocated<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr.as_ptr(), f)
    }
}
