use alloc::boxed::Box;
use core::cell::UnsafeCell;
use core::fmt;
use core::marker::{PhantomData, PhantomPinned};
use core::pin::Pin;
use core::ptr::{self, NonNull};
use std::panic::{RefUnwindSafe, UnwindSafe};

use super::Retained;
use crate::runtime::AnyObject;
use crate::{ffi, Message};

/// A weak pointer to an Objective-C reference counted object.
///
/// Once the object is deallocated, all remaining weak pointers to it is set
/// to `NULL`.
///
/// Useful for breaking reference cycles and safely checking whether an
/// object has been deallocated.
///
///
/// # Comparison to `std` types
///
/// This is the Objective-C equivalent of [`sync::Weak`] from the standard
/// library, and hence is only usable on types where `Retained<T>` acts like
/// [`sync::Arc`], a.k.a. on non-mutable types.
///
///
/// # Pinning
///
/// This is a [pinned][core::pin] object, meaning it can generally only be
/// used in places where you know that it won't move. This can be accomplished
/// using the wrappers `Pin<Box<Weak<T>>>` or `Pin<&Weak<T>>`.
///
/// In instance variables, you will need to wrap the ivar access with
/// `Pin::new_unchecked`, which is safe because instance variables are pinned.
///
///
/// # Memory layout
///
/// This is guaranteed to have the same size and alignment as a pointer to the
/// object, `*const T`. In Objective-C terms, this has the same layout as a
/// `__weak` variable (and is ABI-compatible with that).
///
/// [`sync::Weak`]: std::sync::Weak
/// [`sync::Arc`]: std::sync::Arc
#[repr(transparent)]
#[doc(alias = "WeakId")] // Previous name
#[cfg_attr(
    feature = "unstable-coerce-pointee",
    derive(std::marker::CoercePointee)
)]
pub struct Weak<T: ?Sized> {
    /// The runtime holds the address of this field, and will zero it when the
    /// object is deallocated.
    ///
    /// Since this means the pointer may be modified through a shared
    /// reference, we use an `UnsafeCell`.
    ///
    /// Note that any thread may actually modify the inner value concurrently,
    /// but as long as we only use it through the `objc_XXXWeak` methods, all
    /// access is behind a lock.
    inner: UnsafeCell<*mut AnyObject>,
    /// The runtime holds a reference to this type, so it cannot be moved
    /// after it has been constructed.
    pinned: PhantomPinned,
    /// Covariant over `T`, and necessary for dropck.
    item: PhantomData<T>,
}

/// Fully-deprecated type-alias to [`Weak`].
#[deprecated(since = "0.6.0", note = "Renamed to `Weak`.")]
pub type WeakId<T> = Weak<T>;

impl<T: Message> Weak<T> {
    /// A pointer to the inner object pointer.
    ///
    /// The outer pointer is a valid `__weak` object, or points to `NULL`, for
    /// as long as the [`Weak`] is alive.
    ///
    /// The pointer is not synchronized, so reading from it is only valid when
    /// you know that nothing else is touching it.
    #[inline]
    #[allow(dead_code)] // Difficult semantics, I'd rather not expose this yet.
    fn as_ptr(&self) -> NonNull<*mut T> {
        NonNull::new(self.inner.get().cast()).unwrap()
    }

