#![cfg(feature = "Foundation_NSMutableDictionary")]
use alloc::vec::Vec;
use core::cmp::min;
use core::mem;
use core::ops::{Index, IndexMut};
use core::ptr;

use objc2::mutability::{IsMutable, IsRetainable};
use objc2::rc::DefaultId;
use objc2::runtime::Object;

use super::util;
use crate::common::*;
use crate::Foundation::{self, Copyhelper, NSDictionary, NSMutableDictionary};

extern_methods!(
    unsafe impl<K: Message, V: Message> NSMutableDictionary<K, V> {
        /// Creates an empty [`NSMutableDictionary`].
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSMutableDictionary, NSObject, NSString};
        ///
        /// let dict = NSMutableDictionary::<NSString, NSObject>::new();
        /// ```
        #[method_id(new)]
        pub fn new() -> Id<Self>;

        /// Creates an [`NSMutableDictionary`] from a slice of keys and a
        /// vector of values.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSMutableDictionary, NSNumber, NSObject};
        ///
        /// let dict = NSMutableDictionary::from_keys_and_objects(
        ///    &[
        ///        &*NSNumber::new_i32(1),
        ///        &*NSNumber::new_i32(2),
        ///        &*NSNumber::new_i32(3),
        ///    ],
        ///    vec![NSObject::new(), NSObject::new(), NSObject::new()],
        /// );
        /// ```
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
);

unsafe impl<K: Message, V: Message> Foundation::NSFastEnumeration2 for NSMutableDictionary<K, V> {
    type Item = K;
}

impl<'a, K: Message, V: Message> Index<&'a K> for NSMutableDictionary<K, V> {
    type Output = V;

    fn index<'s>(&'s self, index: &'a K) -> &'s V {
        self.get(index).unwrap()
    }
}

impl<'a, K: Message, V: IsMutable> IndexMut<&'a K> for NSMutableDictionary<K, V> {
    fn index_mut<'s>(&'s mut self, index: &'a K) -> &'s mut V {
        self.get_mut(index).unwrap()
    }
}

impl<K: Message, V: Message> DefaultId for NSMutableDictionary<K, V> {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}
