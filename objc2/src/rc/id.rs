use core::fmt;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use std::panic::{RefUnwindSafe, UnwindSafe};

use super::AutoreleasePool;
use super::{Owned, Ownership, Shared};
use crate::ffi;
use crate::Message;

/// An pointer for Objective-C reference counted objects.
///
/// [`Id`] strongly references or "retains" the given object `T`, and
/// "releases" it again when dropped, thereby ensuring it will be deallocated
/// at the right time.
///
/// An [`Id`] can either be [`Owned`] or [`Shared`], represented with the `O`
/// type parameter.
///
/// If owned, it is guaranteed that there are no other references to the
/// object, and the [`Id`] can therefore be mutably dereferenced.
///
/// If shared, however, it can only be immutably dereferenced because there
/// may be other references to the object, since a shared [`Id`] can be cloned
/// to provide exactly that.
///
/// An [`Id<T, Owned>`] can be safely "downgraded", that is, turned into to a
/// `Id<T, Shared>` using `From`/`Into`. The opposite is not safely possible,
/// but the unsafe option [`Id::from_shared`] is provided.
///
/// `Option<Id<T, O>>` is guaranteed to have the same size as a pointer to the
/// object.
///
/// # Comparison to `std` types
///
/// `Id<T, Owned>` can be thought of as the Objective-C equivalent of [`Box`]
/// from the standard library: It is a unique pointer to some allocated
/// object, and that means you're allowed to get a mutable reference to it.
///
/// Likewise, `Id<T, Shared>` is the Objective-C equivalent of [`Arc`]: It is
/// a reference-counting pointer that, when cloned, increases the reference
/// count.
///
/// [`Box`]: alloc::boxed::Box
/// [`Arc`]: alloc::sync::Arc
///
/// # Caveats
///
/// If the inner type implements [`Drop`], that implementation will not be
/// called, since there is no way to ensure that the Objective-C runtime will
/// do so. If you need to run some code when the object is destroyed,
/// implement the `dealloc` method instead.
///
/// This allows `?Sized` types `T`, but the intention is to only support when
/// `T` is an `extern type` (yet unstable).
///
/// # Examples
///
/// ```no_run
/// use objc2::msg_send;
/// use objc2::runtime::{Class, Object};
/// use objc2::rc::{Id, Owned, Shared, WeakId};
///
/// let cls = Class::get("NSObject").unwrap();
/// let obj: Id<Object, Owned> = unsafe {
///     Id::new(msg_send![cls, new])
/// };
/// // obj will be released when it goes out of scope
///
/// // share the object so we can clone it
/// let obj: Id<_, Shared> = obj.into();
/// let another_ref = obj.clone();
/// // dropping our other reference will decrement the retain count
/// drop(another_ref);
///
/// let weak = WeakId::new(&obj);
/// assert!(weak.load().is_some());
/// // After the object is deallocated, our weak pointer returns none
/// drop(obj);
/// assert!(weak.load().is_none());
/// ```
///
/// ```no_run
/// # use objc2::{class, msg_send};
/// # use objc2::runtime::Object;
/// # use objc2::rc::{Id, Owned, Shared};
/// # type T = Object;
/// let mut owned: Id<T, Owned>;
/// # owned = unsafe { Id::new(msg_send![class!(NSObject), new]) };
/// let mut_ref: &mut T = &mut *owned;
/// // Do something with `&mut T` here
///
/// let shared: Id<T, Shared> = owned.into();
/// let cloned: Id<T, Shared> = shared.clone();
/// // Do something with `&T` here
/// ```
#[repr(transparent)]
// TODO: Figure out if `Message` bound on `T` would be better here?
// TODO: Add `ptr::Thin` bound on `T` to allow for only extern types
// TODO: Consider changing the name of Id -> Retain
pub struct Id<T: ?Sized, O: Ownership> {
    /// A pointer to the contained object. The pointer is always retained.
    ///
    /// It is important that this is `NonNull`, since we want to dereference
    /// it later, and be able to use the null-pointer optimization.
    ///
    /// Additionally, covariance is correct because we're either the unique
    /// owner of `T` (O = Owned), or `T` is immutable (O = Shared).
    ptr: NonNull<T>,
    /// Necessary for dropck even though we never actually run T's destructor,
    /// because it might have a `dealloc` that assumes that contained
    /// references outlive the type.
    ///
    /// See <https://doc.rust-lang.org/nightly/nomicon/phantom-data.html>
    item: PhantomData<T>,
    /// To prevent warnings about unused type parameters.
    own: PhantomData<O>,
}

