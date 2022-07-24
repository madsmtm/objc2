use alloc::vec::Vec;
use core::cmp::min;
use core::fmt;
use core::marker::PhantomData;
use core::ops::Index;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr;

use objc2::rc::{DefaultId, Id, Owned, Shared, SliceId};
use objc2::{msg_send, msg_send_id, Message};

use super::{extern_class, NSArray, NSCopying, NSEnumerator, NSFastEnumeration, NSObject};

extern_class! {
    #[derive(PartialEq, Eq, Hash)]
    unsafe pub struct NSDictionary<K, V>: NSObject {
        key: PhantomData<Id<K, Shared>>,
        obj: PhantomData<Id<V, Owned>>,
    }
}

// TODO: SAFETY
// Approximately same as `NSArray<T, Shared>`
unsafe impl<K: Sync + Send, V: Sync> Sync for NSDictionary<K, V> {}
unsafe impl<K: Sync + Send, V: Send> Send for NSDictionary<K, V> {}

// Approximately same as `NSArray<T, Shared>`
impl<K: UnwindSafe, V: UnwindSafe> UnwindSafe for NSDictionary<K, V> {}
impl<K: RefUnwindSafe, V: RefUnwindSafe> RefUnwindSafe for NSDictionary<K, V> {}

impl<K: Message, V: Message> NSDictionary<K, V> {
    pub fn new() -> Id<Self, Shared> {
        unsafe { msg_send_id![Self::class(), new].unwrap() }
    }

    #[doc(alias = "count")]
    pub fn len(&self) -> usize {
        unsafe { msg_send![self, count] }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[doc(alias = "objectForKey:")]
    pub fn get(&self, key: &K) -> Option<&V> {
        unsafe { msg_send![self, objectForKey: key] }
    }

    #[doc(alias = "getObjects:andKeys:")]
    pub fn keys(&self) -> Vec<&K> {
        let len = self.len();
        let mut keys = Vec::with_capacity(len);
        unsafe {
            let _: () = msg_send![
                self,
                getObjects: ptr::null_mut::<&V>(),
                andKeys: keys.as_mut_ptr(),
            ];
            keys.set_len(len);
        }
        keys
    }

    #[doc(alias = "getObjects:andKeys:")]
    pub fn values(&self) -> Vec<&V> {
        let len = self.len();
        let mut vals = Vec::with_capacity(len);
        unsafe {
            let _: () = msg_send![
                self,
                getObjects: vals.as_mut_ptr(),
                andKeys: ptr::null_mut::<&K>(),
            ];
            vals.set_len(len);
        }
        vals
    }

    #[doc(alias = "getObjects:andKeys:")]
    pub fn keys_and_objects(&self) -> (Vec<&K>, Vec<&V>) {
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
    pub fn iter_keys(&self) -> NSEnumerator<'_, K> {
        unsafe {
            let result = msg_send![self, keyEnumerator];
            NSEnumerator::from_ptr(result)
        }
    }

    #[doc(alias = "objectEnumerator")]
    pub fn iter_values(&self) -> NSEnumerator<'_, V> {
        unsafe {
            let result = msg_send![self, objectEnumerator];
            NSEnumerator::from_ptr(result)
        }
    }

    pub fn keys_array(&self) -> Id<NSArray<K, Shared>, Shared> {
        unsafe { msg_send_id![self, allKeys] }.unwrap()
    }

    pub fn from_keys_and_objects<T>(keys: &[&T], vals: Vec<Id<V, Owned>>) -> Id<Self, Shared>
    where
        T: NSCopying<Output = K>,
    {
        let vals = vals.as_slice_ref();

        let cls = Self::class();
        let count = min(keys.len(), vals.len());
        let obj = unsafe { msg_send_id![cls, alloc] };
        unsafe {
            msg_send_id![
                obj,
                initWithObjects: vals.as_ptr(),
                forKeys: keys.as_ptr(),
                count: count,
            ]
        }
        .unwrap()
    }

    pub fn into_values_array(dict: Id<Self, Owned>) -> Id<NSArray<V, Owned>, Shared> {
        unsafe { msg_send_id![&dict, allValues] }.unwrap()
    }
}

impl<K: Message, V: Message> DefaultId for NSDictionary<K, V> {
    type Ownership = Shared;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

unsafe impl<K: Message, V: Message> NSFastEnumeration for NSDictionary<K, V> {
    type Item = K;
}

impl<'a, K: Message, V: Message> Index<&'a K> for NSDictionary<K, V> {
    type Output = V;

    fn index<'s>(&'s self, index: &'a K) -> &'s V {
        self.get(index).unwrap()
    }
}

impl<K: fmt::Debug + Message, V: fmt::Debug + Message> fmt::Debug for NSDictionary<K, V> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iter = self.iter_keys().zip(self.iter_values());
        f.debug_map().entries(iter).finish()
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;
    use alloc::vec;
    use objc2::rc::{autoreleasepool, Id, Shared};

    use super::NSDictionary;
    use crate::{NSObject, NSString};

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

        // let objs = NSDictionary::into_values_array(dict);
        // assert_eq!(objs.len(), 1);
    }

    #[test]
    fn test_debug() {
        let key = NSString::from_str("a");
        // TODO: Fix this
        let val = unsafe { Id::from_shared(NSString::from_str("b")) };
        let dict = NSDictionary::from_keys_and_objects(&[&*key], vec![val]);
        assert_eq!(format!("{:?}", dict), r#"{"a": "b"}"#);
    }
}
