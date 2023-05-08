#![cfg(feature = "Foundation_NSDictionary")]
use alloc::vec::Vec;
use core::cmp::min;
use core::fmt;
use core::mem;
use core::ops::{Index, IndexMut};
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::{self, NonNull};

use objc2::msg_send;
use objc2::mutability::IsMutable;
use objc2::runtime::Object;

use super::util;
use crate::common::*;
#[cfg(feature = "Foundation_NSMutableDictionary")]
use crate::Foundation::NSMutableDictionary;
use crate::Foundation::{self, Copyhelper, NSDictionary};

impl<K: Message, V: Message> NSDictionary<K, V> {
    pub fn from_keys_and_objects<T>(keys: &[&T], mut vals: Vec<Id<V>>) -> Id<Self>
    where
        T: ClassType + Foundation::NSCopying,
        T::Mutability: Copyhelper<T, CopyOutput = K>,
    {
        let count = min(keys.len(), vals.len());

        let keys: *mut NonNull<T> = util::ref_ptr_cast_const(keys.as_ptr());
        let keys: *mut NonNull<Object> = keys.cast();
        let vals = util::id_ptr_cast(vals.as_mut_ptr());

        unsafe { Self::initWithObjects_forKeys_count(Self::alloc(), vals, keys, count) }
    }
}

#[cfg(feature = "Foundation_NSMutableDictionary")]
impl<K: Message, V: Message> NSMutableDictionary<K, V> {
    pub fn from_keys_and_objects<T>(keys: &[&T], mut vals: Vec<Id<V>>) -> Id<Self>
    where
        T: ClassType + Foundation::NSCopying,
        T::Mutability: Copyhelper<T, CopyOutput = K>,
    {
        let count = min(keys.len(), vals.len());

        let keys: *mut NonNull<T> = util::ref_ptr_cast_const(keys.as_ptr());
        let keys: *mut NonNull<Object> = keys.cast();
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
    pub fn keys(&self) -> Vec<&K> {
        let len = self.len();
        let mut keys: Vec<NonNull<K>> = Vec::with_capacity(len);
        unsafe {
            #[allow(deprecated)]
            self.getObjects_andKeys(ptr::null_mut(), keys.as_mut_ptr());
            keys.set_len(len);
            mem::transmute(keys)
        }
    }

    // We don't provide `keys_mut`, since keys are expected to be
    // immutable.

    #[doc(alias = "getObjects:andKeys:")]
    pub fn keys_retained(&self) -> Vec<Id<K>>
    where
        K: IsIdCloneable,
    {
        // SAFETY: The keys are stored in the array
        self.keys()
            .into_iter()
            .map(|obj| unsafe { util::collection_retain_id(obj) })
            .collect()
    }

    #[doc(alias = "getObjects:andKeys:")]
    pub fn values(&self) -> Vec<&V> {
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
    pub fn values_retained(&self) -> Vec<Id<V>>
    where
        V: IsIdCloneable,
    {
        // SAFETY: The values are stored in the array
        self.values()
            .into_iter()
            .map(|obj| unsafe { util::collection_retain_id(obj) })
            .collect()
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
    pub fn values_mut(&mut self) -> Vec<&mut V>
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
    pub fn keys_and_objects(&self) -> (Vec<&K>, Vec<&V>) {
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

    #[doc(alias = "keyEnumerator")]
    #[cfg(feature = "Foundation_NSEnumerator")]
    pub fn iter_keys(&self) -> Foundation::NSEnumerator2<'_, K> {
        unsafe {
            let result = msg_send![self, keyEnumerator];
            Foundation::NSEnumerator2::from_ptr(result)
        }
    }

    #[doc(alias = "objectEnumerator")]
    #[cfg(feature = "Foundation_NSEnumerator")]
    pub fn iter_values(&self) -> Foundation::NSEnumerator2<'_, V> {
        unsafe {
            let result = msg_send![self, objectEnumerator];
            Foundation::NSEnumerator2::from_ptr(result)
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

        // SAFETY: It is always safe to transmute an `Id` to `Object`.
        let key: Id<Object> = unsafe { Id::cast(key) };
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

unsafe impl<K: Message, V: Message> Foundation::NSFastEnumeration2 for NSDictionary<K, V> {
    type Item = K;
}

#[cfg(feature = "Foundation_NSMutableDictionary")]
unsafe impl<K: Message, V: Message> Foundation::NSFastEnumeration2 for NSMutableDictionary<K, V> {
    type Item = K;
}

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

#[cfg(feature = "Foundation_NSEnumerator")]
impl<K: fmt::Debug + Message, V: fmt::Debug + Message> fmt::Debug for NSDictionary<K, V> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter_keys().zip(self.iter_values());
        f.debug_map().entries(iter).finish()
    }
}
