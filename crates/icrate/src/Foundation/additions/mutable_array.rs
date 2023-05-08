#![cfg(feature = "Foundation_NSMutableArray")]
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ops::{Index, IndexMut};

use objc2::mutability::{IsMutable, IsRetainable};

use super::util;
use crate::common::*;
use crate::Foundation::{self, NSArray, NSComparisonResult, NSInteger, NSMutableArray};

/// Creation methods.
impl<T: Message> NSMutableArray<T> {
    pub fn from_vec(mut vec: Vec<Id<T>>) -> Id<Self> {
        let len = vec.len();
        let ptr = util::id_ptr_cast(vec.as_mut_ptr());
        // SAFETY: Same as `NSArray::from_vec`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn from_id_slice(slice: &[Id<T>]) -> Id<Self>
    where
        T: IsIdCloneable,
    {
        let len = slice.len();
        let ptr = util::id_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_id_slice`
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn from_slice(slice: &[&T]) -> Id<Self>
    where
        T: IsRetainable,
    {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_slice`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }
}

/// Accessor methods.
impl<T: Message> NSMutableArray<T> {
    #[doc(alias = "addObject:")]
    pub fn push(&mut self, obj: Id<T>) {
        // SAFETY: We've consumed ownership of the object.
        unsafe { self.addObject(&obj) }
    }

    #[doc(alias = "insertObject:atIndex:")]
    pub fn insert(&mut self, index: usize, obj: Id<T>) {
        // TODO: Replace this check with catching the thrown NSRangeException
        let len = self.len();
        if index < len {
            // SAFETY: We've consumed ownership of the object, and the
            // index is checked to be in bounds.
            unsafe { self.insertObject_atIndex(&obj, index) }
        } else {
            panic!(
                "insertion index (is {}) should be <= len (is {})",
                index, len
            );
        }
    }

    #[doc(alias = "replaceObjectAtIndex:withObject:")]
    pub fn replace(&mut self, index: usize, obj: Id<T>) -> Result<Id<T>, Id<T>> {
        if let Some(old_obj) = self.get(index) {
            // SAFETY: We remove the object from the array below.
            let old_obj = unsafe { util::mutable_collection_retain_removed_id(old_obj) };
            // SAFETY: The index is checked to be in bounds, and we've
            // consumed ownership of the new object.
            unsafe { self.replaceObjectAtIndex_withObject(index, &obj) };
            Ok(old_obj)
        } else {
            Err(obj)
        }
    }

    #[doc(alias = "removeObjectAtIndex:")]
    pub fn remove(&mut self, index: usize) -> Option<Id<T>> {
        let obj = self.get(index)?;
        // SAFETY: We remove the object from the array below.
        let obj = unsafe { util::mutable_collection_retain_removed_id(obj) };
        // SAFETY: The index is checked to be in bounds.
        unsafe { self.removeObjectAtIndex(index) };
        Some(obj)
    }

    #[doc(alias = "removeLastObject")]
    pub fn pop(&mut self) -> Option<Id<T>> {
        let obj = self.last()?;
        // SAFETY: We remove the object from the array below.
        let obj = unsafe { util::mutable_collection_retain_removed_id(obj) };
        // SAFETY: Just checked that there is an object.
        unsafe { self.removeLastObject() };
        Some(obj)
    }

    #[doc(alias = "sortUsingFunction:context:")]
    pub fn sort_by<F: FnMut(&T, &T) -> Ordering>(&mut self, compare: F) {
        // TODO: "C-unwind"
        unsafe extern "C" fn compare_with_closure<U, F: FnMut(&U, &U) -> Ordering>(
            obj1: NonNull<U>,
            obj2: NonNull<U>,
            context: *mut c_void,
        ) -> NSInteger {
            let context: *mut F = context.cast();
            // Bring back a reference to the closure.
            // Guaranteed to be unique, we gave `sortUsingFunction` unique is
            // ownership, and that method only runs one function at a time.
            let closure: &mut F = unsafe { context.as_mut().unwrap_unchecked() };

            // SAFETY: The objects are guaranteed to be valid
            let (obj1, obj2) = unsafe { (obj1.as_ref(), obj2.as_ref()) };

            NSComparisonResult::from((*closure)(obj1, obj2)) as _
        }

        // Create function pointer
        let f: unsafe extern "C" fn(_, _, _) -> _ = compare_with_closure::<T, F>;

        // Grab a type-erased pointer to the closure (a pointer to stack).
        let mut closure = compare;
        let context: *mut F = &mut closure;

        unsafe { self.sortUsingFunction_context(f, context.cast()) };
        // Keep the closure alive until the function has run.
        drop(closure);
    }

    pub fn into_vec(array: Id<Self>) -> Vec<Id<T>> {
        // SAFETY: We've consumed the array, so taking ownership of the
        // returned values is safe.
        array
            .to_vec()
            .into_iter()
            .map(|obj| unsafe { util::mutable_collection_retain_removed_id(obj) })
            .collect()
    }
}

unsafe impl<T: Message> Foundation::NSFastEnumeration2 for NSMutableArray<T> {
    type Item = T;
}

impl<'a, T: Message> IntoIterator for &'a NSMutableArray<T> {
    type Item = &'a T;
    type IntoIter = Foundation::NSFastEnumerator2<'a, NSMutableArray<T>>;

    fn into_iter(self) -> Self::IntoIter {
        use Foundation::NSFastEnumeration2;
        self.iter_fast()
    }
}

impl<T: Message> Extend<Id<T>> for NSMutableArray<T> {
    fn extend<I: IntoIterator<Item = Id<T>>>(&mut self, iter: I) {
        let iterator = iter.into_iter();
        iterator.for_each(move |item| self.push(item));
    }
}

impl<T: Message> Index<usize> for NSMutableArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

impl<T: IsMutable> IndexMut<usize> for NSMutableArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap()
    }
}
