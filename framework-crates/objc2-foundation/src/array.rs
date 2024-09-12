//! Utilities for the `NSArray` and `NSMutableArray` classes.
use alloc::vec::Vec;
#[cfg(feature = "NSEnumerator")]
use core::fmt;
use core::mem;
use core::ptr::NonNull;

use objc2::rc::{Retained, RetainedFromIterator};
use objc2::{msg_send, AllocAnyThread, Message};

#[cfg(feature = "NSEnumerator")]
use crate::iter;
use crate::{util, NSArray, NSMutableArray};

/// Convenience creation methods.
impl<T: Message> NSArray<T> {
    /// Create a new array from a slice of objects.
    ///
    /// This is a safe interface to `initWithObjects:count:`.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSArray, ns_string};
    ///
    /// let array = NSArray::from_slice(&[
    ///     ns_string!("abc"),
    ///     ns_string!("def"),
    ///     ns_string!("ghi"),
    /// ]);
    /// ```
    #[doc(alias = "initWithObjects:count:")]
    pub fn from_slice(slice: &[&T]) -> Retained<Self> {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY:
        // - All `T: Message` use interior mutability, and the array extends
        //   the lifetime of them internally by retaining them.
        // - The pointer and length are valid until the method has finished
        //   executing, at which point the array will have created its own
        //   internal storage for holding the pointers.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    /// Create a new array from a slice of retained objects.
    ///
    /// This is a safe interface to `initWithObjects:count:`.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSArray, NSObject};
    ///
    /// let array = NSArray::from_retained_slice(&[
    ///     NSObject::new(),
    ///     NSObject::new(),
    ///     NSObject::new(),
    /// ]);
    /// ```
    #[doc(alias = "initWithObjects:count:")]
    pub fn from_retained_slice(slice: &[Retained<T>]) -> Retained<Self> {
        let len = slice.len();
        let ptr = util::retained_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `from_slice`, this is just a faster version to
        // avoid creating a new slice if your elements are already retained.
        //
        // Otherwise equivalent to:
        //     Self::from_slice(&slice.iter().map(|obj| &*obj).collect())
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }
}

/// Convenience creation methods.
impl<T: Message> NSMutableArray<T> {
    #[doc(alias = "initWithObjects:count:")]
    pub fn from_slice(slice: &[&T]) -> Retained<Self> {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_slice`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    #[doc(alias = "initWithObjects:count:")]
    pub fn from_retained_slice(slice: &[Retained<T>]) -> Retained<Self> {
        let len = slice.len();
        let ptr = util::retained_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_retained_slice`
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }
}

/// Direct, unsafe object accessors.
///
/// Foundation's collection types store their items in such a way that they
/// can give out references to their data without having to autorelease it
/// first, see [the docs][collections-own].
///
/// This means that we can more efficiently access the array's objects, but
/// _only_ if the array isn't mutated via. e.g. `NSMutableArray` methods while
/// doing so - otherwise, we might end up accessing a deallocated object.
///
/// [collections-own]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmPractical.html#//apple_ref/doc/uid/TP40004447-SW12
impl<T: Message> NSArray<T> {
    /// Get a direct reference to one of the array's objects.
    ///
    /// Throws an error if the object was not found.
    ///
    /// Consider using the [`objectAtIndex`](Self::objectAtIndex) method
    /// instead, unless you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The array must not be mutated while the reference is live.
    #[doc(alias = "objectAtIndex:")]
    #[inline]
    pub unsafe fn objectAtIndex_unchecked(&self, index: usize) -> &T {
        // SAFETY: Upheld by caller.
        unsafe { msg_send![self, objectAtIndex: index] }
    }

    /// A direct reference to the array's first object, if any.
    ///
    /// Consider using the [`firstObject`](Self::firstObject) method instead,
    /// unless you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The array must not be mutated while the reference is live.
    #[doc(alias = "firstObject")]
    #[inline]
    pub unsafe fn firstObject_unchecked(&self) -> Option<&T> {
        // SAFETY: Upheld by caller.
        unsafe { msg_send![self, firstObject] }
    }

    /// A direct reference to the array's last object, if any.
    ///
    /// Consider using the [`lastObject`](Self::lastObject) method instead,
    /// unless you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The array must not be mutated while the reference is live.
    #[doc(alias = "lastObject")]
    #[inline]
    pub unsafe fn lastObject_unchecked(&self) -> Option<&T> {
        // SAFETY: Upheld by caller.
        unsafe { msg_send![self, lastObject] }
    }

