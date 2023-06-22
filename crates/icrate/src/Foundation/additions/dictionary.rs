//! Utilities for the `NSDictionary` and `NSMutableDictionary` classes.
#![cfg(feature = "Foundation_NSDictionary")]
use alloc::vec::Vec;
use core::cmp::min;
use core::fmt;
use core::mem;
use core::ops::{Index, IndexMut};
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::{self, NonNull};

use objc2::mutability::IsMutable;

use super::iter;
use super::util;
use crate::common::*;
#[cfg(feature = "Foundation_NSMutableDictionary")]
use crate::Foundation::NSMutableDictionary;
use crate::Foundation::{self, Copyhelper, NSCopying, NSDictionary};

impl<K: Message, V: Message> NSDictionary<K, V> {
    pub fn from_keys_and_objects<T>(keys: &[&T], mut vals: Vec<Id<V>>) -> Id<Self>
    where
        T: ClassType + NSCopying,
        T::Mutability: Copyhelper<T, CopyOutput = K>,
    {
        let count = min(keys.len(), vals.len());

        let keys: *mut NonNull<T> = util::ref_ptr_cast_const(keys.as_ptr());
        let keys: *mut NonNull<AnyObject> = keys.cast();
        let vals = util::id_ptr_cast(vals.as_mut_ptr());

        unsafe { Self::initWithObjects_forKeys_count(Self::alloc(), vals, keys, count) }
    }
}

#[cfg(feature = "Foundation_NSMutableDictionary")]
impl<K: Message, V: Message> NSMutableDictionary<K, V> {
    pub fn from_keys_and_objects<T>(keys: &[&T], mut vals: Vec<Id<V>>) -> Id<Self>
    where
        T: ClassType + NSCopying,
        T::Mutability: Copyhelper<T, CopyOutput = K>,
    {
        let count = min(keys.len(), vals.len());

        let keys: *mut NonNull<T> = util::ref_ptr_cast_const(keys.as_ptr());
        let keys: *mut NonNull<AnyObject> = keys.cast();
        let vals = util::id_ptr_cast(vals.as_mut_ptr());

        unsafe { Self::initWithObjects_forKeys_count(Self::alloc(), vals, keys, count) }
    }
}

extern_methods!(
    unsafe impl<K: Message, V: Message> NSDictionary<K, V> {
        pub fn len(&self) -> usize {
            self.count()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        #[doc(alias = "objectForKey:")]
        #[method(objectForKey:)]
        pub fn get(&self, key: &K) -> Option<&V>;

        #[doc(alias = "objectForKey:")]
        pub fn get_retained(&self, key: &K) -> Option<Id<V>>
        where
            V: IsIdCloneable,
        {
            // SAFETY: The object is stored in the dictionary
            self.get(key)
                .map(|obj| unsafe { util::collection_retain_id(obj) })
        }

        /// Returns a mutable reference to the value corresponding to the key.
        ///
        /// # Examples
        ///
        #[cfg_attr(
            all(
                feature = "Foundation_NSMutableDictionary",
                feature = "Foundation_NSMutableString"
            ),
            doc = "```"
        )]
        #[cfg_attr(
            not(all(
                feature = "Foundation_NSMutableDictionary",
                feature = "Foundation_NSMutableString"
            )),
            doc = "```ignore"
        )]
        /// use icrate::Foundation::{NSMutableDictionary, NSMutableString, NSString};
        /// use icrate::ns_string;
        ///
        /// let mut dict = NSMutableDictionary::new();
        /// dict.insert(NSString::from_str("one"), NSMutableString::new());
        /// println!("{:?}", dict.get_mut(ns_string!("one")));
        /// ```
        #[doc(alias = "objectForKey:")]
        #[method(objectForKey:)]
        pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
        where
            V: IsMutable;
    }
);

impl<K: Message, V: Message> NSDictionary<K, V> {
    #[doc(alias = "getObjects:andKeys:")]
    pub fn keys_vec(&self) -> Vec<&K> {
        let len = self.len();
        let mut keys: Vec<NonNull<K>> = Vec::with_capacity(len);
        unsafe {
            #[allow(deprecated)]
            self.getObjects_andKeys(ptr::null_mut(), keys.as_mut_ptr());
            keys.set_len(len);
            mem::transmute(keys)
        }
    }

