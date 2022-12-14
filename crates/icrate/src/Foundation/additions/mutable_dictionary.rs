use alloc::vec::Vec;
use core::ops::{Index, IndexMut};
use core::ptr;

use objc2::rc::{DefaultId, Id, Owned, Shared};
use objc2::{extern_methods, msg_send_id, ClassType, Message};

use crate::Foundation::{NSArray, NSCopying, NSDictionary, NSFastEnumeration, NSMutableDictionary};

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
            T: NSCopying<Output = K>,
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
        pub fn into_values_array(dict: Id<Self, Owned>) -> Id<NSArray<V, Owned>, Shared> {
            unsafe { msg_send_id![&dict, allValues] }
        }
    }
);

unsafe impl<K: Message, V: Message> NSFastEnumeration for NSMutableDictionary<K, V> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    use crate::Foundation::{NSNumber, NSObject, NSString};
    use objc2::rc::{__RcTestObject, __ThreadTestData};

    fn sample_dict() -> Id<NSMutableDictionary<NSNumber, NSObject>, Owned> {
        NSMutableDictionary::from_keys_and_objects(
            &[
                &*NSNumber::new_i32(1),
                &*NSNumber::new_i32(2),
                &*NSNumber::new_i32(3),
            ],
            vec![NSObject::new(), NSObject::new(), NSObject::new()],
        )
    }

    #[test]
    fn test_new() {
        let dict = NSMutableDictionary::<NSString, NSObject>::new();
        assert!(dict.is_empty());
    }

    #[test]
    fn test_get_mut() {
        let mut dict = sample_dict();
        assert!(dict.get_mut(&NSNumber::new_i32(1)).is_some());
        assert!(dict.get_mut(&NSNumber::new_i32(2)).is_some());
        assert!(dict.get_mut(&NSNumber::new_i32(4)).is_none());
    }

    #[test]
    fn test_values_mut() {
        let mut dict = sample_dict();
        let vec = dict.values_mut();
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_insert() {
        let mut dict = NSMutableDictionary::new();
        assert!(dict.insert(NSNumber::new_i32(1), NSObject::new()).is_none());
        assert!(dict.insert(NSNumber::new_i32(2), NSObject::new()).is_none());
        assert!(dict.insert(NSNumber::new_i32(3), NSObject::new()).is_none());
        assert!(dict.insert(NSNumber::new_i32(1), NSObject::new()).is_some());
        assert_eq!(dict.len(), 3);
    }

    #[test]
    fn test_insert_retain_release() {
        let mut dict = NSMutableDictionary::new();
        dict.insert(NSNumber::new_i32(1), __RcTestObject::new());
        let mut expected = __ThreadTestData::current();

        let old = dict.insert(NSNumber::new_i32(1), __RcTestObject::new());
        expected.alloc += 1;
        expected.init += 1;
        expected.retain += 2;
        expected.release += 2;
        expected.assert_current();

        drop(old);
        expected.release += 1;
        expected.dealloc += 1;
        expected.assert_current();
    }

    #[test]
    fn test_remove() {
        let mut dict = sample_dict();
        assert_eq!(dict.len(), 3);
        assert!(dict.remove(&NSNumber::new_i32(1)).is_some());
        assert!(dict.remove(&NSNumber::new_i32(2)).is_some());
        assert!(dict.remove(&NSNumber::new_i32(1)).is_none());
        assert!(dict.remove(&NSNumber::new_i32(4)).is_none());
        assert_eq!(dict.len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut dict = sample_dict();
        assert_eq!(dict.len(), 3);

        dict.removeAllObjects();
        assert!(dict.is_empty());
    }

    #[test]
    fn test_remove_clear_release_dealloc() {
        let mut dict = NSMutableDictionary::new();
        for i in 0..4 {
            dict.insert(NSNumber::new_i32(i), __RcTestObject::new());
        }
        let mut expected = __ThreadTestData::current();

        let _obj = dict.remove(&NSNumber::new_i32(1));
        expected.retain += 1;
        expected.release += 1;
        expected.assert_current();
        assert_eq!(dict.len(), 3);

        let _obj = dict.remove(&NSNumber::new_i32(2));
        expected.retain += 1;
        expected.release += 1;
        expected.assert_current();
        assert_eq!(dict.len(), 2);

        dict.removeAllObjects();
        expected.release += 2;
        expected.dealloc += 2;
        expected.assert_current();
        assert_eq!(dict.len(), 0);
    }

    #[test]
    fn test_into_values_array() {
        let dict = sample_dict();
        let array = NSMutableDictionary::into_values_array(dict);
        assert_eq!(array.len(), 3);
    }
}