    /// A vector containing direct references to the array's objects.
    ///
    /// Consider using the [`to_vec`](Self::to_vec) method instead, unless
    /// you're seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The array must not be mutated while the returned references are alive.
    #[doc(alias = "getObjects:")]
    pub unsafe fn to_vec_unchecked(&self) -> Vec<&T> {
        let len = self.count();
        let mut vec: Vec<NonNull<T>> = Vec::with_capacity(len);
        let ptr: NonNull<NonNull<T>> = NonNull::new(vec.as_mut_ptr()).unwrap();

        // SAFETY: The buffer is at least the size of the array, as guaranteed
        // by `Vec::with_capacity`.
        unsafe {
            #[allow(deprecated)]
            self.getObjects(ptr)
        };

        // SAFETY: The elements were just initialized by `getObjects:`.
        //
        // Note: We set the length _after_ we've copied the elements, so that
        // if `getObjects:` unwinds, we don't end up deallocating
        // uninitialized elements.
        unsafe { vec.set_len(len) };

        // SAFETY: `NonNull<T>` has the same layout as `&T`, and the lifetime
        // is bound to the array, and caller upholds that the array isn't
        // mutated.
        unsafe { mem::transmute::<Vec<NonNull<T>>, Vec<&T>>(vec) }
    }

    /// Iterate over the array without retaining the elements.
    ///
    /// Consider using the [`iter`](Self::iter) method instead, unless you're
    /// seeing performance issues from the retaining.
    ///
    /// # Safety
    ///
    /// The array must not be mutated for the lifetime of the iterator, or the
    /// elements it returns.
    #[cfg(feature = "NSEnumerator")]
    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub unsafe fn iter_unchecked(&self) -> IterUnchecked<'_, T> {
        IterUnchecked(iter::IterUnchecked::new(self))
    }
}

/// Various accessor methods.
impl<T: Message> NSArray<T> {
    /// The amount of elements in the array.
    #[doc(alias = "count")]
    #[inline]
    pub fn len(&self) -> usize {
        self.count()
    }

    /// Whether the array is empty or not.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Convert the array to a `Vec` of the array's objects.
    #[doc(alias = "getObjects:")]
    pub fn to_vec(&self) -> Vec<Retained<T>> {
        // SAFETY: We retain the elements below, so we know that the array
        // isn't mutated while the references are alive.
        //
        // Note that this is _technically_ wrong; the user _could_ have
        // implemented a `retain` method that mutates the array. We're going
        // to rule this out though, as that's basically never going to happen,
        // and will make a lot of other things unsound too.
        let vec = unsafe { self.to_vec_unchecked() };
        vec.into_iter().map(T::retain).collect()
    }

    /// Iterate over the array's elements.
    #[cfg(feature = "NSEnumerator")]
    #[doc(alias = "objectEnumerator")]
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(iter::Iter::new(self))
    }

    /// Returns the objects within the given range.
    ///
    /// # Panics
    ///
    /// Panics if the range was out of bounds.
    #[doc(alias = "getObjects:range:")]
    #[cfg(feature = "NSRange")]
    pub fn objects_in_range(&self, range: core::ops::Range<usize>) -> Vec<Retained<T>> {
        let count = self.count();

        // TODO: Replace this check with catching the thrown NSRangeException
        if range.end > count {
            panic!(
                "range end index {} out of range for array of length {}",
                range.end, count
            );
        }

        let range = crate::NSRange::from(range);
        let mut vec: Vec<NonNull<T>> = Vec::with_capacity(range.length);
        let ptr: NonNull<NonNull<T>> = NonNull::new(vec.as_mut_ptr()).unwrap();

        // SAFETY: Mostly the same as in `to_vec_unchecked`.
        unsafe { self.getObjects_range(ptr, range) };
        unsafe { vec.set_len(range.length) };
        let vec = unsafe { mem::transmute::<Vec<NonNull<T>>, Vec<&T>>(vec) };

        vec.into_iter().map(T::retain).collect()
    }
}

/// Convenience mutation methods.
impl<T: Message> NSMutableArray<T> {
    /// Insert an object into the array at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    #[doc(alias = "insertObject:atIndex:")]
    pub fn insert(&self, index: usize, obj: &T) {
        // TODO: Replace this check with catching the thrown NSRangeException
        let len = self.len();
        if index < len {
            self.insertObject_atIndex(obj, index)
        } else {
            panic!(
                "insertion index (is {}) should be <= len (is {})",
                index, len
            );
        }
    }

