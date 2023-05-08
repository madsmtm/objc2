#![cfg(feature = "Foundation_NSMutableSet")]
use alloc::vec::Vec;

use objc2::mutability::IsRetainable;
use objc2::rc::DefaultId;

use super::util;
use crate::common::*;
use crate::Foundation::{self, NSMutableSet};

impl<T: Message> NSMutableSet<T> {
    /// Creates an [`NSMutableSet`] from a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSMutableSet, NSString};
    ///
    /// let strs = ["one", "two", "three"].map(NSString::from_str).to_vec();
    /// let set = NSMutableSet::from_vec(strs);
    /// ```
    pub fn from_vec(mut vec: Vec<Id<T>>) -> Id<Self> {
        let len = vec.len();
        let ptr = util::id_ptr_cast(vec.as_mut_ptr());
        // SAFETY: Same as `NSArray::from_vec`.
        unsafe { Self::initWithObjects_count(Self::alloc(), ptr, len) }
    }

    /// Creates an [`NSMutableSet`] from a slice of `Id`s.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSMutableSet, NSString};
    ///
    /// let strs = ["one", "two", "three"].map(NSString::from_str);
    /// let set = NSMutableSet::from_id_slice(&strs);
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

    /// Returns a [`Vec`] containing the set's elements, consuming the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSMutableSet, NSMutableString};
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
    pub fn into_vec(set: Id<Self>) -> Vec<Id<T>> {
        // SAFETY: Same as `NSMutableArray::into_vec`
        set.into_iter()
            .map(|obj| unsafe { util::mutable_collection_retain_removed_id(obj) })
            .collect()
    }
}

// We're explicit about `T` being `PartialEq` for these methods because the
// set compares the input value with elements in the set
// For comparison: Rust's HashSet requires similar methods to be `Hash` + `Eq`
impl<T: Message + PartialEq> NSMutableSet<T> {
    /// Adds a value to the set. Returns whether the value was
    /// newly inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSMutableSet, NSString};
    ///
    /// let mut set = NSMutableSet::new();
    ///
    /// assert_eq!(set.insert(NSString::from_str("one")), true);
    /// assert_eq!(set.insert(NSString::from_str("one")), false);
    /// assert_eq!(set.len(), 1);
    /// ```
    #[doc(alias = "addObject:")]
    pub fn insert(&mut self, value: Id<T>) -> bool {
        let contains_value = self.contains(&value);
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
    /// use icrate::Foundation::{NSMutableSet, NSString};
    /// use icrate::ns_string;
    ///
    /// let mut set = NSMutableSet::new();
    ///
    /// set.insert(NSString::from_str("one"));
    /// assert_eq!(set.remove(ns_string!("one")), true);
    /// assert_eq!(set.remove(ns_string!("one")), false);
    /// ```
    #[doc(alias = "removeObject:")]
    pub fn remove(&mut self, value: &T) -> bool {
        let contains_value = self.contains(value);
        unsafe { self.removeObject(value) };
        contains_value
    }
}

unsafe impl<T: Message> Foundation::NSFastEnumeration2 for NSMutableSet<T> {
    type Item = T;
}

impl<'a, T: Message> IntoIterator for &'a NSMutableSet<T> {
    type Item = &'a T;
    type IntoIter = Foundation::NSFastEnumerator2<'a, NSMutableSet<T>>;

    fn into_iter(self) -> Self::IntoIter {
        use Foundation::NSFastEnumeration2;
        self.iter_fast()
    }
}

impl<T: Message + PartialEq> Extend<Id<T>> for NSMutableSet<T> {
    fn extend<I: IntoIterator<Item = Id<T>>>(&mut self, iter: I) {
        for item in iter {
            self.insert(item);
        }
    }
}

impl<T: Message> DefaultId for NSMutableSet<T> {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}