impl<T: Message + ?Sized, O: Ownership> Id<T, O> {
    /// Constructs an [`Id`] to an object that already has +1 retain count.
    ///
    /// This is useful when you have a retain count that has been handed off
    /// from somewhere else, usually Objective-C methods like `init`, `alloc`,
    /// `new`, `copy`, or methods with the `ns_returns_retained` attribute.
    ///
    /// Since most of the above methods create new objects, and you therefore
    /// hold unique access to the object, you would often set the ownership to
    /// be [`Owned`].
    ///
    /// But some immutable objects (like `NSString`) don't always return
    /// unique references, so in those case you would use [`Shared`].
    ///
    /// # Safety
    ///
    /// The caller must ensure the given object has +1 retain count, and that
    /// the object pointer otherwise follows the same safety requirements as
    /// in [`Id::retain`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use objc2::{class, msg_send};
    /// # use objc2::runtime::{Class, Object};
    /// # use objc2::rc::{Id, Owned};
    /// let cls: &Class;
    /// # let cls = class!(NSObject);
    /// let obj: &mut Object = unsafe { msg_send![cls, alloc] };
    /// let obj: Id<Object, Owned> = unsafe { Id::new(msg_send![obj, init]) };
    /// // Or in this case simply just:
    /// let obj: Id<Object, Owned> = unsafe { Id::new(msg_send![cls, new]) };
    /// ```
    ///
    /// ```no_run
    /// # use objc2::{class, msg_send};
    /// # use objc2::runtime::Object;
    /// # use objc2::rc::{Id, Shared};
    /// # type NSString = Object;
    /// let cls = class!(NSString);
    /// // NSString is immutable, so don't create an owned reference to it
    /// let obj: Id<NSString, Shared> = unsafe { Id::new(msg_send![cls, new]) };
    /// ```
    #[inline]
    // Note: We don't take a reference as a parameter since it would be too
    // easy to accidentally create two aliasing mutable references.
    pub unsafe fn new(ptr: NonNull<T>) -> Id<T, O> {
        // SAFETY: Upheld by the caller
        Self {
            ptr,
            item: PhantomData,
            own: PhantomData,
        }
    }

    /// Retains the given object pointer.
    ///
    /// This is useful when you have been given a pointer to an object from
    /// some API, and you would like to ensure that the object stays around
    /// so that you can work with it.
    ///
    /// This is rarely used to construct owned [`Id`]s, see [`Id::new`] for
    /// that.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the ownership is correct; that is, there
    /// must be no [`Owned`] pointers or mutable references to the same
    /// object, and when creating owned [`Id`]s, there must be no other
    /// pointers or references to the object.
    ///
    /// Additionally, the pointer must be valid as a reference (aligned,
    /// dereferencable and initialized, see the [`std::ptr`] module for more
    /// information).
    ///
    /// [`std::ptr`]: core::ptr
    //
    // This would be illegal:
    // ```no_run
    // let owned: Id<T, Owned>;
    // // Lifetime information is discarded
    // let retained: Id<T, Shared> = unsafe { Id::retain(&*owned) };
    // // Which means we can still mutate `Owned`:
    // let x: &mut T = &mut *owned;
    // // While we have an immutable reference
    // let y: &T = &*retained;
    // ```
    #[doc(alias = "objc_retain")]
    #[cfg_attr(not(debug_assertions), inline)]
    pub unsafe fn retain(ptr: NonNull<T>) -> Id<T, O> {
        let ptr = ptr.as_ptr();
        let obj_ptr = ptr as *mut ffi::objc_object;
        // SAFETY: The caller upholds that the pointer is valid
        let res = unsafe { ffi::objc_retain(obj_ptr) };
        debug_assert_eq!(res, obj_ptr, "objc_retain did not return the same pointer");
        // SAFETY: Non-null upheld by the caller, and `objc_retain` always
        // returns the same pointer, see:
        // https://clang.llvm.org/docs/AutomaticReferenceCounting.html#arc-runtime-objc-retain
        //
        // TODO: Use `res` instead, in the odd case where `objc_retain` did
        // not return the same pointer. We can't do this currently because we
        // have a ?Sized bound on T to support extern types, which means we
        // can go from `*mut T` to `*mut objc_object`, but not the other way.
        let res = unsafe { NonNull::new_unchecked(ptr) };
        // SAFETY: We just retained the object, so it has +1 retain count
        unsafe { Self::new(res) }
    }