    /// Constructs a new weak pointer that doesn't reference any object.
    ///
    /// # Example
    ///
    /// Place a weak pointer in a `static`.
    ///
    /// ```
    /// use std::pin::Pin;
    /// use objc2::rc::Weak;
    /// # // New class that is Send + Sync.
    /// # objc2::define_class!(
    /// #     #[derive(Debug, PartialEq, Eq, Hash)]
    /// #     #[unsafe(super(objc2::runtime::NSObject))]
    /// #     #[name = "WeakUUID"]
    /// #     struct NSUUID;
    /// # );
    /// #
    /// # impl NSUUID {
    /// #     objc2::extern_methods!(
    /// #         #[unsafe(method(new))]
    /// #         fn UUID() -> objc2::rc::Retained<Self>;
    /// #     );
    /// # }
    /// # #[cfg(requires_foundation)]
    /// use objc2_foundation::NSUUID;
    ///
    /// static WEAK: Weak<NSUUID> = Weak::empty();
    ///
    /// // Initially nothing is stored in the weak pointer.
    /// assert_eq!(WEAK.load(), None);
    ///
    /// // But we can set the weak pointer to an object.
    /// let obj = NSUUID::UUID();
    /// Pin::static_ref(&WEAK).store(Some(&obj));
    ///
    /// // Such that it now loads that object.
    /// assert_eq!(WEAK.load().as_ref(), Some(&obj));
    ///
    /// // Until the object is no longer alive.
    /// drop(obj);
    /// assert_eq!(WEAK.load(), None);
    /// ```
    #[inline]
    pub const fn empty() -> Self {
        // SAFETY: The pointer is null, which is valid for us to pass to all
        // the `objc_*Weak` functions.
        Self {
            inner: UnsafeCell::new(ptr::null_mut()),
            pinned: PhantomPinned,
            item: PhantomData,
        }
    }

    /// Construct a new boxed weak pointer.
    ///
    /// # Example
    ///
    /// ```
    /// use objc2::rc::Weak;
    /// use objc2::runtime::NSObject;
    ///
    /// let obj = NSObject::new();
    /// let weak = Weak::new(&*obj);
    /// assert_eq!(weak.load(), Some(obj));
    /// ```
    #[doc(alias = "objc_initWeak")]
    // We do two operations (allocate Box and init weak), but we probably want
    // this to be inlined regardless, as that'd allow reordering and re-use of
    // the allocation (though fairly unlikely).
    #[inline]
    pub fn new(obj: &T) -> Pin<Box<Self>> {
        let obj = obj as *const T as *mut T;

        let boxed = Box::pin(Self::default());
        // SAFETY:
        // - The weak pointer is newly initialized, and will never move
        //   because it is pinned.
        // - The object pointer is valid since it came from a reference.
        let _ = unsafe { ffi::objc_initWeak(boxed.inner.get(), obj.cast()) };
        boxed
    }

    /// Construct a new boxed weak pointer that references the given [`Retained`].
    #[doc(alias = "objc_initWeak")]
    #[deprecated = "use `Weak::from_retained` instead"]
    #[inline]
    pub fn from_id(obj: &Retained<T>) -> Pin<Box<Self>> {
        Self::from_retained(obj)
    }

    /// Construct a new boxed weak pointer that references the given
    /// [`Retained`] object.
    ///
    /// Convenience alias for [`Weak::new`].
    ///
    /// # Example
    ///
    /// ```
    /// use objc2::rc::Weak;
    /// use objc2::runtime::NSObject;
    ///
    /// let obj = NSObject::new();
    /// let weak = Weak::from_retained(&obj); // No `Deref` needed
    /// assert_eq!(weak.load(), Some(obj));
    /// ```
    #[doc(alias = "objc_initWeak")]
    #[inline]
    pub fn from_retained(obj: &Retained<T>) -> Pin<Box<Self>> {
        Self::new(obj)
    }

