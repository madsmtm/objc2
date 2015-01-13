use std::cmp::min;
use std::ops::Index;

use objc::{Id, IdSlice, IntoIdVector, Owned, Ownership};

use {class, INSArray, INSCopying, INSObject, NSArray, NSEnumerator};

pub trait INSDictionary : INSObject {
    type Key: INSObject;
    type Value: INSObject;
    type Own: Ownership;

    fn count(&self) -> uint {
        let result = unsafe {
            msg_send![self, count]
        };
        result as uint
    }

    fn object_for(&self, key: &Self::Key) -> Option<&Self::Value> {
        unsafe {
            let obj = msg_send![self, objectForKey:key] as *mut Self::Value;
            obj.as_ref()
        }
    }

    fn all_keys(&self) -> Vec<&Self::Key> {
        let keys = unsafe {
            &*(msg_send![self, allKeys] as *mut NSArray<Self::Key>)
        };
        keys.to_vec()
    }

    fn all_values(&self) -> Vec<&Self::Value> {
        let vals = unsafe {
            &*(msg_send![self, allValues] as *mut NSArray<Self::Value>)
        };
        vals.to_vec()
    }

    fn key_enumerator(&self) -> NSEnumerator<Self::Key> {
        unsafe {
            let result = msg_send![self, keyEnumerator];
            NSEnumerator::from_ptr(result)
        }
    }

    fn object_enumerator(&self) -> NSEnumerator<Self::Value> {
        unsafe {
            let result = msg_send![self, objectEnumerator];
            NSEnumerator::from_ptr(result)
        }
    }

    fn keys_and_objects(&self) -> (Vec<&Self::Key>, Vec<&Self::Value>) {
        let len = self.count();
        let mut keys: Vec<&Self::Key> = Vec::with_capacity(len);
        let mut objs: Vec<&Self::Value> = Vec::with_capacity(len);
        unsafe {
            msg_send![self, getObjects:objs.as_ptr() andKeys:keys.as_ptr()];
            keys.set_len(len);
            objs.set_len(len);
        }
        (keys, objs)
    }

    unsafe fn from_refs<T: INSCopying<Output=Self::Key>>(
            keys: &[&T], vals: &[&Self::Value]) -> Id<Self> {
        let cls = class::<Self>();
        let count = min(keys.len(), vals.len());
        let obj = msg_send![cls, alloc];
        let obj = msg_send![obj, initWithObjects:vals.as_ptr()
                                         forKeys:keys.as_ptr()
                                           count:count];
        Id::from_retained_ptr(obj as *mut Self)
    }

    fn from_keys_and_objects<T: INSCopying<Output=Self::Key>>(
            keys: &[&T], vals: Vec<Id<Self::Value, Self::Own>>) -> Id<Self> {
        let vals_refs = vals.as_refs_slice();
        unsafe {
            INSDictionary::from_refs(keys, vals_refs)
        }
    }

    fn into_keys_and_objects(dict: Id<Self>) -> (
            Vec<Id<Self::Key>>, Vec<Id<Self::Value, Self::Own>>) {
        let (keys, objs) = dict.keys_and_objects();
        unsafe {
            (keys.into_id_vec(), objs.into_id_vec())
        }
    }
}

object_struct!(NSDictionary<K, V>);

impl<K: INSObject, V: INSObject> INSDictionary for NSDictionary<K, V> {
    type Key = K;
    type Value = V;
    type Own = Owned;
}

impl<K: INSObject, V: INSObject> Index<K> for NSDictionary<K, V> {
    type Output = V;

    fn index(&self, index: &K) -> &V {
        self.object_for(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use objc::{Id};
    use {INSObject, INSString, NSObject, NSString};
    use super::{INSDictionary, NSDictionary};

    fn sample_dict(key: &str) -> Id<NSDictionary<NSString, NSObject>> {
        let string: Id<NSString> = INSString::from_str(key);
        let obj: Id<NSObject> = INSObject::new();
        INSDictionary::from_keys_and_objects(&[&*string], vec![obj])
    }

    #[test]
    fn test_count() {
        let dict = sample_dict("abcd");
        assert!(dict.count() == 1);
    }

    #[test]
    fn test_object_for() {
        let dict = sample_dict("abcd");

        let string: Id<NSString> = INSString::from_str("abcd");
        assert!(dict.object_for(&*string).is_some());

        let string: Id<NSString> = INSString::from_str("abcde");
        assert!(dict.object_for(&*string).is_none());
    }

    #[test]
    fn test_all_keys() {
        let dict = sample_dict("abcd");
        let keys = dict.all_keys();

        assert!(keys.len() == 1);
        assert!(keys[0].as_str() == "abcd");
    }

    #[test]
    fn test_all_values() {
        let dict = sample_dict("abcd");
        let vals = dict.all_values();

        assert!(vals.len() == 1);
    }

    #[test]
    fn test_keys_and_objects() {
        let dict = sample_dict("abcd");
        let (keys, objs) = dict.keys_and_objects();

        assert!(keys.len() == 1);
        assert!(objs.len() == 1);
        assert!(keys[0].as_str() == "abcd");
        assert!(objs[0] == dict.object_for(keys[0]).unwrap());
    }

    #[test]
    fn test_key_enumerator() {
        let dict = sample_dict("abcd");
        assert!(dict.key_enumerator().count() == 1);
        assert!(dict.key_enumerator().next().unwrap().as_str() == "abcd");
    }

    #[test]
    fn test_object_enumerator() {
        let dict = sample_dict("abcd");
        assert!(dict.object_enumerator().count() == 1);
    }
}
