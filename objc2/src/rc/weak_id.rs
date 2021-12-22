use alloc::boxed::Box;
use core::cell::UnsafeCell;
use core::fmt;
use core::marker::PhantomData;
use core::ptr;
use core::ptr::NonNull;
use std::panic::{RefUnwindSafe, UnwindSafe};

use super::{Id, Shared};
use crate::ffi;
use crate::Message;

/// A pointer type for a weak reference to an Objective-C reference counted
/// object.
///
/// Allows breaking reference cycles and safely checking whether the object
/// has been deallocated.
#[repr(transparent)]
pub struct WeakId<T: ?Sized> {
    /// We give the runtime the address to this box, so that it can modify it
    /// even if the `WeakId` is moved.
    ///
    /// Loading may modify the pointer through a shared reference, so we use
    /// an UnsafeCell to get a *mut without self being mutable.
    ///
    /// TODO: Verify the need for UnsafeCell?
    inner: Box<UnsafeCell<*mut T>>,
    /// WeakId inherits variance, dropck and various marker traits from
    /// `Id<T, Shared>` because it can be upgraded to a shared Id.
    item: PhantomData<Id<T, Shared>>,
}

impl<T: Message + ?Sized + ptr::Thin> WeakId<T> {
    /// Construct a new [`WeakId`] referencing the given shared [`Id`].
    #[doc(alias = "objc_initWeak")]
    pub fn new(obj: &Id<T, Shared>) -> Self {
        // Note that taking `&Id<T, Owned>` would not be safe since that would
        // allow loading an `Id<T, Shared>` later on.

        // SAFETY: `obj` is valid
        unsafe { Self::new_inner(&**obj as *const T as *mut T) }
    }

    /// # Safety
    ///
    /// The object must be valid or null.
    unsafe fn new_inner(obj: *mut T) -> Self {
        let null = ptr::from_raw_parts_mut(ptr::null_mut(), ());
        let inner = Box::new(UnsafeCell::new(null));
        // SAFETY: `ptr` will never move, and the caller verifies `obj`
        let _ = unsafe { ffi::objc_initWeak(inner.get() as _, obj as _) };
        Self {
            inner,
            item: PhantomData,
        }
    }

    /// Load a shared (and retained) [`Id`] if the object still exists.
    ///
    /// Returns [`None`] if the object has been deallocated or was created
    /// with [`Default::default`].
    #[doc(alias = "upgrade")]
    #[doc(alias = "objc_loadWeak")]
    #[doc(alias = "objc_loadWeakRetained")]
    #[inline]
    pub fn load(&self) -> Option<Id<T, Shared>> {
        let ptr: *mut *mut ffi::objc_object = self.inner.get() as _;
        let obj = unsafe { ffi::objc_loadWeakRetained(ptr) } as *mut ();
        let obj: *mut T = ptr::from_raw_parts_mut(obj, ());
        NonNull::new(obj).map(|obj| unsafe { Id::new(obj) })
    }
}

impl<T: ?Sized> Drop for WeakId<T> {
    /// Drops the `WeakId` pointer.
    #[doc(alias = "objc_destroyWeak")]
    fn drop(&mut self) {
        unsafe { ffi::objc_destroyWeak(self.inner.get() as _) }
    }
}

impl<T: ?Sized + ptr::Thin> Clone for WeakId<T> {
    /// Makes a clone of the `WeakId` that points to the same object.
    #[doc(alias = "objc_copyWeak")]
    fn clone(&self) -> Self {
        let null = ptr::from_raw_parts_mut(ptr::null_mut(), ());
        let ptr = Box::new(UnsafeCell::new(null));
        unsafe { ffi::objc_copyWeak(ptr.get() as _, self.inner.get() as _) };
        Self {
            inner: ptr,
            item: PhantomData,
        }
    }
}

impl<T: Message + ?Sized + ptr::Thin> Default for WeakId<T> {
    /// Constructs a new `WeakId<T>` that doesn't reference any object.
    ///
    /// Calling [`Self::load`] on the return value always gives [`None`].
    fn default() -> Self {
        let null = ptr::from_raw_parts_mut(ptr::null_mut(), ());
        // SAFETY: The pointer is null
        unsafe { Self::new_inner(null) }
    }
}

/// This implementation follows the same reasoning as `Id<T, Shared>`.
unsafe impl<T: Sync + Send + ?Sized> Sync for WeakId<T> {}

/// This implementation follows the same reasoning as `Id<T, Shared>`.
unsafe impl<T: Sync + Send + ?Sized> Send for WeakId<T> {}

// Unsure about the Debug bound on T, see std::sync::Weak
impl<T: fmt::Debug + ?Sized> fmt::Debug for WeakId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(WeakId)")
    }
}

// Underneath this is just a `Box`
impl<T: ?Sized> Unpin for WeakId<T> {}

// Same as `Id<T, Shared>`.
impl<T: RefUnwindSafe + ?Sized> RefUnwindSafe for WeakId<T> {}

// Same as `Id<T, Shared>`.
impl<T: RefUnwindSafe + ?Sized> UnwindSafe for WeakId<T> {}

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;

    use super::WeakId;
    use super::{Id, Shared};
    use crate::runtime::Object;
    use crate::{class, msg_send};

    #[test]
    fn test_weak() {
        let cls = class!(NSObject);
        let obj: Id<Object, Shared> = unsafe {
            let obj: *mut Object = msg_send![cls, alloc];
            let obj: *mut Object = msg_send![obj, init];
            Id::new(NonNull::new_unchecked(obj))
        };

        let weak = WeakId::new(&obj);
        let strong = weak.load().unwrap();
        let strong_ptr: *const Object = &*strong;
        let obj_ptr: *const Object = &*obj;
        assert_eq!(strong_ptr, obj_ptr);
        drop(strong);

        drop(obj);
        assert!(weak.load().is_none());
    }

    #[test]
    fn test_weak_clone() {
        let obj: Id<Object, Shared> = unsafe { Id::new(msg_send![class!(NSObject), new]) };
        let weak = WeakId::new(&obj);

        let weak2 = weak.clone();

        let strong = weak.load().unwrap();
        let strong2 = weak2.load().unwrap();
        let strong_ptr: *const Object = &*strong;
        let strong2_ptr: *const Object = &*strong2;
        let obj_ptr: *const Object = &*obj;
        assert_eq!(strong_ptr, obj_ptr);
        assert_eq!(strong2_ptr, obj_ptr);
    }

    #[test]
    fn test_weak_default() {
        let weak: WeakId<Object> = WeakId::default();
        assert!(weak.load().is_none());
        drop(weak);
    }
}
