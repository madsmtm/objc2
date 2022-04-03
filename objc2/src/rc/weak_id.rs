use alloc::boxed::Box;
use core::cell::UnsafeCell;
use core::fmt;
use core::marker::PhantomData;
use core::ptr;
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
    /// Remember that any thread may actually modify the inner value
    /// concurrently, but as long as we only use it through the `objc_XXXWeak`
    /// methods, all access is behind a lock.
    ///
    /// TODO: Verify the need for UnsafeCell?
    /// TODO: Investigate if we can avoid some allocations using `Pin`.
    inner: Box<UnsafeCell<*mut ffi::objc_object>>,
    /// WeakId inherits variance, dropck and various marker traits from
    /// `Id<T, Shared>` because it can be upgraded to a shared Id.
    item: PhantomData<Id<T, Shared>>,
}

impl<T: Message> WeakId<T> {
    /// Construct a new [`WeakId`] referencing the given shared [`Id`].
    #[doc(alias = "objc_initWeak")]
    #[inline]
    pub fn new(obj: &Id<T, Shared>) -> Self {
        // Note that taking `&Id<T, Owned>` would not be safe since that would
        // allow loading an `Id<T, Shared>` later on.

        // SAFETY: `obj` is valid
        unsafe { Self::new_inner(obj.as_ptr()) }
    }

    /// # Safety
    ///
    /// The object must be valid or null.
    unsafe fn new_inner(obj: *mut T) -> Self {
        let inner = Box::new(UnsafeCell::new(ptr::null_mut()));
        // SAFETY: `ptr` will never move, and the caller verifies `obj`
        let _ = unsafe { ffi::objc_initWeak(inner.get(), obj as *mut ffi::objc_object) };
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
    #[doc(alias = "retain")]
    #[doc(alias = "objc_loadWeak")]
    #[doc(alias = "objc_loadWeakRetained")]
    #[inline]
    pub fn load(&self) -> Option<Id<T, Shared>> {
        let ptr = self.inner.get();
        let obj = unsafe { ffi::objc_loadWeakRetained(ptr) } as *mut T;
        unsafe { Id::new(obj) }
    }

    // TODO: Add `autorelease(&self) -> Option<&T>` using `objc_loadWeak`?
}

impl<T: ?Sized> Drop for WeakId<T> {
    /// Drops the `WeakId` pointer.
    #[doc(alias = "objc_destroyWeak")]
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::objc_destroyWeak(self.inner.get()) }
    }
}

// TODO: Add ?Sized
impl<T> Clone for WeakId<T> {
    /// Makes a clone of the `WeakId` that points to the same object.
    #[doc(alias = "objc_copyWeak")]
    fn clone(&self) -> Self {
        let ptr = Box::new(UnsafeCell::new(ptr::null_mut()));
        unsafe { ffi::objc_copyWeak(ptr.get(), self.inner.get()) };
        Self {
            inner: ptr,
            item: PhantomData,
        }
    }
}

// TODO: Add ?Sized
impl<T: Message> Default for WeakId<T> {
    /// Constructs a new `WeakId<T>` that doesn't reference any object.
    ///
    /// Calling [`Self::load`] on the return value always gives [`None`].
    #[inline]
    fn default() -> Self {
        // SAFETY: The pointer is null
        unsafe { Self::new_inner(ptr::null_mut()) }
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

impl<T: Message> From<Id<T, Shared>> for WeakId<T> {
    #[inline]
    fn from(obj: Id<T, Shared>) -> Self {
        WeakId::new(&obj)
    }
}

impl<T: Message> TryFrom<WeakId<T>> for Id<T, Shared> {
    type Error = ();
    fn try_from(weak: WeakId<T>) -> Result<Self, ()> {
        return weak.load().ok_or(());
    }
}

#[cfg(test)]
mod tests {
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
            Id::new(obj).unwrap()
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
        let obj: Id<Object, Shared> = unsafe { Id::new(msg_send![class!(NSObject), new]).unwrap() };
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
