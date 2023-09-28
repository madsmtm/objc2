use core::fmt;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;

use crate::mutability::IsMutable;
use crate::runtime::{objc_release_fast, AnyObject};
use crate::Message;

/// An Objective-C object that has been allocated, but not initialized.
///
/// Objective-C splits the allocation and initialization steps up into two, so
/// we need to track it in the type-system whether something has been
/// intialized or not.
///
/// Note that allocation in Objective-C can fail, e.g. in Out Of Memory
/// situations! This is handled by `objc2` automatically, but if you really
/// need to, you can check for this explicitly by inspecting the pointer
/// returned from [`as_ptr`].
///
/// Note also that this represents that the _current_ class's instance
/// variables are not yet initialized; but subclass instance variables may
/// have been so.
///
/// See [Apple's documentation on Object Allocation][object-allocation] for a
/// few more details.
///
/// [`as_ptr`]: Self::as_ptr
/// [object-allocation]: https://developer.apple.com/library/archive/documentation/General/Conceptual/CocoaEncyclopedia/ObjectAllocation/ObjectAllocation.html
///
///
/// # Memory layout
///
/// This is guaranteed to have the same size and alignment as a pointer to the
/// object, `*const T`. The pointer may be NULL.
#[repr(transparent)]
#[derive(Debug)]
pub struct Allocated<T: ?Sized> {
    /// The yet-to-be initialized object.
    ///
    /// We don't use `Id` here, since that has different auto-trait impls, and
    /// requires in it's safety contract that the object is initialized (which
    /// makes it difficult to ensure correctness if such things are split
    /// across different files). Additionally, we want to have fine control
    /// over NULL-ness.
    ///
    /// Covariance is correct, same as `Id`.
    ptr: *const T, // Intentially not `NonNull`!
    /// Necessary for dropck, as with `Id`.
    p: PhantomData<T>,
    /// Necessary for restricting auto traits.
    p_auto_traits: PhantomData<AnyObject>,
}

// We _could_ probably implement auto traits `Send` and `Sync` here, but to be
// safe, we won't for now.
//
// Explicitly don't implement `Deref`, `Message` nor `RefEncode`, though!

impl<T: ?Sized + Message> Allocated<T> {
    /// # Safety
    ///
    /// The caller must ensure the given object has +1 retain count, and that
    /// the object behind the pointer has been allocated (but not yet
    /// initialized), or that the pointer is NULL.
    #[inline]
    pub(crate) unsafe fn new(ptr: *mut T) -> Self {
        Self {
            ptr,
            p: PhantomData,
            p_auto_traits: PhantomData,
        }
    }

    /// Returns a raw pointer to the object.
    ///
    /// The pointer is valid for at least as long as the `Allocated` is held.
    ///
    /// See [`Allocated::as_mut_ptr`] for the mutable equivalent.
    ///
    /// This is an associated method, and must be called as
    /// `Allocated::as_ptr(obj)`.
    #[inline]
    pub fn as_ptr(this: &Self) -> *const T {
        this.ptr
    }

    /// Returns a raw mutable pointer to the object.
    ///
    /// The pointer is valid for at least as long as the `Allocated` is held.
    ///
    /// See [`Allocated::as_ptr`] for the immutable equivalent.
    ///
    /// This is an associated method, and must be called as
    /// `Allocated::as_mut_ptr(obj)`.
    ///
    ///
    /// # Note about mutable references
    ///
    /// In general, you're not allowed to create a mutable reference from
    /// `Allocated`, unless you're defining the object and know that to be
    /// safe.
    ///
    /// For example, `+[NSMutableString alloc]` is allowed to return a
    /// non-unique object as an optimization, and then only figure out
    /// afterwards whether it needs to allocate, or if it can store an
    /// `NSString` internally.
    ///
    /// Similarly, while e.g. `+[NSData alloc]` may return a unique object,
    /// calling `-[NSData init]` afterwards could return a shared empty
    /// `NSData` instance.
    #[inline]
    #[allow(unknown_lints)] // New lint below
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn as_mut_ptr(this: &mut Self) -> *mut T
    where
        T: IsMutable,
    {
        this.ptr as *mut T
    }

    #[inline]
    pub(crate) fn into_ptr(this: Self) -> *mut T {
        let this = ManuallyDrop::new(this);
        this.ptr as *mut T
    }
}

impl<T: ?Sized> Drop for Allocated<T> {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: Allocated objects can always safely be released, since
        // destructors are written to take into account that the object may
        // not have been initialized.
        //
        // This is also safe in the case where the object is NULL,
        // since `objc_release` allows NULL pointers.
        //
        // Rest is same as `Id`'s `Drop`.
        unsafe { objc_release_fast(self.ptr as *mut _) };
    }
}

impl<T: ?Sized> fmt::Pointer for Allocated<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}

#[cfg(test)]
mod tests {
    use core::panic::{RefUnwindSafe, UnwindSafe};

    use static_assertions::assert_not_impl_any;

    use super::*;
    use crate::runtime::NSObject;

    #[test]
    fn auto_traits() {
        assert_not_impl_any!(Allocated<()>: Send, Sync, UnwindSafe, RefUnwindSafe, Unpin);
    }

    #[repr(C)]
    struct MyObject<'a> {
        inner: NSObject,
        p: PhantomData<&'a str>,
    }

    /// Test that `Allocated<T>` is covariant over `T`.
    #[allow(unused)]
    fn assert_variance<'b>(obj: Allocated<MyObject<'static>>) -> Allocated<MyObject<'b>> {
        obj
    }
}
