#![cfg(feature = "Foundation_NSMutableDictionary")]
use alloc::vec::Vec;
use core::ops::{Index, IndexMut};
use core::ptr;

use objc2::msg_send_id;
use objc2::rc::DefaultId;

use crate::common::*;
use crate::Foundation::{self, NSDictionary, NSMutableDictionary};

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
        // SAFETY:
        // Mutable dictionaries are always unique, so it's safe to return
        // `Id<Self, Owned>`
        #[method_id(new)]
        pub fn new() -> Id<Self, Owned>;

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
        pub fn from_keys_and_objects<T>(keys: &[&T], vals: Vec<Id<V, Owned>>) -> Id<Self, Owned>
        where
            T: Foundation::NSCopying<Output = K>,
        {
            let mut dict = NSMutableDictionary::new();
            dict.setDictionary(&*NSDictionary::from_keys_and_objects(keys, vals));
            dict
        }

        /// Returns a mutable reference to the value corresponding to the key.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSMutableDictionary, NSObject, NSString};
        /// use icrate::ns_string;
        ///
        /// let mut dict = NSMutableDictionary::new();
        /// dict.insert(NSString::from_str("one"), NSObject::new());
        /// println!("{:?}", dict.get_mut(ns_string!("one")));
        /// ```
        #[doc(alias = "objectForKey:")]
        #[method(objectForKey:)]
        pub fn get_mut(&mut self, key: &K) -> Option<&mut V>;

        #[method(getObjects:andKeys:)]
        unsafe fn get_objects_and_keys(&self, objects: *mut &mut V, keys: *mut &K);

        /// Returns a vector of mutable references to the values in the dictionary.
        ///
        /// # Examples
        ///
        /// ```
        /// use icrate::Foundation::{NSMutableDictionary, NSObject, NSString};
        ///
        /// let mut dict = NSMutableDictionary::new();
        /// dict.insert(NSString::from_str("one"), NSObject::new());
        /// for val in dict.values_mut() {
        ///     println!("{:?}", val);
        /// }
        /// ```
        #[doc(alias = "getObjects:andKeys:")]
        pub fn values_mut(&mut self) -> Vec<&mut V> {
            let len = self.len();
            let mut vals: Vec<&mut V> = Vec::with_capacity(len);
            // SAFETY: `vals` is not null
            unsafe {
                self.get_objects_and_keys(vals.as_mut_ptr(), ptr::null_mut());
                vals.set_len(len);
            }
            vals
        }

        #[method(setObject:forKey:)]
        fn set_object_for_key(&mut self, object: &V, key: &K);

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
        pub fn insert(&mut self, key: Id<K, Shared>, value: Id<V, Owned>) -> Option<Id<V, Owned>> {
            // SAFETY:
            // `obj` is a reference to a value in the dictionary so it's safe
            // to cast it to a pointer and pass it to `Id::retain_autoreleased`
            let obj = self.get(&*key).map(|obj| unsafe {
                Id::retain_autoreleased(obj as *const V as *mut V).unwrap_unchecked()
            });
            self.set_object_for_key(&*value, &*key);
            obj
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
        pub fn remove(&mut self, key: &K) -> Option<Id<V, Owned>> {
            // SAFETY:
            // `obj` is a reference to a value in the dictionary so it's safe
            // to cast it to a pointer and pass it to `Id::retain_autoreleased`
            let obj = self.get(key).map(|obj| unsafe {
                Id::retain_autoreleased(obj as *const V as *mut V).unwrap_unchecked()
            });
            self.removeObjectForKey(key);
            obj
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
        pub fn into_values_array(
            dict: Id<Self, Owned>,
        ) -> Id<Foundation::NSArray<V, Owned>, Shared> {
            unsafe { msg_send_id![&dict, allValues] }
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

impl<'a, K: Message, V: Message> IndexMut<&'a K> for NSMutableDictionary<K, V> {
    fn index_mut<'s>(&'s mut self, index: &'a K) -> &'s mut V {
        self.get_mut(index).unwrap()
    }
}

impl<K: Message, V: Message> DefaultId for NSMutableDictionary<K, V> {
    type Ownership = Owned;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}