    // We don't provide `keys_mut_vec`, since keys are expected to be
    // immutable.

    #[doc(alias = "getObjects:andKeys:")]
    pub fn values_vec(&self) -> Vec<&V> {
        let len = self.len();
        let mut vals: Vec<NonNull<V>> = Vec::with_capacity(len);
        unsafe {
            #[allow(deprecated)]
            self.getObjects_andKeys(vals.as_mut_ptr(), ptr::null_mut());
            vals.set_len(len);
            mem::transmute(vals)
        }
    }

    /// Returns a vector of mutable references to the values in the dictionary.
    ///
    /// # Examples
    ///
    #[cfg_attr(
        all(
            feature = "Foundation_NSMutableDictionary",
            feature = "Foundation_NSMutableString"
        ),
        doc = "```"
    )]
    #[cfg_attr(
        not(all(
            feature = "Foundation_NSMutableDictionary",
            feature = "Foundation_NSMutableString"
        )),
        doc = "```ignore"
    )]
    /// use icrate::Foundation::{NSMutableDictionary, NSMutableString, NSString};
    ///
    /// let mut dict = NSMutableDictionary::new();
    /// dict.insert(NSString::from_str("one"), NSMutableString::from_str("two"));
    /// for val in dict.values_mut() {
    ///     println!("{:?}", val);
    /// }
    /// ```
    #[doc(alias = "getObjects:andKeys:")]
    pub fn values_vec_mut(&mut self) -> Vec<&mut V>
    where
        V: IsMutable,
    {
        let len = self.len();
        let mut vals: Vec<NonNull<V>> = Vec::with_capacity(len);
        unsafe {
            #[allow(deprecated)]
            self.getObjects_andKeys(vals.as_mut_ptr(), ptr::null_mut());
            vals.set_len(len);
            mem::transmute(vals)
        }
    }

    #[doc(alias = "getObjects:andKeys:")]
    pub fn to_vecs(&self) -> (Vec<&K>, Vec<&V>) {
        let len = self.len();
        let mut keys: Vec<NonNull<K>> = Vec::with_capacity(len);
        let mut objs: Vec<NonNull<V>> = Vec::with_capacity(len);
        unsafe {
            #[allow(deprecated)]
            self.getObjects_andKeys(objs.as_mut_ptr(), keys.as_mut_ptr());
            keys.set_len(len);
            objs.set_len(len);
            (mem::transmute(keys), mem::transmute(objs))
        }
    }
}

#[cfg(feature = "Foundation_NSMutableDictionary")]
impl<K: Message, V: Message> NSMutableDictionary<K, V> {
    /// Inserts a key-value pair into the dictionary.
    ///
    /// If the dictionary did not have this key present, None is returned.
    /// If the dictionary did have this key present, the value is updated,
    /// and the old value is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSMutableDictionary, NSObject, NSString};
    ///
    /// let mut dict = NSMutableDictionary::new();
    /// dict.insert(NSString::from_str("one"), NSObject::new());
    /// ```
    #[doc(alias = "setObject:forKey:")]
    pub fn insert(&mut self, key: Id<K>, value: Id<V>) -> Option<Id<V>> {
        // SAFETY: We remove the object from the dictionary below
        let old_obj = self
            .get(&key)
            .map(|old_obj| unsafe { util::mutable_collection_retain_removed_id(old_obj) });

        // SAFETY: It is always safe to transmute an `Id` to `AnyObject`.
        let key: Id<AnyObject> = unsafe { Id::cast(key) };
        // SAFETY: We have ownership over both the key and the value.
        unsafe { self.setObject_forKey(&value, &key) };
        old_obj
    }

