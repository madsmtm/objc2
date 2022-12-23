use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ffi::c_void;
use core::ops::{Index, IndexMut};
use core::ptr::NonNull;

use objc2::rc::{DefaultId, Id, Owned, Ownership, Shared, SliceId};
use objc2::{extern_methods, ClassType, Message};

use super::array::with_objects;
use crate::Foundation::{
    NSArray, NSComparisonResult, NSCopying, NSFastEnumeration2, NSFastEnumerator2, NSInteger,
    NSMutableArray, NSMutableCopying,
};

extern_methods!(
    /// Generic creation methods.
    unsafe impl<T: Message, O: Ownership> NSMutableArray<T, O> {
        // SAFETY: Same as `NSArray::new`, except mutable arrays are always
        // unique.
        #[method_id(new)]
        pub fn new() -> Id<Self, Owned>;

        pub fn from_vec(vec: Vec<Id<T, O>>) -> Id<Self, Owned> {
            // SAFETY: Same as `NSArray::from_vec`, except mutable arrays are
            // always unique.
            unsafe { with_objects(vec.as_slice_ref()) }
        }
    }

    /// Creation methods that produce shared arrays.
    unsafe impl<T: Message> NSMutableArray<T, Shared> {
        pub fn from_slice(slice: &[Id<T, Shared>]) -> Id<Self, Owned> {
            // SAFETY: Same as `NSArray::from_slice`, except mutable arrays are
            // always unique.
            unsafe { with_objects(slice.as_slice_ref()) }
        }
    }

    /// Generic accessor methods.
    unsafe impl<T: Message, O: Ownership> NSMutableArray<T, O> {
        #[doc(alias = "addObject:")]
        pub fn push(&mut self, obj: Id<T, O>) {
            // SAFETY: The object has correct ownership.
            unsafe { self.addObject(&obj) }
        }

        #[doc(alias = "insertObject:atIndex:")]
        pub fn insert(&mut self, index: usize, obj: Id<T, O>) {
            // TODO: Replace this check with catching the thrown NSRangeException
            let len = self.len();
            if index < len {
                // SAFETY: The object has correct ownership, and the index is
                // checked to be in bounds.
                unsafe { self.insertObject_atIndex(&obj, index) }
            } else {
                panic!(
                    "insertion index (is {}) should be <= len (is {})",
                    index, len
                );
            }
        }

        #[doc(alias = "replaceObjectAtIndex:withObject:")]
        pub fn replace(&mut self, index: usize, obj: Id<T, O>) -> Id<T, O> {
            let old_obj = unsafe {
                let obj = self.get(index).unwrap();
                Id::retain_autoreleased(obj as *const T as *mut T).unwrap_unchecked()
            };
            // SAFETY: The object has correct ownership.
            unsafe { self.replaceObjectAtIndex_withObject(index, &obj) };
            old_obj
        }

        #[doc(alias = "removeObjectAtIndex:")]
        pub fn remove(&mut self, index: usize) -> Id<T, O> {
            let obj = if let Some(obj) = self.get(index) {
                unsafe { Id::retain_autoreleased(obj as *const T as *mut T).unwrap_unchecked() }
            } else {
                panic!("removal index should be < len");
            };
            // SAFETY: The index is checked to be in bounds.
            unsafe { self.removeObjectAtIndex(index) };
            obj
        }

        #[doc(alias = "removeLastObject")]
        pub fn pop(&mut self) -> Option<Id<T, O>> {
            self.last()
                .map(|obj| unsafe { Id::retain(obj as *const T as *mut T).unwrap_unchecked() })
                .map(|obj| {
                    // SAFETY: `Self::last` just checked that there is an object
                    unsafe { self.removeLastObject() };
                    obj
                })
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
    }
);

// Copying only possible when ItemOwnership = Shared

/// This is implemented as a shallow copy.
unsafe impl<T: Message> NSCopying for NSMutableArray<T, Shared> {
    type Ownership = Shared;
    type Output = NSArray<T, Shared>;
}

/// This is implemented as a shallow copy.
unsafe impl<T: Message> NSMutableCopying for NSMutableArray<T, Shared> {
    type Output = NSMutableArray<T, Shared>;
}

impl<T: Message> alloc::borrow::ToOwned for NSMutableArray<T, Shared> {
    type Owned = Id<NSMutableArray<T, Shared>, Owned>;
    fn to_owned(&self) -> Self::Owned {
        self.mutable_copy()
    }
}

unsafe impl<T: Message, O: Ownership> NSFastEnumeration2 for NSMutableArray<T, O> {
    type Item = T;
}

impl<'a, T: Message, O: Ownership> IntoIterator for &'a NSMutableArray<T, O> {
    type Item = &'a T;
    type IntoIter = NSFastEnumerator2<'a, NSMutableArray<T, O>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_fast()
    }
}

