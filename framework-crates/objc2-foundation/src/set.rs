//! Utilities for the `NSSet` and `NSMutableSet` classes.
use alloc::vec::Vec;
#[cfg(feature = "NSEnumerator")]
use core::fmt;

use objc2::mutability::{IsIdCloneable, IsRetainable};
use objc2::rc::{Retained, RetainedFromIterator};
use objc2::{extern_methods, ClassType, Message};

#[cfg(feature = "NSEnumerator")]
use super::iter;
use super::util;
use crate::Foundation::{NSMutableSet, NSSet};

impl<T: Message> NSSet<T> {
    /// Returns the number of elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{ns_string, NSSet};
    ///
    /// let strs = [ns_string!("one"), ns_string!("two"), ns_string!("three")];
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
    /// use objc2_foundation::{NSSet, NSObject};
    ///
    /// let set = NSSet::<NSObject>::new();
    /// assert!(set.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Message> NSSet<T> {
    /// Creates an [`NSSet`] from a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSSet, NSString};
    ///
    /// let strs = ["one", "two", "three"].map(NSString::from_str).to_vec();
    /// let set = NSSet::from_vec(strs);
    /// ```
    pub fn from_vec(mut vec: Vec<Retained<T>>) -> Retained<Self> {
        let len = vec.len();
        let ptr = util::retained_ptr_cast(vec.as_mut_ptr());
        // SAFETY: Same as `NSArray::from_vec`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    /// Creates an [`NSSet`] from a slice of `Retained`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSSet, NSString};
    ///
    /// let strs = ["one", "two", "three"].map(NSString::from_str);
    /// let set = NSSet::from_id_slice(&strs);
    /// ```
    pub fn from_id_slice(slice: &[Retained<T>]) -> Retained<Self>
    where
        T: IsIdCloneable,
    {
        let len = slice.len();
        let ptr = util::retained_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_id_slice`
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn from_slice(slice: &[&T]) -> Retained<Self>
    where
        T: IsRetainable,
    {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_slice`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    /// Returns a [`Vec`] containing the set's elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSMutableString, NSSet};
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
    #[cfg(feature = "NSEnumerator")]
    pub fn to_vec(&self) -> Vec<&T> {
        self.into_iter().collect()
    }

    #[cfg(feature = "NSEnumerator")]
    pub fn to_vec_retained(&self) -> Vec<Retained<T>>
    where
        T: IsIdCloneable,
    {
        // SAFETY: The objects are stored in the set
        self.into_iter()
            .map(|obj| unsafe { util::collection_retain(obj) })
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
    /// use objc2_foundation::{NSNumber, NSSet, NSString};
    ///
    /// let nums = [1, 2, 3];
    /// let set = NSSet::from_id_slice(&nums.map(NSNumber::new_i32));
    ///
    /// assert_eq!(set.to_array().len(), 3);
    /// assert!(set.to_array().iter().all(|i| nums.contains(&i.as_i32())));
    /// ```
    #[doc(alias = "allObjects")]
    #[cfg(feature = "NSArray")]
    pub fn to_array(&self) -> Retained<crate::Foundation::NSArray<T>>
    where
        T: IsIdCloneable,
    {
        // SAFETY: The `T: IsIdCloneable` bound ensures that it is safe to
        // create what is effectively a copy of the collection from a `&self`
        // reference.
        //
        // Could be implemented as:
        //    NSArray::from_vec(self.to_vec_retained())
        unsafe { self.allObjects() }
    }
}

impl<T: Message> NSMutableSet<T> {
    /// Creates an [`NSMutableSet`] from a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSMutableSet, NSString};
    ///
    /// let strs = ["one", "two", "three"].map(NSString::from_str).to_vec();
    /// let set = NSMutableSet::from_vec(strs);
    /// ```
    pub fn from_vec(mut vec: Vec<Retained<T>>) -> Retained<Self> {
        let len = vec.len();
        let ptr = util::retained_ptr_cast(vec.as_mut_ptr());
        // SAFETY: Same as `NSArray::from_vec`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    /// Creates an [`NSMutableSet`] from a slice of `Retained`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSMutableSet, NSString};
    ///
    /// let strs = ["one", "two", "three"].map(NSString::from_str);
    /// let set = NSMutableSet::from_id_slice(&strs);
    /// ```
    pub fn from_id_slice(slice: &[Retained<T>]) -> Retained<Self>
    where
        T: IsIdCloneable,
    {
        let len = slice.len();
        let ptr = util::retained_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_id_slice`
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    pub fn from_slice(slice: &[&T]) -> Retained<Self>
    where
        T: IsRetainable,
    {
        let len = slice.len();
        let ptr = util::ref_ptr_cast_const(slice.as_ptr());
        // SAFETY: Same as `NSArray::from_slice`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    /// Returns a [`Vec`] containing the set's elements, consuming the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSMutableSet, NSMutableString};
    ///
    /// let strs = vec![
    ///     NSMutableString::from_str("one"),
    ///     NSMutableString::from_str("two"),
    ///     NSMutableString::from_str("three"),
    /// ];
    /// let set = NSMutableSet::from_vec(strs);
    /// let vec = NSMutableSet::into_vec(set);
    /// assert_eq!(vec.len(), 3);
    /// ```
    #[cfg(feature = "NSEnumerator")]
    pub fn into_vec(set: Retained<Self>) -> Vec<Retained<T>> {
        set.into_iter().collect()
    }
}

extern_methods!(
    unsafe impl<T: Message> NSSet<T> {
        /// Returns a reference to one of the objects in the set, or `None` if
        /// the set is empty.
        ///
        /// # Examples
        ///
        /// ```
        /// use objc2_foundation::{ns_string, NSSet};
        ///
        /// let strs = [ns_string!("one"), ns_string!("two"), ns_string!("three")];
        /// let set = NSSet::from_slice(&strs);
        /// let any = set.get_any().unwrap();
        /// assert!(any == &*strs[0] || any == &*strs[1] || any == &*strs[2]);
        /// ```
        #[doc(alias = "anyObject")]
        #[method(anyObject)]
        pub fn get_any(&self) -> Option<&T>;
    }

    unsafe impl<T: Message> NSSet<T> {
        /// Returns a reference to the value in the set, if any, that is equal
        /// to the given value.
        ///
        /// # Examples
        ///
        /// ```
        /// use objc2_foundation::{ns_string, NSSet};
        ///
        /// let strs = [ns_string!("one"), ns_string!("two"), ns_string!("three")];
        /// let set = NSSet::from_slice(&strs);
        /// assert_eq!(set.get(ns_string!("one")), Some(&*strs[0]));
        /// assert_eq!(set.get(ns_string!("four")), None);
        /// ```
        #[doc(alias = "member:")]
        #[method(member:)]
        pub fn get(&self, value: &T) -> Option<&T>;

        #[doc(alias = "member:")]
        pub fn get_retained(&self, value: &T) -> Option<Retained<T>>
        where
            T: IsIdCloneable,
        {
            self.get(value)
                .map(|obj| unsafe { util::collection_retain(obj) })
        }

        // Note: No `get_mut` method exposed on sets, since their objects'
        // hashes are immutable.
    }
);

impl<T: Message> NSMutableSet<T> {
    /// Add a value to the set. Returns whether the value was
    /// newly inserted.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature = "NSValue", doc = "```")]
    #[cfg_attr(not(feature = "NSValue"), doc = "```ignore")]
    /// use objc2_foundation::{NSNumber, NSMutableSet};
    ///
    /// let mut set = NSMutableSet::new();
    ///
    /// assert_eq!(set.insert(&*NSNumber::new_u32(42)), true);
    /// assert_eq!(set.insert(&*NSNumber::new_u32(42)), false);
    /// assert_eq!(set.len(), 1);
    /// ```
    #[doc(alias = "addObject:")]
    pub fn insert(&mut self, value: &T) -> bool
    where
        T: IsRetainable,
    {
        let contains_value = self.containsObject(value);
        // SAFETY: Because of the `T: IsRetainable` bound, it is safe for the
        // set to retain the object here.
        unsafe { self.addObject(value) };
        !contains_value
    }

    /// Add an `Retained` to the set. Returns whether the value was
    /// newly inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{NSMutableSet, NSString};
    ///
    /// let mut set = NSMutableSet::new();
    ///
    /// assert_eq!(set.insert_id(NSString::from_str("one")), true);
    /// assert_eq!(set.insert_id(NSString::from_str("one")), false);
    /// assert_eq!(set.len(), 1);
    /// ```
    #[doc(alias = "addObject:")]
    pub fn insert_id(&mut self, value: Retained<T>) -> bool {
        let contains_value = self.containsObject(&value);
        // SAFETY: We've consumed ownership of the object.
        unsafe { self.addObject(&value) };
        !contains_value
    }

    /// Removes a value from the set. Returns whether the value was present
    /// in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{ns_string, NSMutableSet, NSString};
    ///
    /// let mut set = NSMutableSet::new();
    ///
    /// set.insert_id(NSString::from_str("one"));
    /// assert_eq!(set.remove(ns_string!("one")), true);
    /// assert_eq!(set.remove(ns_string!("one")), false);
    /// ```
    #[doc(alias = "removeObject:")]
    pub fn remove(&mut self, value: &T) -> bool {
        let contains_value = self.containsObject(value);
        unsafe { self.removeObject(value) };
        contains_value
    }
}

impl<T: Message> NSSet<T> {
    /// An iterator visiting all elements in arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use objc2_foundation::{ns_string, NSSet};
    ///
    /// let set = NSSet::from_slice(&[ns_string!("one"), ns_string!("two"), ns_string!("three")]);
    /// for s in &set {
    ///     println!("{s}");
    /// }
    /// ```
    #[doc(alias = "objectEnumerator")]
    #[cfg(feature = "NSEnumerator")]
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(super::iter::Iter::new(self))
    }

    #[doc(alias = "objectEnumerator")]
    #[cfg(feature = "NSEnumerator")]
    #[inline]
    pub fn iter_retained(&self) -> IterRetained<'_, T>
    where
        T: IsIdCloneable,
    {
        IterRetained(super::iter::IterRetained::new(self))
    }
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<T: Message> iter::FastEnumerationHelper for NSSet<T> {
    type Item = T;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

#[cfg(feature = "NSEnumerator")]
unsafe impl<T: Message> iter::FastEnumerationHelper for NSMutableSet<T> {
    type Item = T;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

/// An iterator over the items of a `NSSet`.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct Iter<'a, T: Message>(iter::Iter<'a, NSSet<T>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, T: Message> Iterator<Item = &'a T> for Iter<'a, T> { ... }
}

/// An iterator that retains the items of a `NSSet`.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct IterRetained<'a, T: Message>(iter::IterRetained<'a, NSSet<T>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, T: Message + IsIdCloneable> Iterator<Item = Retained<T>> for IterRetained<'a, T> { ... }
}

/// A consuming iterator over the items of a `NSSet`.
#[derive(Debug)]
#[cfg(feature = "NSEnumerator")]
pub struct IntoIter<T: Message>(iter::IntoIter<NSSet<T>>);

#[cfg(feature = "NSEnumerator")]
__impl_iter! {
    impl<'a, T: Message> Iterator<Item = Retained<T>> for IntoIter<T> { ... }
}

#[cfg(feature = "NSEnumerator")]
__impl_into_iter! {
    impl<T: Message> IntoIterator for &NSSet<T> {
        type IntoIter = Iter<'_, T>;
    }

    impl<T: Message> IntoIterator for &NSMutableSet<T> {
        type IntoIter = Iter<'_, T>;
    }

    impl<T: Message + IsIdCloneable> IntoIterator for Retained<NSSet<T>> {
        type IntoIter = IntoIter<T>;
    }

    impl<T: Message> IntoIterator for Retained<NSMutableSet<T>> {
        type IntoIter = IntoIter<T>;
    }
}

#[cfg(feature = "NSEnumerator")]
impl<T: fmt::Debug + Message> fmt::Debug for NSSet<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self).finish()
    }
}

#[cfg(feature = "NSEnumerator")]
impl<T: fmt::Debug + Message> fmt::Debug for NSMutableSet<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

#[cfg(feature = "NSEnumerator")]
impl<T: fmt::Debug + Message> fmt::Debug for crate::Foundation::NSCountedSet<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: Message> Extend<Retained<T>> for NSMutableSet<T> {
    fn extend<I: IntoIterator<Item = Retained<T>>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |item| {
            self.insert_id(item);
        });
    }
}

impl<'a, T: Message + IsRetainable> Extend<&'a T> for NSMutableSet<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        iter.into_iter().for_each(move |item| {
            self.insert(item);
        });
    }
}

impl<'a, T: Message + IsRetainable + 'a> RetainedFromIterator<&'a T> for NSSet<T> {
    fn id_from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_slice(&vec)
    }
}

impl<T: Message> RetainedFromIterator<Retained<T>> for NSSet<T> {
    fn id_from_iter<I: IntoIterator<Item = Retained<T>>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_vec(vec)
    }
}

impl<'a, T: Message + IsRetainable + 'a> RetainedFromIterator<&'a T> for NSMutableSet<T> {
    fn id_from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_slice(&vec)
    }
}

impl<T: Message> RetainedFromIterator<Retained<T>> for NSMutableSet<T> {
    fn id_from_iter<I: IntoIterator<Item = Retained<T>>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_vec(vec)
    }
}
