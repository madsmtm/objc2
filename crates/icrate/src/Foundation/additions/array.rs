#![cfg(feature = "Foundation_NSArray")]
use alloc::vec::Vec;
use core::fmt;
use core::mem;
use core::ops::{Index, IndexMut, Range};
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::NonNull;

use objc2::rc::{DefaultId, Id, Owned, Ownership, Shared, SliceId};
use objc2::runtime::Object;
use objc2::{extern_methods, msg_send, msg_send_id, ClassType, Message};

use crate::Foundation::{self, NSArray};

// SAFETY: Same as Id<T, O> (what NSArray and NSMutableArray effectively store).
unsafe impl<T: Message + Sync + Send> Sync for NSArray<T, Shared> {}
unsafe impl<T: Message + Sync + Send> Send for NSArray<T, Shared> {}
unsafe impl<T: Message + Sync> Sync for NSArray<T, Owned> {}
unsafe impl<T: Message + Send> Send for NSArray<T, Owned> {}

#[cfg(feature = "Foundation_NSMutableArray")]
unsafe impl<T: Message + Sync + Send> Sync for Foundation::NSMutableArray<T, Shared> {}
#[cfg(feature = "Foundation_NSMutableArray")]
unsafe impl<T: Message + Sync + Send> Send for Foundation::NSMutableArray<T, Shared> {}
#[cfg(feature = "Foundation_NSMutableArray")]
unsafe impl<T: Message + Sync> Sync for Foundation::NSMutableArray<T, Owned> {}
#[cfg(feature = "Foundation_NSMutableArray")]
unsafe impl<T: Message + Send> Send for Foundation::NSMutableArray<T, Owned> {}

// Also same as Id<T, O>
impl<T: Message + RefUnwindSafe, O: Ownership> RefUnwindSafe for NSArray<T, O> {}
impl<T: Message + RefUnwindSafe> UnwindSafe for NSArray<T, Shared> {}
impl<T: Message + UnwindSafe> UnwindSafe for NSArray<T, Owned> {}

#[cfg(feature = "Foundation_NSMutableArray")]
impl<T: Message + RefUnwindSafe, O: Ownership> RefUnwindSafe for Foundation::NSMutableArray<T, O> {}
#[cfg(feature = "Foundation_NSMutableArray")]
impl<T: Message + RefUnwindSafe> UnwindSafe for Foundation::NSMutableArray<T, Shared> {}
#[cfg(feature = "Foundation_NSMutableArray")]
impl<T: Message + UnwindSafe> UnwindSafe for Foundation::NSMutableArray<T, Owned> {}

#[track_caller]
pub(crate) unsafe fn with_objects<T: Message + ?Sized, R: ClassType, O: Ownership>(
    objects: &[&T],
) -> Id<R, O> {
    unsafe {
        msg_send_id![
            R::alloc(),
            initWithObjects: objects.as_ptr(),
            count: objects.len(),
        ]
    }
}

