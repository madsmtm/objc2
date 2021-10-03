use alloc::borrow;
use core::fmt;
use core::hash;
use core::iter::FusedIterator;
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;
use std::error::Error;

use super::AutoreleasePool;
use super::{Owned, Ownership, Shared};
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
// TODO: Add `?Sized + ptr::Thin` bound on `T` to allow for extern types
// TODO: Consider changing the name of Id -> Retain
pub struct Id<T, O: Ownership> {
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

// TODO: Maybe make most of these functions "associated" functions instead?
impl<T: Message, O: Ownership> Id<T, O> {
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
        Id {
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
    #[cfg_attr(debug_assertions, inline)]
    pub unsafe fn retain(ptr: NonNull<T>) -> Id<T, O> {
        let ptr = ptr.as_ptr() as *mut objc2_sys::objc_object;
        // SAFETY: The caller upholds that the pointer is valid
        let res = objc2_sys::objc_retain(ptr);
        debug_assert_eq!(res, ptr, "objc_retain did not return the same pointer");
        // SAFETY: Non-null upheld by the caller, and `objc_retain` always
        // returns the same pointer, see:
        // https://clang.llvm.org/docs/AutomaticReferenceCounting.html#arc-runtime-objc-retain
        Id::new(NonNull::new_unchecked(res as *mut T))
    }

    #[cfg_attr(debug_assertions, inline)]
    fn autorelease_inner(self) -> *mut T {
        // Note that this (and the actual `autorelease`) is not an associated
        // function. This breaks the guideline that smart pointers shouldn't
        // add inherent methods, but since autoreleasing only works on already
        // retained objects it is hard to imagine a case where the inner type
        // has a method with the same name.

        let ptr = ManuallyDrop::new(self).ptr.as_ptr() as *mut objc2_sys::objc_object;
        // SAFETY: The `ptr` is guaranteed to be valid and have at least one
        // retain count.
        // And because of the ManuallyDrop, we don't call the Drop
        // implementation, so the object won't also be released there.
        let res = unsafe { objc2_sys::objc_autorelease(ptr) };
        debug_assert_eq!(res, ptr, "objc_autorelease did not return the same pointer");
        res as *mut T
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

impl<T: Message> Id<T, Owned> {
    /// Autoreleases the owned [`Id`], returning a mutable reference bound to
    /// the pool.
    ///
    /// The object is not immediately released, but will be when the innermost
    /// / current autorelease pool (given as a parameter) is drained.
    #[doc(alias = "objc_autorelease")]
    #[must_use = "If you don't intend to use the object any more, just drop it as usual"]
    #[inline]
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
    /// The caller must ensure that there are no other pointers to the same
    /// object (which also means that the given [`Id`] should have a retain
    /// count of exactly 1 all cases, except when autoreleases are involved).
    #[inline]
    pub unsafe fn from_shared(obj: Id<T, Shared>) -> Self {
        // Note: We can't debug_assert retainCount because of autoreleases
        let ptr = ManuallyDrop::new(obj).ptr;
        // SAFETY: The pointer is valid
        // Ownership rules are upheld by the caller
        <Id<T, Owned>>::new(ptr)
    }
}

impl<T: Message> Id<T, Shared> {
    /// Autoreleases the shared [`Id`], returning an aliased reference bound
    /// to the pool.
    ///
    /// The object is not immediately released, but will be when the innermost
    /// / current autorelease pool (given as a parameter) is drained.
    #[doc(alias = "objc_autorelease")]
    #[must_use = "If you don't intend to use the object any more, just drop it as usual"]
    #[inline]
    pub fn autorelease<'p>(self, pool: &'p AutoreleasePool) -> &'p T {
        let ptr = self.autorelease_inner();
        // SAFETY: The pointer is valid as a reference
        unsafe { pool.ptr_as_ref(ptr) }
    }
}

impl<T: Message> From<Id<T, Owned>> for Id<T, Shared> {
    /// Downgrade from an owned to a shared [`Id`], allowing it to be cloned.
    #[inline]
    fn from(obj: Id<T, Owned>) -> Self {
        let ptr = ManuallyDrop::new(obj).ptr;
        // SAFETY: The pointer is valid, and ownership is simply decreased
        unsafe { <Id<T, Shared>>::new(ptr) }
    }
}

impl<T: Message> Clone for Id<T, Shared> {
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
impl<T, O: Ownership> Drop for Id<T, O> {
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
        unsafe { objc2_sys::objc_release(self.ptr.as_ptr() as *mut _) };
    }
}

/// The `Send` implementation requires `T: Sync` because `Id<T, Shared>` give
/// access to `&T`.
///
/// Additiontally, it requires `T: Send` because if `T: !Send`, you could
/// clone a `Id<T, Shared>`, send it to another thread, and drop the clone
/// last, making `dealloc` get called on the other thread, and violate
/// `T: !Send`.
unsafe impl<T: Sync + Send> Send for Id<T, Shared> {}

/// The `Sync` implementation requires `T: Sync` because `&Id<T, Shared>` give
/// access to `&T`.
///
/// Additiontally, it requires `T: Send`, because if `T: !Send`, you could
/// clone a `&Id<T, Shared>` from another thread, and drop the clone last,
/// making `dealloc` get called on the other thread, and violate `T: !Send`.
unsafe impl<T: Sync + Send> Sync for Id<T, Shared> {}

/// `Id<T, Owned>` are `Send` if `T` is `Send` because they give the same
/// access as having a T directly.
unsafe impl<T: Send> Send for Id<T, Owned> {}

/// `Id<T, Owned>` are `Sync` if `T` is `Sync` because they give the same
/// access as having a `T` directly.
unsafe impl<T: Sync> Sync for Id<T, Owned> {}

impl<T, O: Ownership> Deref for Id<T, O> {
    type Target = T;