    /// Removes a key from the dictionary, returning the value at the key
    /// if the key was previously in the dictionary.
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSMutableDictionary, NSObject, NSString};
    /// use icrate::ns_string;
    ///
    /// let mut dict = NSMutableDictionary::new();
    /// dict.insert(NSString::from_str("one"), NSObject::new());
    /// dict.remove(ns_string!("one"));
    /// assert!(dict.is_empty());
    /// ```
    #[doc(alias = "removeObjectForKey:")]
    pub fn remove(&mut self, key: &K) -> Option<Id<V>> {
        // SAFETY: We remove the object from the dictionary below
        let old_obj = self
            .get(key)
            .map(|old_obj| unsafe { util::mutable_collection_retain_removed_id(old_obj) });
        self.removeObjectForKey(key);
        old_obj
    }

    /// Returns an [`NSArray`] containing the dictionary's values,
    /// consuming the dictionary.
    ///
    /// [`NSArray`]: crate::Foundation::NSArray
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use icrate::Foundation::{NSMutableDictionary, NSObject, NSString};
    ///
    /// let mut dict = NSMutableDictionary::new();
    /// dict.insert(NSString::from_str("one"), NSObject::new());
    /// let array = NSMutableDictionary::into_values_array(dict);
    /// println!("{:?}", array);
    /// ```
    #[cfg(feature = "Foundation_NSArray")]
    pub fn into_values_array(this: Id<Self>) -> Id<Foundation::NSArray<V>> {
        // SAFETY: We've consumed the dictionary, so getting an array from
        // it is safe.
        unsafe { this.allValues() }
    }
}

impl<K: Message, V: Message> NSDictionary<K, V> {
    #[doc(alias = "keyEnumerator")]
    #[cfg(feature = "Foundation_NSEnumerator")]
    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys(iter::Iter::new(self))
    }

    #[doc(alias = "keyEnumerator")]
    #[cfg(feature = "Foundation_NSEnumerator")]
    pub fn keys_retained(&self) -> KeysRetained<'_, K, V>
    where
        K: IsIdCloneable,
    {
        KeysRetained(iter::IterRetained::new(self))
    }

    // TODO: Is this ever useful?
    // pub fn into_keys(this: Id<Self>) -> IntoKeys<K, V> {
    //     todo!()
    // }

    #[doc(alias = "objectEnumerator")]
    #[cfg(feature = "Foundation_NSEnumerator")]
    pub fn values(&self) -> Values<'_, K, V> {
        let enumerator = unsafe { self.objectEnumerator() };
        // SAFETY: The enumerator came from the dictionary.
        Values(unsafe { iter::IterWithBackingEnum::new(self, enumerator) })
    }

    #[doc(alias = "objectEnumerator")]
    #[cfg(feature = "Foundation_NSEnumerator")]
    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V>
    where
        V: IsMutable,
    {
        let enumerator = unsafe { self.objectEnumerator() };
        // SAFETY: The enumerator came from the dictionary.
        ValuesMut(unsafe { iter::IterMutWithBackingEnum::new(self, enumerator) })
    }

    #[doc(alias = "objectEnumerator")]
    #[cfg(feature = "Foundation_NSEnumerator")]
    pub fn values_retained(&self) -> ValuesRetained<'_, K, V>
    where
        V: IsIdCloneable,
    {
        let enumerator = unsafe { self.objectEnumerator() };
        // SAFETY: The enumerator came from the dictionary.
        ValuesRetained(unsafe { iter::IterRetainedWithBackingEnum::new(self, enumerator) })
    }

    #[doc(alias = "objectEnumerator")]
    #[cfg(feature = "Foundation_NSEnumerator")]
    pub fn into_values(this: Id<Self>) -> IntoValues<K, V>
    where
        V: IsIdCloneable,
    {
        let enumerator = unsafe { this.objectEnumerator() };
        // SAFETY: The enumerator came from the dictionary.
        IntoValues(unsafe { iter::IntoIterWithBackingEnum::new_immutable(this, enumerator) })
    }
}

unsafe impl<K: Message, V: Message> iter::FastEnumerationHelper for NSDictionary<K, V> {
    // Fast enumeration for dictionaries returns the keys.
    type Item = K;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

#[cfg(feature = "Foundation_NSMutableDictionary")]
unsafe impl<K: Message, V: Message> iter::FastEnumerationHelper for NSMutableDictionary<K, V> {
    // The same goes for mutable dictionaries.
    type Item = K;

    #[inline]
    fn maybe_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

// We also cfg-gate `Keys` behind `NSEnumerator` for symmetry, even though we
// don't necessarily need to.
#[cfg(feature = "Foundation_NSEnumerator")]
mod iter_helpers {
    use super::*;
    use crate::Foundation::NSEnumerator;

