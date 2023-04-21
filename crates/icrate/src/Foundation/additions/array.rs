#![cfg(feature = "Foundation_NSArray")]
use alloc::vec::Vec;
use core::fmt;
use core::mem;
use core::ops::{Index, IndexMut, Range};
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::msg_send;
use objc2::mutability::{IsMutable, IsRetainable};
use objc2::rc::DefaultId;

use super::util;
use crate::common::*;
use crate::Foundation::{self, NSArray};

extern_methods!(
    /// Creation methods.
    unsafe impl<T: Message> NSArray<T> {
        /// Get an empty array.
        #[method_id(new)]
        pub fn new() -> Id<Self>;

        pub fn from_vec(mut vec: Vec<Id<T>>) -> Id<Self> {
            // We intentionally extract the length before we access the
            // pointer as mutable, to not invalidate that mutable pointer.
            let len = vec.len();
            let ptr = util::id_ptr_cast(vec.as_mut_ptr());
            // SAFETY: We've consumed the `Id<T>`s, which means that we can
            // now safely take ownership (even if `T` is mutable).
            unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
            // The drop of `Vec` here would invalidate our mutable pointer,
            // except for the fact that we're using `UnsafeCell` in `Object`.
        }

        pub fn from_id_slice(slice: &[Id<T>]) -> Id<Self>
        where
            T: IsIdCloneable,
        {
            let len = slice.len();
            let ptr = util::id_ptr_cast_const(slice.as_ptr());
            // SAFETY: Because of the `T: IsIdCloneable` bound, and since we
            // take `&[Id<T>]` (effectively `&Id<T>`), we are allowed to give
            // the slice to Objective-C, which will retain it internally.
            //
            // Faster version of:
            //     Self::from_vec(slice.iter().map(|obj| obj.clone()).collect())
            unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
        }

        pub fn from_slice(slice: &[&T]) -> Id<Self>
        where
            T: IsRetainable,
        {
            let len = slice.len();
            let ptr = util::ref_ptr_cast_const(slice.as_ptr());
            // SAFETY: Because of the `T: IsRetainable` bound, we are allowed
            // to give the slice to Objective-C, which will retain it
            // internally.
            //
            // Faster version of:
            //     Self::from_vec(slice.iter().map(|obj| obj.retain()).collect())
            unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
        }
    }

    /// Accessor methods.
    unsafe impl<T: Message> NSArray<T> {
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

        #[doc(alias = "objectAtIndex:")]
        pub fn get_retained(&self, index: usize) -> Option<Id<T>>
        where
            T: IsIdCloneable,
        {
            // SAFETY: The object is stored in the array
            self.get(index)
                .map(|obj| unsafe { util::collection_retain_id(obj) })
        }

        #[method(objectAtIndex:)]
        unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T;

        #[doc(alias = "objectAtIndex:")]
        pub fn get_mut(&mut self, index: usize) -> Option<&mut T>
        where
            T: IsMutable,
        {
            // TODO: Replace this check with catching the thrown NSRangeException
            if index < self.len() {
                // SAFETY: The index is checked to be in bounds, and the
                // reference is safe as mutable because of the `T: IsMutable`
                // bound.
                Some(unsafe { self.get_unchecked_mut(index) })
            } else {
                None
            }
        }

        #[doc(alias = "firstObject")]
        #[method(firstObject)]
        pub fn first(&self) -> Option<&T>;

        #[doc(alias = "firstObject")]
        pub fn first_retained(&self) -> Option<Id<T>>
        where
            T: IsIdCloneable,
        {
            // SAFETY: The object is stored in the array
            self.first()
                .map(|obj| unsafe { util::collection_retain_id(obj) })
        }

        #[doc(alias = "firstObject")]
        #[method(firstObject)]
        pub fn first_mut(&mut self) -> Option<&mut T>
        where
            T: IsMutable;

        #[doc(alias = "lastObject")]
        #[method(lastObject)]
        pub fn last(&self) -> Option<&T>;

        #[doc(alias = "lastObject")]
        pub fn last_retained(&self) -> Option<Id<T>>
        where
            T: IsIdCloneable,
        {
            // SAFETY: The object is stored in the array
            self.last()
                .map(|obj| unsafe { util::collection_retain_id(obj) })
        }

        #[doc(alias = "lastObject")]
        #[method(lastObject)]
        pub fn last_mut(&mut self) -> Option<&mut T>
        where
            T: IsMutable;

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

        #[doc(alias = "getObjects:range:")]
        pub fn objects_in_range(&self, range: Range<usize>) -> Option<Vec<&T>> {
            if range.end > self.len() {
                return None;
            }
            // SAFETY: Just checked that the range is in bounds
            Some(unsafe { self.objects_in_range_unchecked(range) })
        }

        #[doc(alias = "getObjects:range:")]
        pub fn to_vec(&self) -> Vec<&T> {
            // SAFETY: The range is know to be in bounds
            unsafe { self.objects_in_range_unchecked(0..self.len()) }
        }

        #[doc(alias = "getObjects:range:")]
        pub fn to_vec_retained(&self) -> Vec<Id<T>>
        where
            T: IsIdCloneable,
        {
            // SAFETY: The objects are stored in the array
            self.to_vec()
                .into_iter()
                .map(|obj| unsafe { util::collection_retain_id(obj) })
                .collect()
        }

        // `fn into_vec(Id<NSArray>) -> Vec<Id<T>>` would not be safe, since
        // the array itself is unconditionally `IsIdCloneable`, even when
        // containing mutable elements, and hence we would be able to
        // duplicate those.
    }
);

unsafe impl<T: Message> Foundation::NSFastEnumeration2 for NSArray<T> {
    type Item = T;
}

impl<'a, T: Message> IntoIterator for &'a NSArray<T> {
    type Item = &'a T;
    type IntoIter = Foundation::NSFastEnumerator2<'a, NSArray<T>>;

    fn into_iter(self) -> Self::IntoIter {
        use Foundation::NSFastEnumeration2;
        self.iter_fast()
    }
}

impl<T: Message> Index<usize> for NSArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.get(index).unwrap()
    }
}

impl<T: IsMutable> IndexMut<usize> for NSArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).unwrap()
    }
}

impl<T: Message> DefaultId for NSArray<T> {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

impl<T: fmt::Debug + Message> fmt::Debug for NSArray<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Foundation::NSFastEnumeration2;
        f.debug_list().entries(self.iter_fast()).finish()
    }
}
