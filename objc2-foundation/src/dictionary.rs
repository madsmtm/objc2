use alloc::vec::Vec;
use core::cmp::min;
use core::marker::PhantomData;
use core::ops::Index;
use core::ptr::{self, NonNull};

use objc2::msg_send;
use objc2::rc::{Id, Owned, Ownership, Shared, SliceId};

use super::{INSCopying, INSFastEnumeration, INSObject, NSArray, NSEnumerator};

unsafe fn from_refs<D, T>(keys: &[&T], vals: &[&D::Value]) -> Id<D, Shared>
where
    D: INSDictionary,
    T: INSCopying<Output = D::Key>,
{
    let cls = D::class();
    let count = min(keys.len(), vals.len());
    let obj: *mut D = unsafe { msg_send![cls, alloc] };
    let obj: *mut D = unsafe {
        msg_send![
            obj,
            initWithObjects: vals.as_ptr(),
            forKeys: keys.as_ptr(),
            count: count,
        ]
    };
    let obj = unsafe { NonNull::new_unchecked(obj) };
    unsafe { Id::new(obj) }
}

pub unsafe trait INSDictionary: INSObject {
    type Key: INSObject;
    type Value: INSObject;
    type ValueOwnership: Ownership;

    #[doc(alias = "count")]
    fn len(&self) -> usize {
        unsafe { msg_send![self, count] }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[doc(alias = "objectForKey:")]
    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        unsafe { msg_send![self, objectForKey: key] }
    }

    #[doc(alias = "getObjects:andKeys:")]
    fn keys(&self) -> Vec<&Self::Key> {
        let len = self.len();
        let mut keys = Vec::with_capacity(len);
        unsafe {
            let _: () = msg_send![
                self,
                getObjects: ptr::null_mut::<&Self::Value>(),
                andKeys: keys.as_mut_ptr(),
            ];
            keys.set_len(len);
        }
        keys
    }

    #[doc(alias = "getObjects:andKeys:")]
    fn values(&self) -> Vec<&Self::Value> {
        let len = self.len();
        let mut vals = Vec::with_capacity(len);
        unsafe {
            let _: () = msg_send![
                self,
                getObjects: vals.as_mut_ptr(),
                andKeys: ptr::null_mut::<&Self::Key>(),
            ];
            vals.set_len(len);
        }
        vals
    }

    #[doc(alias = "getObjects:andKeys:")]
    fn keys_and_objects(&self) -> (Vec<&Self::Key>, Vec<&Self::Value>) {
        let len = self.len();
        let mut keys = Vec::with_capacity(len);
        let mut objs = Vec::with_capacity(len);
        unsafe {
            let _: () = msg_send![
                self,
                getObjects: objs.as_mut_ptr(),
                andKeys: keys.as_mut_ptr(),
            ];
            keys.set_len(len);
            objs.set_len(len);
        }
        (keys, objs)
    }

    #[doc(alias = "keyEnumerator")]
    fn iter_keys(&self) -> NSEnumerator<'_, Self::Key> {
        unsafe {
            let result = msg_send![self, keyEnumerator];
            NSEnumerator::from_ptr(result)
        }
    }

    #[doc(alias = "objectEnumerator")]
    fn iter_values(&self) -> NSEnumerator<'_, Self::Value> {
        unsafe {
            let result = msg_send![self, objectEnumerator];
            NSEnumerator::from_ptr(result)
        }
    }

    fn keys_array(&self) -> Id<NSArray<Self::Key, Shared>, Shared> {
        unsafe {
            let keys = msg_send![self, allKeys];
            Id::retain(NonNull::new_unchecked(keys))
        }
    }

    fn from_keys_and_objects<T>(
        keys: &[&T],
        vals: Vec<Id<Self::Value, Self::ValueOwnership>>,
    ) -> Id<Self, Shared>
    where
        T: INSCopying<Output = Self::Key>,
    {
        unsafe { from_refs(keys, vals.as_slice_ref()) }
    }

    fn into_values_array(
        dict: Id<Self, Owned>,
    ) -> Id<NSArray<Self::Value, Self::ValueOwnership>, Shared> {
        unsafe {
            let vals = msg_send![dict, allValues];
            Id::retain(NonNull::new_unchecked(vals))
        }
    }
}

