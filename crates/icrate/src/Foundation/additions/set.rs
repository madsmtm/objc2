#![cfg(feature = "Foundation_NSSet")]
use alloc::vec::Vec;
use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::msg_send;
use objc2::mutability::IsRetainable;
use objc2::rc::DefaultId;

use super::util;
use crate::common::*;
use crate::Foundation::{self, NSSet};

extern_methods!(
    unsafe impl<T: Message> NSSet<T> {
        /// Creates an empty [`NSSet`].
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let set = NSSet::<NSString>::new();
        /// ```
        #[method_id(new)]
        pub fn new() -> Id<Self>;

        /// Creates an [`NSSet`] from a vector.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let strs = ["one", "two", "three"].map(NSString::from_str).to_vec();
        /// let set = NSSet::from_vec(strs);
        /// ```
        pub fn from_vec(mut vec: Vec<Id<T>>) -> Id<Self> {
            let len = vec.len();
            let ptr = util::id_ptr_cast(vec.as_mut_ptr());
            // SAFETY: Same as `NSArray::from_vec`.
            unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
        }

        /// Creates an [`NSSet`] from a slice of `Id`s.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let strs = ["one", "two", "three"].map(NSString::from_str);
        /// let set = NSSet::from_id_slice(&strs);
        /// ```
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

        /// Returns the number of elements in the set.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let strs = ["one", "two", "three"].map(NSString::from_str);
        /// let set = NSSet::from_id_slice(&strs);
        /// assert_eq!(set.len(), 3);
        /// ```
        #[doc(alias = "count")]
        pub fn len(&self) -> usize {
            self.count()
        }

        /// Returns `true` if the set contains no elements.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let set = NSSet::<NSString>::new();
        /// assert!(set.is_empty());
        /// ```
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Returns a reference to one of the objects in the set, or `None` if
        /// the set is empty.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let strs = ["one", "two", "three"].map(NSString::from_str);
        /// let set = NSSet::from_id_slice(&strs);
        /// let any = set.get_any().unwrap();
        /// assert!(any == &*strs[0] || any == &*strs[1] || any == &*strs[2]);
        /// ```
        #[doc(alias = "anyObject")]
        #[method(anyObject)]
        pub fn get_any(&self) -> Option<&T>;

        /// An iterator visiting all elements in arbitrary order.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let strs = ["one", "two", "three"].map(NSString::from_str);
        /// let set = NSSet::from_id_slice(&strs);
        /// for s in set.iter() {
        ///     println!("{s}");
        /// }
        /// ```
        #[doc(alias = "objectEnumerator")]
        #[cfg(feature = "Foundation_NSEnumerator")]
        pub fn iter(&self) -> Foundation::NSEnumerator2<'_, T> {
            unsafe {
                let result = msg_send![self, objectEnumerator];
                Foundation::NSEnumerator2::from_ptr(result)
            }
        }

        /// Returns a [`Vec`] containing the set's elements.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSMutableString, NSSet};
        ///
        /// let strs = vec![
        ///     NSMutableString::from_str("one"),
        ///     NSMutableString::from_str("two"),
        ///     NSMutableString::from_str("three"),
        /// ];
        /// let set = NSSet::from_vec(strs);
        /// let vec = set.to_vec();
        /// assert_eq!(vec.len(), 3);
        /// ```
        pub fn to_vec(&self) -> Vec<&T> {
            self.into_iter().collect()
        }

        #[doc(alias = "getObjects:range:")]
        pub fn to_vec_retained(&self) -> Vec<Id<T>>
        where
            T: IsIdCloneable,
        {
            // SAFETY: The objects are stored in the set
            self.into_iter()
                .map(|obj| unsafe { util::collection_retain_id(obj) })
                .collect()
        }

        /// Returns an [`NSArray`] containing the set's elements, or an empty
        /// array if the set is empty.
        ///
        /// [`NSArray`]: crate::Foundation::NSArray
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSNumber, NSSet, NSString};
        ///
        /// let nums = [1, 2, 3];
        /// let set = NSSet::from_id_slice(&nums.map(NSNumber::new_i32));
        ///
        /// assert_eq!(set.to_array().len(), 3);
        /// assert!(set.to_array().iter().all(|i| nums.contains(&i.as_i32())));
        /// ```
        #[doc(alias = "allObjects")]
        #[cfg(feature = "Foundation_NSArray")]
        pub fn to_array(&self) -> Id<Foundation::NSArray<T>>
        where
            T: IsIdCloneable,
        {
            // SAFETY: The `T: IsIdCloneable` bound ensures that it is safe to
            // create what is effectively a copy from an `&self` reference.
            //
            // Could be implemented as:
            //    NSArray::from_vec(self.to_vec_retained())
            unsafe { self.allObjects() }
        }
    }

    // We're explicit about `T` being `PartialEq` for these methods because the
    // set compares the input value(s) with elements in the set
    // For comparison: Rust's HashSet requires similar methods to be `Hash` + `Eq`
    unsafe impl<T: Message + PartialEq> NSSet<T> {
        /// Returns `true` if the set contains a value.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        /// use icrate::ns_string;
        ///
        /// let strs = ["one", "two", "three"].map(NSString::from_str);
        /// let set = NSSet::from_id_slice(&strs);
        /// assert!(set.contains(ns_string!("one")));
        /// ```
        #[doc(alias = "containsObject:")]
        pub fn contains(&self, value: &T) -> bool {
            unsafe { self.containsObject(value) }
        }

        /// Returns a reference to the value in the set, if any, that is equal
        /// to the given value.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        /// use icrate::ns_string;
        ///
        /// let strs = ["one", "two", "three"].map(NSString::from_str);
        /// let set = NSSet::from_id_slice(&strs);
        /// assert_eq!(set.get(ns_string!("one")), Some(&*strs[0]));
        /// assert_eq!(set.get(ns_string!("four")), None);
        /// ```
        #[doc(alias = "member:")]
        #[method(member:)]
        pub fn get(&self, value: &T) -> Option<&T>;

        #[doc(alias = "member:")]
        pub fn get_retained(&self, value: &T) -> Option<Id<T>>
        where
            T: IsIdCloneable,
        {
            self.get(value)
                .map(|obj| unsafe { util::collection_retain_id(obj) })
        }

        // Note: No `get_mut` method exposed on sets, since their objects'
        // hashes are supposed to be immutable.

        /// Returns `true` if the set is a subset of another, i.e., `other`
        /// contains at least all the values in `self`.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let set1 = NSSet::from_id_slice(&["one", "two"].map(NSString::from_str));
        /// let set2 = NSSet::from_id_slice(&["one", "two", "three"].map(NSString::from_str));
        ///
        /// assert!(set1.is_subset(&set2));
        /// assert!(!set2.is_subset(&set1));
        /// ```
        #[doc(alias = "isSubsetOfSet:")]
        #[method(isSubsetOfSet:)]
        pub fn is_subset(&self, other: &NSSet<T>) -> bool;

        /// Returns `true` if the set is a superset of another, i.e., `self`
        /// contains at least all the values in `other`.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let set1 = NSSet::from_id_slice(&["one", "two"].map(NSString::from_str));
        /// let set2 = NSSet::from_id_slice(&["one", "two", "three"].map(NSString::from_str));
        ///
        /// assert!(!set1.is_superset(&set2));
        /// assert!(set2.is_superset(&set1));
        /// ```
        pub fn is_superset(&self, other: &NSSet<T>) -> bool {
            other.is_subset(self)
        }

        #[method(intersectsSet:)]
        fn intersects_set(&self, other: &NSSet<T>) -> bool;

        /// Returns `true` if `self` has no elements in common with `other`.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let set1 = NSSet::from_id_slice(&["one", "two"].map(NSString::from_str));
        /// let set2 = NSSet::from_id_slice(&["one", "two", "three"].map(NSString::from_str));
        /// let set3 = NSSet::from_id_slice(&["four", "five", "six"].map(NSString::from_str));
        ///
        /// assert!(!set1.is_disjoint(&set2));
        /// assert!(set1.is_disjoint(&set3));
        /// assert!(set2.is_disjoint(&set3));
        /// ```
        pub fn is_disjoint(&self, other: &NSSet<T>) -> bool {
            !self.intersects_set(other)
        }
    }
);

unsafe impl<T: Message> Foundation::NSFastEnumeration2 for NSSet<T> {
    type Item = T;
}

impl<'a, T: Message> IntoIterator for &'a NSSet<T> {
    type Item = &'a T;
    type IntoIter = Foundation::NSFastEnumerator2<'a, NSSet<T>>;

    fn into_iter(self) -> Self::IntoIter {
        use Foundation::NSFastEnumeration2;
        self.iter_fast()
    }
}

impl<T: Message> DefaultId for NSSet<T> {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

#[cfg(feature = "Foundation_NSEnumerator")]
impl<T: fmt::Debug + Message> fmt::Debug for NSSet<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Foundation::NSFastEnumeration2;
        f.debug_set().entries(self.iter_fast()).finish()
    }
}