    /// Load the object into an [`Retained`] if it still exists.
    ///
    /// Returns [`None`] if:
    /// - The object has been deallocated.
    /// - The weak pointer was created with [`Weak::empty()`] and not yet set.
    /// - The weak pointer was cleared with [`weak.store(None)`][Self::store].
    ///
    /// # Example
    ///
    /// Load an object from a weak pointer as long as it's still alive.
    ///
    /// ```
    /// use objc2::rc::Weak;
    /// use objc2::runtime::NSObject;
    ///
    /// let obj = NSObject::new();
    /// let weak = Weak::from_retained(&obj);
    ///
    /// // Loading an object that is alive returns `Some`.
    /// assert_eq!(weak.load().as_ref(), Some(&obj));
    ///
    /// drop(obj);
    /// // The object is no longer alive, so now loading returns `None`.
    /// assert_eq!(weak.load(), None);
    /// ```
    ///
    /// Load an zero-initialized weak pointer.
    ///
    /// ```
    /// use objc2::rc::Weak;
    /// use objc2::runtime::NSObject;
    ///
    /// let obj = NSObject::new();
    /// let weak = Weak::<NSObject>::empty();
    /// assert_eq!(weak.load(), None);
    /// ```
    #[doc(alias = "retain")]
    #[doc(alias = "objc_loadWeak")]
    #[doc(alias = "objc_loadWeakRetained")]
    #[inline]
    pub fn load(&self) -> Option<Retained<T>> {
        // SAFETY: The weak pointer is either NULL (newly initialized) or
        // contains a weak pointer.
        //
        // NOTE: We don't need `self: Pin<&Self>`, since all functions that
        // store something into the pointer require pinning. Thus the only
        // time where `self` would be un-pinned would be if it was
        // zero-initialized.
        //
        // This makes usage slightly nicer (we don't need `.as_ref()`), and
        let obj = unsafe { ffi::objc_loadWeakRetained(self.inner.get()) }.cast();
        // SAFETY: The object has +1 retain count from ^.
        unsafe { Retained::from_raw(obj) }
    }

    // TODO: Add `autorelease(&self, pool) -> Option<&T>` using `objc_loadWeak`?

    /// Make a boxed copy of the weak pointer that points to the same object.
    #[doc(alias = "objc_copyWeak")]
    #[inline]
    pub fn copy(self: Pin<&Self>) -> Pin<Box<Self>> {
        let boxed = Box::pin(Self::empty());
        // SAFETY:
        // - The source pointer is either NULL (newly initialized) or contains
        //   a weak pointer, and it will never move because it is pinned.
        // - The destination pointer is newly initialized, and will never move
        //   because it is pinned.
        unsafe { ffi::objc_copyWeak(boxed.inner.get(), self.inner.get()) };
        boxed
    }

    /// Set the weak pointer to a new object.
    ///
    /// You can explicitly set the value to [`None`], though it is rarely that
    /// useful, as that will will be done automatically once the object is
    /// deallocated.
    ///
    ///
    /// # Examples
    ///
    /// Store a value into an empty weak pointer.
    ///
    /// ```
    /// use std::pin::pin;
    /// use objc2::rc::Weak;
    /// use objc2::runtime::NSObject;
    ///
    /// let weak = pin!(Weak::default());
    /// assert_eq!(weak.load(), None);
    ///
    /// let obj = NSObject::new();
    /// weak.as_ref().store(Some(&*obj));
    /// assert_eq!(weak.load(), Some(obj));
    /// ```
    ///
    /// Overwrite a boxed weak pointer.
    ///
    /// ```
    /// use objc2::rc::Weak;
    /// use objc2::runtime::NSObject;
    ///
    /// let obj1 = NSObject::new();
    /// let weak = Weak::from_retained(&obj1);
    ///
    /// let obj2 = NSObject::new();
    /// weak.as_ref().store(Some(&obj2));
    ///
    /// assert_eq!(weak.load(), Some(obj2));
    /// ```
    ///
    /// Explicitly set the weak pointer to [`None`].
    ///
    /// ```
    /// use objc2::rc::Weak;
    /// use objc2::runtime::NSObject;
    ///
    /// let obj = NSObject::new();
    /// let weak = Weak::from_retained(&obj);
    ///
    /// weak.as_ref().store(None);
    /// assert_eq!(weak.load(), None);
    /// ```
    #[doc(alias = "objc_storeWeak")]
    #[inline]
    pub fn store(self: Pin<&Self>, obj: Option<&T>) {
        let obj = obj
            .map(|obj| obj as *const T as *mut T)
            .unwrap_or_else(ptr::null_mut);

        // NOTE: We don't use `objc_destroyWeak` on `NULL` pointers, as that
        // is not guaranteed to be thread-safe.
        // If we wanted to do that instead, we'd need `Pin<&mut Self>`.

        // SAFETY:
        // - The weak pointer is either NULL (newly initialized) or contains a
        //   weak pointer, and it will never move because it is pinned.
        // - The object pointer is either NULL or valid since it came from a
        //   reference.
        let _ = unsafe { ffi::objc_storeWeak(self.inner.get(), obj.cast()) };
    }