extern_methods!(
    /// Generic creation methods.
    unsafe impl<T: Message, O: Ownership> NSArray<T, O> {
        /// Get an empty array.
        // SAFETY:
        // - `new` may not create a new object, but instead return a shared
        //   instance. We remedy this by returning `Id<Self, Shared>`.
        // - `O` don't actually matter here! E.g. `NSArray<T, Owned>` is
        //   perfectly legal, since the array doesn't have any elements, and
        //   hence the notion of ownership over the elements is void.
        #[method_id(new)]
        pub fn new() -> Id<Self, Shared>;

        pub fn from_vec(vec: Vec<Id<T, O>>) -> Id<Self, O> {
            // SAFETY:
            // `initWithObjects:` may choose to deduplicate arrays (I could
            // imagine it having a special case for arrays with one `NSNumber`
            // object), and returning mutable references to those would be
            // unsound!
            // However, when we know that we have ownership over the variables, we
            // also know that there cannot be another array in existence with the
            // same objects, so `Id<NSArray<T, Owned>, Owned>` is safe to return.
            //
            // In essence, we can choose between always returning `Id<T, Shared>`
            // or `Id<T, O>`, and the latter is probably the most useful, as we
            // would like to know when we're the only owner of the array, to
            // allow mutation of the array's items.
            unsafe { with_objects(vec.as_slice_ref()) }
        }
    }

    /// Creation methods that produce shared arrays.
    unsafe impl<T: Message> NSArray<T, Shared> {
        pub fn from_slice(slice: &[Id<T, Shared>]) -> Id<Self, Shared> {
            // SAFETY: Taking `&T` would not be sound, since the `&T` could come
            // from an `Id<T, Owned>` that would now no longer be owned!
            //
            // (Note that NSArray internally retains all the objects it is given,
            // effectively making the safety requirements the same as
            // `Id::retain`).
            unsafe { with_objects(slice.as_slice_ref()) }
        }
    }

    /// Generic accessor methods.
    unsafe impl<T: Message, O: Ownership> NSArray<T, O> {
        #[doc(alias = "count")]
        pub fn len(&self) -> usize {
            self.count()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        #[method(objectAtIndex:)]
        unsafe fn get_unchecked(&self, index: usize) -> &T;

        #[doc(alias = "objectAtIndex:")]
        pub fn get(&self, index: usize) -> Option<&T> {
            // TODO: Replace this check with catching the thrown NSRangeException
            if index < self.len() {
                // SAFETY: The index is checked to be in bounds.
                Some(unsafe { self.get_unchecked(index) })
            } else {
                None
            }
        }

        #[doc(alias = "firstObject")]
        #[method(firstObject)]
        pub fn first(&self) -> Option<&T>;

        #[doc(alias = "lastObject")]
        #[method(lastObject)]
        pub fn last(&self) -> Option<&T>;

        #[doc(alias = "objectEnumerator")]
        #[cfg(feature = "Foundation_NSEnumerator")]
        pub fn iter(&self) -> Foundation::NSEnumerator2<'_, T> {
            unsafe {
                let result: *mut Object = msg_send![self, objectEnumerator];
                Foundation::NSEnumerator2::from_ptr(result)
            }
        }

        unsafe fn objects_in_range_unchecked(&self, range: Range<usize>) -> Vec<&T> {
            let range = Foundation::NSRange::from(range);
            let mut vec: Vec<NonNull<T>> = Vec::with_capacity(range.length);
            unsafe {
                self.getObjects_range(NonNull::new(vec.as_mut_ptr()).unwrap(), range);
                vec.set_len(range.length);
                mem::transmute(vec)
            }
        }

        pub fn objects_in_range(&self, range: Range<usize>) -> Option<Vec<&T>> {
            if range.end > self.len() {
                return None;
            }
            // SAFETY: Just checked that the range is in bounds
            Some(unsafe { self.objects_in_range_unchecked(range) })
        }

        pub fn to_vec(&self) -> Vec<&T> {
            // SAFETY: The range is know to be in bounds
            unsafe { self.objects_in_range_unchecked(0..self.len()) }
        }

        // TODO: Take Id<Self, Self::ItemOwnership> ?
        pub fn into_vec(array: Id<Self, Owned>) -> Vec<Id<T, O>> {
            array
                .to_vec()
                .into_iter()
                .map(|obj| unsafe { Id::retain(obj as *const T as *mut T).unwrap_unchecked() })
                .collect()
        }
    }

    /// Accessor methods that work on shared arrays.
    unsafe impl<T: Message> NSArray<T, Shared> {
        #[doc(alias = "objectAtIndex:")]
        pub fn get_retained(&self, index: usize) -> Id<T, Shared> {
            let obj = self.get(index).unwrap();
            // SAFETY: The object is originally shared (see `where` bound).
            unsafe { Id::retain_autoreleased(obj as *const T as *mut T).unwrap_unchecked() }
        }

        pub fn to_shared_vec(&self) -> Vec<Id<T, Shared>> {
            self.to_vec()
                .into_iter()
                .map(|obj| unsafe { Id::retain(obj as *const T as *mut T).unwrap_unchecked() })
                .collect()
        }
    }

    /// Accessor methods that work on owned arrays.
    unsafe impl<T: Message> NSArray<T, Owned> {
        #[doc(alias = "objectAtIndex:")]
        pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            // TODO: Replace this check with catching the thrown NSRangeException
            if index < self.len() {
                // SAFETY: The index is checked to be in bounds.
                Some(unsafe { msg_send![self, objectAtIndex: index] })
            } else {
                None
            }
        }

        #[doc(alias = "firstObject")]
        #[method(firstObject)]
        pub fn first_mut(&mut self) -> Option<&mut T>;

        #[doc(alias = "lastObject")]
        #[method(lastObject)]
        pub fn last_mut(&mut self) -> Option<&mut T>;
    }
);

unsafe impl<T: Message, O: Ownership> Foundation::NSFastEnumeration2 for NSArray<T, O> {
    type Item = T;
}

impl<'a, T: Message, O: Ownership> IntoIterator for &'a NSArray<T, O> {
    type Item = &'a T;
    type IntoIter = Foundation::NSFastEnumerator2<'a, NSArray<T, O>>;

    fn into_iter(self) -> Self::IntoIter {
        use Foundation::NSFastEnumeration2;
        self.iter_fast()
    }
}

impl<T: Message, O: Ownership> Index<usize> for NSArray<T, O> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

impl<T: Message> IndexMut<usize> for NSArray<T, Owned> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

impl<T: Message, O: Ownership> DefaultId for NSArray<T, O> {
    type Ownership = Shared;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

impl<T: fmt::Debug + Message, O: Ownership> fmt::Debug for NSArray<T, O> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Foundation::NSFastEnumeration2;
        f.debug_list().entries(self.iter_fast()).finish()
    }
}