    /// An iterator over the keys of a `NSDictionary`.
    #[derive(Debug)]
    pub struct Keys<'a, K: Message, V: Message>(pub(super) iter::Iter<'a, NSDictionary<K, V>>);

    __impl_iter! {
        impl<'a, K: Message, V: Message> Iterator<Item = &'a K> for Keys<'a, K, V> { ... }
    }

    /// An iterator that retains the keys of a `NSDictionary`.
    #[derive(Debug)]
    pub struct KeysRetained<'a, K: Message, V: Message>(
        pub(super) iter::IterRetained<'a, NSDictionary<K, V>>,
    );

    __impl_iter! {
        impl<'a, K: IsIdCloneable, V: Message> Iterator<Item = Id<K>> for KeysRetained<'a, K, V> { ... }
    }

    /// An iterator over the values of a `NSDictionary`.
    #[derive(Debug)]
    pub struct Values<'a, K: Message, V: Message>(
        pub(super) iter::IterWithBackingEnum<'a, NSDictionary<K, V>, NSEnumerator<V>>,
    );

    __impl_iter! {
        impl<'a, K: Message, V: Message> Iterator<Item = &'a V> for Values<'a, K, V> { ... }
    }

    /// A mutable iterator over the values of a `NSDictionary`.
    #[derive(Debug)]
    pub struct ValuesMut<'a, K: Message, V: Message>(
        pub(super) iter::IterMutWithBackingEnum<'a, NSDictionary<K, V>, NSEnumerator<V>>,
    );

    __impl_iter! {
        impl<'a, K: Message, V: IsMutable> Iterator<Item = &'a mut V> for ValuesMut<'a, K, V> { ... }
    }

    /// A iterator that retains the values of a `NSDictionary`.
    #[derive(Debug)]
    pub struct ValuesRetained<'a, K: Message, V: Message>(
        pub(super) iter::IterRetainedWithBackingEnum<'a, NSDictionary<K, V>, NSEnumerator<V>>,
    );

    __impl_iter! {
        impl<'a, K: Message, V: IsIdCloneable> Iterator<Item = Id<V>> for ValuesRetained<'a, K, V> { ... }
    }

    /// A consuming iterator over the values of a `NSDictionary`.
    #[derive(Debug)]
    pub struct IntoValues<K: Message, V: Message>(
        pub(super) iter::IntoIterWithBackingEnum<NSDictionary<K, V>, NSEnumerator<V>>,
    );

    __impl_iter! {
        impl<K: Message, V: Message> Iterator<Item = Id<V>> for IntoValues<K, V> { ... }
    }
}

#[cfg(feature = "Foundation_NSEnumerator")]
pub use self::iter_helpers::*;

impl<'a, K: Message, V: Message> Index<&'a K> for NSDictionary<K, V> {
    type Output = V;

    fn index<'s>(&'s self, index: &'a K) -> &'s V {
        self.get(index).unwrap()
    }
}

#[cfg(feature = "Foundation_NSMutableDictionary")]
impl<'a, K: Message, V: Message> Index<&'a K> for NSMutableDictionary<K, V> {
    type Output = V;

    fn index<'s>(&'s self, index: &'a K) -> &'s V {
        self.get(index).unwrap()
    }
}

impl<'a, K: Message, V: IsMutable> IndexMut<&'a K> for NSDictionary<K, V> {
    fn index_mut<'s>(&'s mut self, index: &'a K) -> &'s mut V {
        self.get_mut(index).unwrap()
    }
}

#[cfg(feature = "Foundation_NSMutableDictionary")]
impl<'a, K: Message, V: IsMutable> IndexMut<&'a K> for NSMutableDictionary<K, V> {
    fn index_mut<'s>(&'s mut self, index: &'a K) -> &'s mut V {
        self.get_mut(index).unwrap()
    }
}

impl<K: fmt::Debug + Message, V: fmt::Debug + Message> fmt::Debug for NSDictionary<K, V> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (keys, values) = self.to_vecs();
        let iter = keys.into_iter().zip(values);
        f.debug_map().entries(iter).finish()
    }
}