    /// Obtain an immutable reference to the object.
    fn deref(&self) -> &T {
        // SAFETY: The pointer's validity is verified when the type is created
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for Id<T, Owned> {
    /// Obtain a mutable reference to the object.
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: The pointer's validity is verified when the type is created
        // Additionally, the owned `Id` is the unique owner of the object, so
        // mutability is safe.
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: PartialEq, O: Ownership> PartialEq for Id<T, O> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (**self).eq(&**other)
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        (**self).ne(&**other)
    }
}

impl<T: Eq, O: Ownership> Eq for Id<T, O> {}

impl<T: PartialOrd, O: Ownership> PartialOrd for Id<T, O> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        (**self).partial_cmp(&**other)
    }
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        (**self).lt(&**other)
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        (**self).le(&**other)
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        (**self).ge(&**other)
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        (**self).gt(&**other)
    }
}

impl<T: Ord, O: Ownership> Ord for Id<T, O> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: hash::Hash, O: Ownership> hash::Hash for Id<T, O> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

// TODO: impl Hasher?

impl<T: fmt::Display, O: Ownership> fmt::Display for Id<T, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: fmt::Debug, O: Ownership> fmt::Debug for Id<T, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T, O: Ownership> fmt::Pointer for Id<T, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr.as_ptr(), f)
    }
}

impl<I: Iterator> Iterator for Id<I, Owned> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        (**self).next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        (**self).nth(n)
    }
}

impl<I: DoubleEndedIterator> DoubleEndedIterator for Id<I, Owned> {
    fn next_back(&mut self) -> Option<I::Item> {
        (**self).next_back()
    }
    fn nth_back(&mut self, n: usize) -> Option<I::Item> {
        (**self).nth_back(n)
    }
}

impl<I: ExactSizeIterator> ExactSizeIterator for Id<I, Owned> {
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<I: FusedIterator> FusedIterator for Id<I, Owned> {}

impl<T, O: Ownership> borrow::Borrow<T> for Id<T, O> {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T> borrow::BorrowMut<T> for Id<T, Owned> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut **self
    }
}

impl<T, O: Ownership> AsRef<T> for Id<T, O> {
    fn as_ref(&self) -> &T {
        &**self
    }
}

impl<T> AsMut<T> for Id<T, Owned> {
    fn as_mut(&mut self) -> &mut T {
        &mut **self
    }
}

impl<T: Error, O: Ownership> Error for Id<T, O> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        (**self).source()
    }
}

// TODO: impl io traits Seek, Read, BufRead and Write

// TODO:
// impl<F: Future + Unpin> Future for Id<F, Owned> {
//     type Output = F::Output;
//     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         F::poll(Pin::new(&mut *self), cx)
//     }
// }

// This is valid without `T: Unpin` because we don't implement any projection.
//
// See https://doc.rust-lang.org/1.54.0/src/alloc/boxed.rs.html#1652-1675
// and the `Arc` implementation.
impl<T, O: Ownership> Unpin for Id<T, O> {}

// TODO: When stabilized impl Fn traits & CoerceUnsized

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
        assert!(retain_count(&obj) == 1);

        let obj: Id<_, Shared> = obj.into();
        assert!(retain_count(&obj) == 1);

        let cloned = obj.clone();
        assert!(retain_count(&cloned) == 2);
        assert!(retain_count(&obj) == 2);

        drop(obj);
        assert!(retain_count(&cloned) == 1);
    }
}
