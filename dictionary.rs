use std::cmp::min;
use std::mem;
use std::ptr;

use {class, Id, IdVector, IntoIdVector};
use super::{INSArray, INSCopying, INSObject, NSArray, NSEnumerator};

pub trait INSDictionary<K: INSObject, V: INSObject> : INSObject {
	fn count(&self) -> uint {
		let result = unsafe {
			msg_send![self count]
		};
		result as uint
	}

	fn object_for(&self, key: &K) -> Option<&V> {
		unsafe {
			let obj = msg_send![self objectForKey:key.as_ptr()] as *mut V;
			if obj.is_null() {
				None
			} else {
				Some(&*obj)
			}
		}
	}

	fn all_keys(&self) -> Vec<&K> {
		let keys = unsafe {
			&*(msg_send![self allKeys] as *mut NSArray<K>)
		};
		keys.to_vec()
	}

	fn all_values(&self) -> Vec<&V> {
		let vals = unsafe {
			&*(msg_send![self allValues] as *mut NSArray<V>)
		};
		vals.to_vec()
	}

	fn key_enumerator(&self) -> NSEnumerator<K> {
		unsafe {
			let result = msg_send![self keyEnumerator];
			NSEnumerator::from_ptr(result)
		}
	}

	fn object_enumerator(&self) -> NSEnumerator<V> {
		unsafe {
			let result = msg_send![self objectEnumerator];
			NSEnumerator::from_ptr(result)
		}
	}

	fn keys_and_objects(&self) -> (Vec<&K>, Vec<&V>) {
		let len = self.count();
		let keys: Vec<*mut K> = Vec::from_elem(len, ptr::mut_null());
		let objs: Vec<*mut V> = Vec::from_elem(len, ptr::mut_null());
		unsafe {
			msg_send![self getObjects:objs.as_ptr() andKeys:keys.as_ptr()];
			(mem::transmute(keys), mem::transmute(objs))
		}
	}

	unsafe fn from_refs<T: INSCopying<K>>(keys: &[&T], vals: &[&V]) -> Id<Self> {
		let cls = class::<Self>();
		let count = min(keys.len(), vals.len());
		let obj = msg_send![cls alloc];
		let obj = msg_send![obj initWithObjects:vals.as_ptr()
		                                forKeys:keys.as_ptr()
		                                  count:count];
		Id::from_retained_ptr(obj as *mut Self)
	}

	fn from_keys_and_objects<T: INSCopying<K>>(keys: &[&T], vals: Vec<Id<V>>) -> Id<Self> {
		let vals_refs = vals.as_refs_slice();
		unsafe {
			INSDictionary::from_refs(keys, vals_refs)
		}
	}

	fn from_arrays<T: INSCopying<K>>(keys: &NSArray<T>, objs: Id<NSArray<V>>) -> Id<Self> {
		let cls = class::<Self>();
		unsafe {
			let obj = msg_send![cls alloc];
			let obj = msg_send![obj initWithObjects:&*objs forKeys:keys];
			Id::from_retained_ptr(obj as *mut Self)
		}
	}

	fn into_keys_and_objects(dict: Id<Self>) -> (Vec<Id<K>>, Vec<Id<V>>) {
		let (keys, objs) = dict.keys_and_objects();
		unsafe {
			(keys.into_id_vec(), objs.into_id_vec())
		}
	}

	fn into_keys_array(dict: Id<Self>) -> Id<NSArray<K>> {
		unsafe {
			let keys = msg_send![dict allKeys] as *mut NSArray<K>;
			Id::from_ptr(keys)
		}
	}

	fn into_values_array(dict: Id<Self>) -> Id<NSArray<V>> {
		unsafe {
			let vals = msg_send![dict allValues] as *mut NSArray<V>;
			Id::from_ptr(vals)
		}
	}
}

object_struct!(NSDictionary<K, V>)

impl<K: INSObject, V: INSObject> INSDictionary<K, V> for NSDictionary<K, V> { }

impl<K: INSObject, V: INSObject> Collection for NSDictionary<K, V> {
	fn len(&self) -> uint {
		self.count()
	}
}

impl<K: INSObject, V: INSObject> Map<K, V> for NSDictionary<K, V> {
	fn find(&self, key: &K) -> Option<&V> {
		self.object_for(key)
	}
}

impl<K: INSObject, V: INSObject> Index<K, V> for NSDictionary<K, V> {
	fn index(&self, index: &K) -> &V {
		self.object_for(index).unwrap()
	}
}

#[cfg(test)]
mod tests {
	use {Id};
	use foundation::{INSArray, INSObject, INSString, NSArray, NSObject, NSString};
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
		assert!(keys.get(0).as_str() == "abcd");
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
		assert!(keys.get(0).as_str() == "abcd");
		assert!(*objs.get(0) == dict.object_for(*keys.get(0)).unwrap());
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

	#[test]
	fn test_from_arrays() {
		let key: Id<NSString> = INSString::from_str("abcd");
		let keys: Id<NSArray<NSString>> = INSArray::from_vec(vec![key]);
		let val: Id<NSObject> = INSObject::new();
		let vals: Id<NSArray<NSObject>> = INSArray::from_vec(vec![val]);

		let dict: Id<NSDictionary<NSString, NSObject>> =
			INSDictionary::from_arrays(&*keys, vals);
		assert!(dict.count() == 1);
	}
}