    #[cfg_attr(not(debug_assertions), inline)]
    fn autorelease_inner(self) -> *mut T {
        // Note that this (and the actual `autorelease`) is not an associated
        // function. This breaks the guideline that smart pointers shouldn't
        // add inherent methods, but since autoreleasing only works on already
        // retained objects it is hard to imagine a case where the inner type
        // has a method with the same name.

        let ptr = ManuallyDrop::new(self).ptr.as_ptr();
        let obj_ptr = ptr as *mut ffi::objc_object;
        // SAFETY: The `ptr` is guaranteed to be valid and have at least one
        // retain count.
        // And because of the ManuallyDrop, we don't call the Drop
        // implementation, so the object won't also be released there.
        let res = unsafe { ffi::objc_autorelease(obj_ptr) };
        debug_assert_eq!(
            res, obj_ptr,
            "objc_autorelease did not return the same pointer"
        );
        // TODO: Return `res`, see explanation in `retain`.
        ptr
    }

    // TODO: objc_retainAutoreleasedReturnValue
    // TODO: objc_autoreleaseReturnValue
    // TODO: objc_retainAutorelease
    // TODO: objc_retainAutoreleaseReturnValue
    // TODO: objc_autoreleaseReturnValue
    // TODO: objc_autoreleaseReturnValue
}

// TODO: Consider something like this
// #[cfg(block)]
// impl<T: Block, O> Id<T, O> {
//     #[doc(alias = "objc_retainBlock")]
//     pub unsafe fn retain_block(block: NonNull<T>) -> Self {
//         todo!()
//     }
// }

impl<T: Message + ?Sized> Id<T, Owned> {
    /// Autoreleases the owned [`Id`], returning a mutable reference bound to
    /// the pool.
    ///
    /// The object is not immediately released, but will be when the innermost
    /// / current autorelease pool (given as a parameter) is drained.
    #[doc(alias = "objc_autorelease")]
    #[must_use = "If you don't intend to use the object any more, just drop it as usual"]
    #[inline]
    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::mut_from_ref)]
    pub fn autorelease<'p>(self, pool: &'p AutoreleasePool) -> &'p mut T {
        let ptr = self.autorelease_inner();
        // SAFETY: The pointer is valid as a reference, and we've consumed
        // the unique access to the `Id` so mutability is safe.
        unsafe { pool.ptr_as_mut(ptr) }
    }

    /// Promote a shared [`Id`] to an owned one, allowing it to be mutated.
    ///
    /// # Safety
    ///
    /// The caller must ensure that there are no other pointers (including
    /// [`WeakId`][`super::WeakId`] pointers) to the same object.
    ///
    /// This also means that the given [`Id`] should have a retain count of
    /// exactly 1 (except when autoreleases are involved).
    #[inline]
    pub unsafe fn from_shared(obj: Id<T, Shared>) -> Self {
        // Note: We can't debug_assert retainCount because of autoreleases
        let ptr = ManuallyDrop::new(obj).ptr;
        // SAFETY: The pointer is valid
        // Ownership rules are upheld by the caller
        unsafe { <Id<T, Owned>>::new(ptr) }
    }
}

impl<T: Message + ?Sized> Id<T, Shared> {
    /// Autoreleases the shared [`Id`], returning an aliased reference bound
    /// to the pool.
    ///
    /// The object is not immediately released, but will be when the innermost
    /// / current autorelease pool (given as a parameter) is drained.
    #[doc(alias = "objc_autorelease")]
    #[must_use = "If you don't intend to use the object any more, just drop it as usual"]
    #[inline]
    #[allow(clippy::needless_lifetimes)]
    pub fn autorelease<'p>(self, pool: &'p AutoreleasePool) -> &'p T {
        let ptr = self.autorelease_inner();
        // SAFETY: The pointer is valid as a reference
        unsafe { pool.ptr_as_ref(ptr) }
    }
}

impl<T: Message + ?Sized> From<Id<T, Owned>> for Id<T, Shared> {
    /// Downgrade from an owned to a shared [`Id`], allowing it to be cloned.
    #[inline]
    fn from(obj: Id<T, Owned>) -> Self {
        let ptr = ManuallyDrop::new(obj).ptr;
        // SAFETY: The pointer is valid, and ownership is simply decreased
        unsafe { <Id<T, Shared>>::new(ptr) }
    }
}

impl<T: Message + ?Sized> Clone for Id<T, Shared> {
    /// Makes a clone of the shared object.
    ///
    /// This increases the object's reference count.
    #[doc(alias = "objc_retain")]
    #[doc(alias = "retain")]
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: The pointer is valid
        unsafe { Id::retain(self.ptr) }
    }
}

