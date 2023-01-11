#![cfg(feature = "Foundation_NSSet")]
use alloc::vec::Vec;
use core::fmt;
use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::rc::{DefaultId, Id, Owned, Ownership, Shared, SliceId};
use objc2::{extern_methods, msg_send, msg_send_id, ClassType, Message};

use crate::Foundation::{self, NSSet};

// SAFETY: Same as NSArray<T, O>
unsafe impl<T: Message + Sync + Send> Sync for NSSet<T, Shared> {}
unsafe impl<T: Message + Sync + Send> Send for NSSet<T, Shared> {}
unsafe impl<T: Message + Sync> Sync for NSSet<T, Owned> {}
unsafe impl<T: Message + Send> Send for NSSet<T, Owned> {}

#[cfg(feature = "Foundation_NSMutableSet")]
unsafe impl<T: Message + Sync + Send> Sync for Foundation::NSMutableSet<T, Shared> {}
#[cfg(feature = "Foundation_NSMutableSet")]
unsafe impl<T: Message + Sync + Send> Send for Foundation::NSMutableSet<T, Shared> {}
#[cfg(feature = "Foundation_NSMutableSet")]
unsafe impl<T: Message + Sync> Sync for Foundation::NSMutableSet<T, Owned> {}
#[cfg(feature = "Foundation_NSMutableSet")]
unsafe impl<T: Message + Send> Send for Foundation::NSMutableSet<T, Owned> {}

// SAFETY: Same as NSArray<T, O>
impl<T: Message + RefUnwindSafe, O: Ownership> RefUnwindSafe for NSSet<T, O> {}
impl<T: Message + RefUnwindSafe> UnwindSafe for NSSet<T, Shared> {}
impl<T: Message + UnwindSafe> UnwindSafe for NSSet<T, Owned> {}

#[cfg(feature = "Foundation_NSMutableSet")]
impl<T: Message + RefUnwindSafe, O: Ownership> RefUnwindSafe for Foundation::NSMutableSet<T, O> {}
#[cfg(feature = "Foundation_NSMutableSet")]
impl<T: Message + RefUnwindSafe> UnwindSafe for Foundation::NSMutableSet<T, Shared> {}
#[cfg(feature = "Foundation_NSMutableSet")]
impl<T: Message + UnwindSafe> UnwindSafe for Foundation::NSMutableSet<T, Owned> {}

#[track_caller]
pub(crate) unsafe fn with_objects<T: Message + ?Sized, R: ClassType, O: Ownership>(
    objects: &[&T],
) -> Id<R, O> {
    unsafe {
        msg_send_id![
            R::alloc(),
            initWithObjects: objects.as_ptr(),
            count: objects.len()
        ]
    }
}