    // TODO: objc_moveWeak? Probably needs better pin ergonomics.
}

impl<T: ?Sized> Drop for Weak<T> {
    /// Destroys the weak pointer.
    #[doc(alias = "objc_destroyWeak")]
    #[inline]
    fn drop(&mut self) {
        // SAFETY: The weak pointer is either valid or contains NULL.
        //
        // We also know that it was never moved since its construction because
        // `Self` is `!Unpin` (which means we can safely pass).
        unsafe { ffi::objc_destroyWeak(self.inner.get()) }
    }
}

// TODO: Add ?Sized
impl<T: Message> Clone for Pin<Box<Weak<T>>> {
    #[doc(alias = "objc_copyWeak")]
    #[inline]
    fn clone(&self) -> Self {
        self.as_ref().copy()
    }
}

// TODO: Add ?Sized
impl<T: Message> Default for Weak<T> {
    /// Constructs a new weak pointer that doesn't reference any object.
    ///
    /// This is equivalent to [`Weak::empty()`].
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl<T: ?Sized> fmt::Debug for Weak<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Note: We intentionally don't try to debug-print the value, since
        // that could lead to cycles. See:
        // https://github.com/rust-lang/rust/pull/90291
        write!(f, "(Weak)")
    }
}

// SAFETY: Same as `std::sync::Weak<T>`.
// The `objc_*Weak` methods use a lock internally, so it's safe to load this
// from multiple threads, even while the underlying object may be deallocated.
unsafe impl<T: ?Sized + Sync + Send> Sync for Weak<T> {}

// SAFETY: Same as `std::sync::Weak<T>`.
// The `objc_*Weak` methods use a lock internally, so it's safe to load this
// from multiple threads, even while the underlying object may be deallocated.
unsafe impl<T: ?Sized + Sync + Send> Send for Weak<T> {}

// Same as `std::sync::Weak<T>`.
impl<T: ?Sized + RefUnwindSafe> RefUnwindSafe for Weak<T> {}

// Same as `std::sync::Weak<T>`.
impl<T: ?Sized + RefUnwindSafe> UnwindSafe for Weak<T> {}

impl<T: Message> From<&T> for Pin<Box<Weak<T>>> {
    #[inline]
    fn from(obj: &T) -> Self {
        Weak::new(obj)
    }
}

impl<T: Message> From<&Retained<T>> for Pin<Box<Weak<T>>> {
    #[inline]
    fn from(obj: &Retained<T>) -> Self {
        Weak::from_retained(obj)
    }
}

impl<T: Message> From<Retained<T>> for Pin<Box<Weak<T>>> {
    #[inline]
    fn from(obj: Retained<T>) -> Self {
        Weak::from_retained(&obj)
    }
}

#[cfg(test)]
mod tests {
    use core::mem;

    use super::*;
    use crate::rc::{RcTestObject, ThreadTestData};
    use crate::runtime::NSObject;
    use crate::{define_class, msg_send, AnyThread, Ivars};