    /// Sort the array by the given comparison closure.
    #[cfg(feature = "NSObjCRuntime")]
    #[doc(alias = "sortUsingFunction:context:")]
    pub fn sort_by<F: FnMut(&T, &T) -> core::cmp::Ordering>(&self, compare: F) {
        unsafe extern "C-unwind" fn compare_with_closure<
            T,
            F: FnMut(&T, &T) -> core::cmp::Ordering,
        >(
            obj1: core::ptr::NonNull<T>,
            obj2: core::ptr::NonNull<T>,
            context: *mut core::ffi::c_void,
        ) -> isize {
            let context: *mut F = context.cast();
            // Bring back a reference to the closure.
            // Guaranteed to be unique, we gave `sortUsingFunction` unique
            // ownership, and that method only runs one function at a time.
            let closure: &mut F = unsafe { context.as_mut().unwrap_unchecked() };

            // SAFETY: The objects are guaranteed to be valid
            let (obj1, obj2) = unsafe { (obj1.as_ref(), obj2.as_ref()) };

            crate::NSComparisonResult::from((*closure)(obj1, obj2)) as _
        }

        // Create function pointer
        let f: unsafe extern "C-unwind" fn(_, _, _) -> _ = compare_with_closure::<T, F>;

        // Grab a type-erased pointer to the closure (a pointer to stack).
        let mut closure = compare;
        let context: *mut F = &mut closure;

        unsafe { self.sortUsingFunction_context(f, context.cast()) };
        // Keep the closure alive until the function has run.
        drop(closure);
    }
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<T: Message> iter::FastEnumerationHelper for NSArray<T> {
    type Item = T;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<T: Message> iter::FastEnumerationHelper for NSMutableArray<T> {
    type Item = T;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

/// An iterator over the items of an array.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct Iter<'a, T: Message>(iter::Iter<'a, NSArray<T>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, T: Message> Iterator<Item = Retained<T>> for Iter<'a, T> { ... }
}

/// An iterator over unretained items of an array.
///
/// # Safety
///
/// The array must not be mutated while this is alive.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct IterUnchecked<'a, T: Message>(iter::IterUnchecked<'a, NSArray<T>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, T: Message> Iterator<Item = &'a T> for IterUnchecked<'a, T> { ... }
}

/// A retained iterator over the items of an array.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct IntoIter<T: Message>(iter::IntoIter<NSArray<T>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<T: Message> Iterator<Item = Retained<T>> for IntoIter<T> { ... }
}

#[cfg(feature = "NSEnumerator")]
__impl_into_iter! {
    impl<T: Message> IntoIterator for &NSArray<T> {
        type IntoIter = Iter<'_, T>;
    }

    impl<T: Message> IntoIterator for &NSMutableArray<T> {
        type IntoIter = Iter<'_, T>;
    }

    impl<T: Message> IntoIterator for Retained<NSArray<T>> {
        #[uses(new)]
        type IntoIter = IntoIter<T>;
    }

    impl<T: Message> IntoIterator for Retained<NSMutableArray<T>> {
        #[uses(new_mutable)]
        type IntoIter = IntoIter<T>;
    }
}

#[cfg(feature = "NSEnumerator")]
impl<T: fmt::Debug + Message> fmt::Debug for NSArray<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

#[cfg(feature = "NSEnumerator")]
impl<T: fmt::Debug + Message> fmt::Debug for NSMutableArray<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: Message> Extend<Retained<T>> for &NSMutableArray<T> {
    fn extend<I: IntoIterator<Item = Retained<T>>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |item| self.addObject(&item));
    }
}

impl<'a, T: Message> Extend<&'a T> for &NSMutableArray<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |item| self.addObject(item));
    }
}

impl<'a, T: Message + 'a> RetainedFromIterator<&'a T> for NSArray<T> {
    fn id_from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_slice(&vec)
    }
}

impl<T: Message> RetainedFromIterator<Retained<T>> for NSArray<T> {
    fn id_from_iter<I: IntoIterator<Item = Retained<T>>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_retained_slice(&vec)
    }
}

impl<'a, T: Message + 'a> RetainedFromIterator<&'a T> for NSMutableArray<T> {
    fn id_from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Retained<Self> {
        // TODO: Is this, or is using `initWithCapacity` the most optimal?
        let vec = Vec::from_iter(iter);
        Self::from_slice(&vec)
    }
}

impl<T: Message> RetainedFromIterator<Retained<T>> for NSMutableArray<T> {
    fn id_from_iter<I: IntoIterator<Item = Retained<T>>>(iter: I) -> Retained<Self> {
        // TODO: Is this, or is using `initWithCapacity` the most optimal?
        let vec = Vec::from_iter(iter);
        Self::from_retained_slice(&vec)
    }
}