extern_methods!(
    unsafe impl<T: Message, O: Ownership> NSSet<T, O> {
        /// Creates an empty [`NSSet`].
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let set = NSSet::<NSString>::new();
        /// ```
        // SAFETY:
        // - `new` may not create a new object, but instead return a shared
        //   instance. We remedy this by returning `Id<Self, Shared>`.
        // - `O` don't actually matter here! E.g. `NSSet<T, Owned>` is
        //   perfectly legal, since the set doesn't have any elements, and
        //   hence the notion of ownership over the elements is void.
        #[method_id(new)]
        pub fn new() -> Id<Self, Shared>;

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
        pub fn from_vec(vec: Vec<Id<T, O>>) -> Id<Self, O> {
            // SAFETY:
            // When we know that we have ownership over the variables, we also
            // know that there cannot be another set in existence with the same
            // objects, so `Id<NSSet<T, Owned>, Owned>` is safe to return when
            // we receive `Vec<Id<T, Owned>>`.
            unsafe { with_objects(vec.as_slice_ref()) }
        }

        /// Returns the number of elements in the set.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let strs = ["one", "two", "three"].map(NSString::from_str);
        /// let set = NSSet::from_slice(&strs);
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
        /// let set = NSSet::from_slice(&strs);
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
        /// let set = NSSet::from_slice(&strs);
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

        /// Returns a [`Vec`] containing the set's elements, consuming the set.
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
        /// let vec = NSSet::into_vec(set);
        /// assert_eq!(vec.len(), 3);
        /// ```
        pub fn into_vec(set: Id<Self, O>) -> Vec<Id<T, O>> {
            set.into_iter()
                .map(|obj| unsafe { Id::retain(obj as *const T as *mut T).unwrap_unchecked() })
                .collect()
        }
    }

    unsafe impl<T: Message> NSSet<T, Shared> {
        /// Creates an [`NSSet`] from a slice.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let strs = ["one", "two", "three"].map(NSString::from_str);
        /// let set = NSSet::from_slice(&strs);
        /// ```
        pub fn from_slice(slice: &[Id<T, Shared>]) -> Id<Self, Shared> {
            // SAFETY:
            // Taking `&T` would not be sound, since the `&T` could come from
            // an `Id<T, Owned>` that would now no longer be owned!
            //
            // We always return `Id<NSSet<T, Shared>, Shared>` because the
            // elements are shared.
            unsafe { with_objects(slice.as_slice_ref()) }
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
        /// let set = NSSet::from_slice(&nums.map(NSNumber::new_i32));
        ///
        /// assert_eq!(set.to_array().len(), 3);
        /// assert!(set.to_array().iter().all(|i| nums.contains(&i.as_i32())));
        /// ```
        #[doc(alias = "allObjects")]
        #[cfg(feature = "Foundation_NSArray")]
        pub fn to_array(&self) -> Id<Foundation::NSArray<T, Shared>, Shared> {
            // SAFETY: The set's elements are shared
            unsafe { self.allObjects() }
        }
    }

    // We're explicit about `T` being `PartialEq` for these methods because the
    // set compares the input value(s) with elements in the set
    // For comparison: Rust's HashSet requires similar methods to be `Hash` + `Eq`
    unsafe impl<T: Message + PartialEq, O: Ownership> NSSet<T, O> {
        /// Returns `true` if the set contains a value.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        /// use icrate::ns_string;
        ///
        /// let strs = ["one", "two", "three"].map(NSString::from_str);
        /// let set = NSSet::from_slice(&strs);
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
        /// let set = NSSet::from_slice(&strs);
        /// assert_eq!(set.get(ns_string!("one")), Some(&*strs[0]));
        /// assert_eq!(set.get(ns_string!("four")), None);
        /// ```
        #[doc(alias = "member:")]
        #[method(member:)]
        pub fn get(&self, value: &T) -> Option<&T>;

        /// Returns `true` if the set is a subset of another, i.e., `other`
        /// contains at least all the values in `self`.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let set1 = NSSet::from_slice(&["one", "two"].map(NSString::from_str));
        /// let set2 = NSSet::from_slice(&["one", "two", "three"].map(NSString::from_str));
        ///
        /// assert!(set1.is_subset(&set2));
        /// assert!(!set2.is_subset(&set1));
        /// ```
        #[doc(alias = "isSubsetOfSet:")]
        #[method(isSubsetOfSet:)]
        pub fn is_subset(&self, other: &NSSet<T, O>) -> bool;

        /// Returns `true` if the set is a superset of another, i.e., `self`
        /// contains at least all the values in `other`.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let set1 = NSSet::from_slice(&["one", "two"].map(NSString::from_str));
        /// let set2 = NSSet::from_slice(&["one", "two", "three"].map(NSString::from_str));
        ///
        /// assert!(!set1.is_superset(&set2));
        /// assert!(set2.is_superset(&set1));
        /// ```
        pub fn is_superset(&self, other: &NSSet<T, O>) -> bool {
            other.is_subset(self)
        }

        #[method(intersectsSet:)]
        fn intersects_set(&self, other: &NSSet<T, O>) -> bool;

        /// Returns `true` if `self` has no elements in common with `other`.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSSet, NSString};
        ///
        /// let set1 = NSSet::from_slice(&["one", "two"].map(NSString::from_str));
        /// let set2 = NSSet::from_slice(&["one", "two", "three"].map(NSString::from_str));
        /// let set3 = NSSet::from_slice(&["four", "five", "six"].map(NSString::from_str));
        ///
        /// assert!(!set1.is_disjoint(&set2));
        /// assert!(set1.is_disjoint(&set3));
        /// assert!(set2.is_disjoint(&set3));
        /// ```
        pub fn is_disjoint(&self, other: &NSSet<T, O>) -> bool {
            !self.intersects_set(other)
        }
    }
);

unsafe impl<T: Message, O: Ownership> Foundation::NSFastEnumeration2 for NSSet<T, O> {
    type Item = T;
}

impl<'a, T: Message, O: Ownership> IntoIterator for &'a NSSet<T, O> {
    type Item = &'a T;
    type IntoIter = Foundation::NSFastEnumerator2<'a, NSSet<T, O>>;

    fn into_iter(self) -> Self::IntoIter {
        use Foundation::NSFastEnumeration2;
        self.iter_fast()
    }
}

impl<T: Message, O: Ownership> DefaultId for NSSet<T, O> {
    type Ownership = Shared;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

#[cfg(feature = "Foundation_NSEnumerator")]
impl<T: fmt::Debug + Message, O: Ownership> fmt::Debug for NSSet<T, O> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Foundation::NSFastEnumeration2;
        f.debug_set().entries(self.iter_fast()).finish()
    }
}