impl<T: Message, O: Ownership> Extend<Id<T, O>> for NSMutableArray<T, O> {
    fn extend<I: IntoIterator<Item = Id<T, O>>>(&mut self, iter: I) {
        let iterator = iter.into_iter();
        iterator.for_each(move |item| self.push(item));
    }
}

impl<T: Message, O: Ownership> Index<usize> for NSMutableArray<T, O> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

impl<T: Message> IndexMut<usize> for NSMutableArray<T, Owned> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

impl<T: Message, O: Ownership> DefaultId for NSMutableArray<T, O> {
    type Ownership = Owned;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::*;
    use crate::Foundation::NSString;
    use objc2::rc::{__RcTestObject, __ThreadTestData, autoreleasepool};

    #[test]
    fn test_adding() {
        let mut array = NSMutableArray::new();
        let obj1 = __RcTestObject::new();
        let obj2 = __RcTestObject::new();
        let mut expected = __ThreadTestData::current();

        array.push(obj1);
        expected.retain += 1;
        expected.release += 1;
        expected.assert_current();
        assert_eq!(array.len(), 1);
        assert_eq!(array.get(0), array.get(0));

        array.insert(0, obj2);
        expected.retain += 1;
        expected.release += 1;
        expected.assert_current();
        assert_eq!(array.len(), 2);
    }

    #[test]
    fn test_replace() {
        let mut array = NSMutableArray::new();
        let obj1 = __RcTestObject::new();
        let obj2 = __RcTestObject::new();
        array.push(obj1);
        let mut expected = __ThreadTestData::current();

        let old_obj = array.replace(0, obj2);
        expected.retain += 2;
        expected.release += 2;
        expected.assert_current();
        assert_ne!(&*old_obj, array.get(0).unwrap());
    }

    #[test]
    fn test_remove() {
        let mut array = NSMutableArray::new();
        for _ in 0..4 {
            array.push(__RcTestObject::new());
        }
        let mut expected = __ThreadTestData::current();

        let _obj = array.remove(1);
        expected.retain += 1;
        expected.release += 1;
        expected.assert_current();
        assert_eq!(array.len(), 3);

        let _obj = array.pop();
        expected.retain += 1;
        expected.release += 1;
        expected.assert_current();
        assert_eq!(array.len(), 2);

        array.removeAllObjects();
        expected.release += 2;
        expected.dealloc += 2;
        expected.assert_current();
        assert_eq!(array.len(), 0);
    }

    #[test]
    fn test_sort() {
        let strings = vec![NSString::from_str("hello"), NSString::from_str("hi")];
        let mut strings = NSMutableArray::from_vec(strings);

        autoreleasepool(|pool| {
            strings.sort_by(|s1, s2| s1.as_str(pool).len().cmp(&s2.as_str(pool).len()));
            assert_eq!(strings[0].as_str(pool), "hi");
            assert_eq!(strings[1].as_str(pool), "hello");
        });
    }
}