object!(
    unsafe pub struct NSDictionary<K, V> {
        key: PhantomData<Id<K, Shared>>,
        obj: PhantomData<Id<V, Owned>>,
    }
);

// TODO: SAFETY
unsafe impl<K: Sync + Send, V: Sync> Sync for NSDictionary<K, V> {}
unsafe impl<K: Sync + Send, V: Send> Send for NSDictionary<K, V> {}

impl<K: INSObject, V: INSObject> NSDictionary<K, V> {
    unsafe_def_fn!(pub fn new -> Shared);
}

unsafe impl<K: INSObject, V: INSObject> INSDictionary for NSDictionary<K, V> {
    type Key = K;
    type Value = V;
    type ValueOwnership = Owned;
}

unsafe impl<K: INSObject, V: INSObject> INSFastEnumeration for NSDictionary<K, V> {
    type Item = K;
}

impl<'a, K: INSObject, V: INSObject> Index<&'a K> for NSDictionary<K, V> {
    type Output = V;

    fn index(&self, index: &K) -> &V {
        self.get(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use objc2::rc::{autoreleasepool, Id, Shared};

    use super::{INSDictionary, NSDictionary};
    use crate::{INSArray, INSString, NSObject, NSString};

    fn sample_dict(key: &str) -> Id<NSDictionary<NSString, NSObject>, Shared> {
        let string = NSString::from_str(key);
        let obj = NSObject::new();
        NSDictionary::from_keys_and_objects(&[&*string], vec![obj])
    }

    #[test]
    fn test_len() {
        let dict = sample_dict("abcd");
        assert_eq!(dict.len(), 1);
    }

    #[test]
    fn test_get() {
        let dict = sample_dict("abcd");

        let string = NSString::from_str("abcd");
        assert!(dict.get(&string).is_some());

        let string = NSString::from_str("abcde");
        assert!(dict.get(&string).is_none());
    }

    #[test]
    fn test_keys() {
        let dict = sample_dict("abcd");
        let keys = dict.keys();

        assert_eq!(keys.len(), 1);
        autoreleasepool(|pool| {
            assert_eq!(keys[0].as_str(pool), "abcd");
        });
    }

    #[test]
    fn test_values() {
        let dict = sample_dict("abcd");
        let vals = dict.values();

        assert_eq!(vals.len(), 1);
    }

    #[test]
    fn test_keys_and_objects() {
        let dict = sample_dict("abcd");
        let (keys, objs) = dict.keys_and_objects();

        assert_eq!(keys.len(), 1);
        assert_eq!(objs.len(), 1);
        autoreleasepool(|pool| {
            assert_eq!(keys[0].as_str(pool), "abcd");
        });
        assert_eq!(objs[0], dict.get(keys[0]).unwrap());
    }

    #[test]
    fn test_iter_keys() {
        let dict = sample_dict("abcd");
        assert_eq!(dict.iter_keys().count(), 1);
        autoreleasepool(|pool| {
            assert_eq!(dict.iter_keys().next().unwrap().as_str(pool), "abcd");
        });
    }

    #[test]
    fn test_iter_values() {
        let dict = sample_dict("abcd");
        assert_eq!(dict.iter_values().count(), 1);
    }

    #[test]
    fn test_arrays() {
        let dict = sample_dict("abcd");

        let keys = dict.keys_array();
        assert_eq!(keys.len(), 1);
        autoreleasepool(|pool| {
            assert_eq!(keys[0].as_str(pool), "abcd");
        });

        // let objs = INSDictionary::into_values_array(dict);
        // assert_eq!(objs.len(), 1);
    }
}
