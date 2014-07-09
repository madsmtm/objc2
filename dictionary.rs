use std::cmp::min;
use std::mem;

use runtime::Messageable;
use id::{class, Id, IdVector};
use super::{INSCopying, INSObject};

pub trait INSDictionary<K: Messageable, V> : INSObject {
	fn count(&self) -> uint {
		let result = unsafe {
			msg_send![self.as_ptr() count]
		};
		result as uint
	}

	fn object_for<'a>(&'a self, key: &K) -> Option<&'a V> {
		unsafe {
			let obj = msg_send![self.as_ptr() objectForKey:key.as_ptr()];
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
		let obj = msg_send![cls.as_ptr() alloc];
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

impl<K: Messageable, V> INSDictionary<K, V> for NSDictionary<K, V> { }

impl<K: Messageable, V> Collection for NSDictionary<K, V> {
	fn len(&self) -> uint {
		self.count()
	}
}