/// `#[may_dangle]` (see [this][dropck_eyepatch]) doesn't apply here since we
/// don't run `T`'s destructor (rather, we want to discourage having `T`s with
/// a destructor); and even if we did run the destructor, it would not be safe
/// to add since we cannot verify that a `dealloc` method doesn't access
/// borrowed data.
///
/// [dropck_eyepatch]: https://doc.rust-lang.org/nightly/nomicon/dropck.html#an-escape-hatch
impl<T: ?Sized, O: Ownership> Drop for Id<T, O> {
    /// Releases the retained object.
    ///
    /// The contained object's destructor (if it has one) is never run!
    #[doc(alias = "objc_release")]
    #[doc(alias = "release")]
    #[inline]
    fn drop(&mut self) {
        // We could technically run the destructor for `T` when `O = Owned`,
        // and when `O = Shared` with (retainCount == 1), but that would be
        // confusing and inconsistent since we cannot guarantee that it's run.

        // SAFETY: The `ptr` is guaranteed to be valid and have at least one
        // retain count
        unsafe { ffi::objc_release(self.ptr.as_ptr() as *mut _) };
    }
}

/// The `Send` implementation requires `T: Sync` because `Id<T, Shared>` give
/// access to `&T`.
///
/// Additiontally, it requires `T: Send` because if `T: !Send`, you could
/// clone a `Id<T, Shared>`, send it to another thread, and drop the clone
/// last, making `dealloc` get called on the other thread, and violate
/// `T: !Send`.
unsafe impl<T: Sync + Send + ?Sized> Send for Id<T, Shared> {}

/// The `Sync` implementation requires `T: Sync` because `&Id<T, Shared>` give
/// access to `&T`.
///
/// Additiontally, it requires `T: Send`, because if `T: !Send`, you could
/// clone a `&Id<T, Shared>` from another thread, and drop the clone last,
/// making `dealloc` get called on the other thread, and violate `T: !Send`.
unsafe impl<T: Sync + Send + ?Sized> Sync for Id<T, Shared> {}

/// `Id<T, Owned>` are `Send` if `T` is `Send` because they give the same
/// access as having a T directly.
unsafe impl<T: Send + ?Sized> Send for Id<T, Owned> {}

/// `Id<T, Owned>` are `Sync` if `T` is `Sync` because they give the same
/// access as having a `T` directly.
unsafe impl<T: Sync + ?Sized> Sync for Id<T, Owned> {}

impl<T: ?Sized, O: Ownership> Deref for Id<T, O> {
    type Target = T;

    /// Obtain an immutable reference to the object.
    // Box doesn't inline, but that's because it's a compiler built-in
    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: The pointer's validity is verified when the type is created
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for Id<T, Owned> {
    /// Obtain a mutable reference to the object.
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: The pointer's validity is verified when the type is created
        // Additionally, the owned `Id` is the unique owner of the object, so
        // mutability is safe.
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized, O: Ownership> fmt::Pointer for Id<T, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr.as_ptr(), f)
    }
}

// This is valid without `T: Unpin` because we don't implement any projection.
//
// See https://doc.rust-lang.org/1.54.0/src/alloc/boxed.rs.html#1652-1675
// and the `Arc` implementation.
impl<T: ?Sized, O: Ownership> Unpin for Id<T, O> {}

impl<T: RefUnwindSafe + ?Sized, O: Ownership> RefUnwindSafe for Id<T, O> {}

// Same as `Arc<T>`.
impl<T: RefUnwindSafe + ?Sized> UnwindSafe for Id<T, Shared> {}

// Same as `Box<T>`.
impl<T: UnwindSafe + ?Sized> UnwindSafe for Id<T, Owned> {}

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;

    use super::{Id, Owned, Shared};
    use crate::rc::autoreleasepool;
    use crate::runtime::Object;
    use crate::{class, msg_send};

    fn retain_count(obj: &Object) -> usize {
        unsafe { msg_send![obj, retainCount] }
    }

    #[test]
    fn test_autorelease() {
        let obj: Id<Object, Shared> = unsafe { Id::new(msg_send![class!(NSObject), new]) };

        let cloned = obj.clone();

        autoreleasepool(|pool| {
            let _ref = obj.autorelease(pool);
            assert_eq!(retain_count(&*cloned), 2);
        });

        // make sure that the autoreleased value has been released
        assert_eq!(retain_count(&*cloned), 1);
    }

    #[test]
    fn test_clone() {
        let cls = class!(NSObject);
        let obj: Id<Object, Owned> = unsafe {
            let obj: *mut Object = msg_send![cls, alloc];
            let obj: *mut Object = msg_send![obj, init];
            Id::new(NonNull::new_unchecked(obj))
        };
        assert_eq!(retain_count(&obj), 1);

        let obj: Id<_, Shared> = obj.into();
        assert_eq!(retain_count(&obj), 1);

        let cloned = obj.clone();
        assert_eq!(retain_count(&cloned), 2);
        assert_eq!(retain_count(&obj), 2);

        drop(obj);
        assert_eq!(retain_count(&cloned), 1);
    }
}
