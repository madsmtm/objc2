use std::cmp::min;
use std::mem;

use {class, Id, IdVector};
use super::{INSCopying, INSObject};

pub trait INSDictionary<K: INSObject, V> : INSObject {
	fn count(&self) -> uint {
		let result = unsafe {
			msg_send![self count]
		};
		result as uint
	}

	fn object_for<'a>(&'a self, key: &K) -> Option<&'a V> {
		unsafe {
			let obj = msg_send![self objectForKey:key.as_ptr()];
			if obj.is_null() {
				None
			} else {
				Some(mem::transmute(obj))
			}
		}
	}

	unsafe fn from_refs<T: INSCopying<K>>(keys: &[&T], vals: &[&V]) -> Id<Self> {
		let cls = class::<Self>();
		let count = min(keys.len(), vals.len());
		let obj = msg_send![cls alloc];
		let obj = msg_send![obj initWithObjects:vals.as_ptr()
		                                forKeys:keys.as_ptr()
		                                  count:count];
		Id::from_retained_ptr(obj as *Self)
	}

	fn from_keys_and_objects<T: INSCopying<K>>(keys: &[&T], vals: Vec<Id<V>>) -> Id<Self> {
		let vals_refs = vals.as_refs_slice();
		unsafe {
			INSDictionary::from_refs(keys, vals_refs)
		}
	}
}

object_struct!(NSDictionary<K, V>)

impl<K: INSObject, V> INSDictionary<K, V> for NSDictionary<K, V> { }

impl<K: INSObject, V> Collection for NSDictionary<K, V> {
	fn len(&self) -> uint {
		self.count()
	}
}

impl<K: INSObject, V> Map<K, V> for NSDictionary<K, V> {
	fn find<'a>(&'a self, key: &K) -> Option<&'a V> {
		self.object_for(key)
	}
}