    #[test]
    fn test_weak() {
        let obj = RcTestObject::new();
        let mut expected = ThreadTestData::current();

        let weak = Weak::from_retained(&obj);
        expected.assert_current();

        // A weak object was registered at the pointer.
        assert!(!unsafe { weak.inner.get().read() }.is_null());

        let strong = weak.load().unwrap();
        expected.try_retain += 1;
        expected.assert_current();
        assert!(ptr::eq(&*strong, &*obj));

        drop(obj);
        drop(strong);
        expected.release += 2;
        expected.drop += 1;
        expected.assert_current();

        // The object is deallocated, which sets weak pointers to null.
        assert!(unsafe { weak.inner.get().read() }.is_null());

        if cfg!(not(feature = "gnustep-1-7")) {
            // This loads the object on GNUStep for some reason??
            assert!(weak.load().is_none());
            expected.assert_current();
        }

        drop(weak);
        expected.assert_current();
    }

    #[test]
    fn test_weak_clone() {
        let obj = RcTestObject::new();
        let mut expected = ThreadTestData::current();

        let weak = Weak::from_retained(&obj);
        expected.assert_current();

        let weak2 = weak.clone();
        if cfg!(target_vendor = "apple") {
            expected.try_retain += 1;
            expected.release += 1;
        }
        expected.assert_current();

        let strong = weak.load().unwrap();
        expected.try_retain += 1;
        expected.assert_current();
        assert!(ptr::eq(&*strong, &*obj));

        let strong2 = weak2.load().unwrap();
        expected.try_retain += 1;
        expected.assert_current();
        assert!(ptr::eq(&*strong, &*strong2));

        drop(weak);
        drop(weak2);
        expected.assert_current();
    }

    #[test]
    fn test_weak_default() {
        let weak: Weak<RcTestObject> = Default::default();
        assert!(weak.load().is_none());
        drop(weak);

        let weak: Pin<Box<Weak<RcTestObject>>> = Default::default();
        assert!(weak.clone().load().is_none());
        assert!(weak.load().is_none());
        drop(weak);
    }

    #[repr(C)]
    struct MyObject<'a> {
        inner: NSObject,
        p: PhantomData<&'a str>,
    }

    /// Test that `Weak<T>` is covariant over `T`.
    #[allow(unused)]
    fn assert_variance<'a, 'b>(obj: &'a Weak<MyObject<'static>>) -> &'a Weak<MyObject<'b>> {
        obj
    }

    #[test]
    fn test_size_of() {
        let ptr_size = mem::size_of::<*const ()>();
        assert_eq!(mem::size_of::<Weak<NSObject>>(), ptr_size);
        assert_ne!(mem::size_of::<Option<Weak<NSObject>>>(), ptr_size);
    }

    static_assertions::assert_not_impl_any!(Weak<NSObject>: Unpin);

    define_class!(
        #[unsafe(super(NSObject))]
        struct WeakClass {
            boxed: Pin<Box<Weak<RcTestObject>>>,
            plain: Weak<RcTestObject>,
        }
    );

    impl WeakClass {
        fn new(obj: &RcTestObject) -> Retained<Self> {
            let this = Self::alloc().set_ivars(Ivars::<Self> {
                boxed: Weak::new(obj),
                plain: Weak::empty(),
            });
            // Call `NSObject`'s `init` method.
            let this: Retained<Self> = unsafe { msg_send![super(this), init] };
            // SAFETY: Ivars are pinned.
            unsafe { Pin::new_unchecked(this.plain()) }.store(Some(obj));
            this
        }
    }

    #[test]
    fn defined() {
        let obj = RcTestObject::new();
        let mut expected = ThreadTestData::current();

        let x = WeakClass::new(&obj);
        expected.assert_current();

        assert_eq!(x.boxed().load().as_ref(), Some(&obj));
        expected.try_retain += 1;
        expected.release += 1;
        expected.assert_current();

        assert_eq!(x.plain().load().as_ref(), Some(&obj));
        expected.try_retain += 1;
        expected.release += 1;
        expected.assert_current();
    }
}
