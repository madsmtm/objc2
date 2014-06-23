use std::cmp::min;

use runtime::{Messageable, Object, Sel, objc_msgSend};
use id::{class, ClassName, Id, FromId};
use super::{INSCopying, INSObject};

pub trait INSDictionary<K: Messageable, V: INSObject> : INSObject {
	fn count(&self) -> uint {
		let count = Sel::register("count");
		let result = unsafe {
			objc_msgSend(self.as_ptr(), count)
		};
		result as uint
	}

	fn object_for(&self, key: &K) -> Option<V> {
		let object_for = Sel::register("objectForKey:");
		unsafe {
			let obj = objc_msgSend(self.as_ptr(), object_for, key.as_ptr());
			FromId::maybe_from_ptr(obj)
		}
	}

	unsafe fn from_ptrs(keys: &[*Object], vals: &[*Object]) -> Self {
		let class = class::<Self>();
		let alloc = Sel::register("alloc");
		let init = Sel::register("initWithObjects:forKeys:count:");
		let count = min(keys.len(), vals.len());

		let obj = objc_msgSend(class.as_ptr(), alloc);
		let obj = objc_msgSend(obj, init, vals.as_ptr(), keys.as_ptr(), count);
		FromId::from_retained_ptr(obj)
	}

	fn from_keys_and_objects<T: INSCopying<K>>(keys: &[T], vals: &[V]) -> Self {
		let mut key_ptrs = Vec::with_capacity(keys.len());
		for key in keys.iter() {
			key_ptrs.push(unsafe { key.as_ptr() });
		}
		let mut val_ptrs = Vec::with_capacity(vals.len());
		for val in vals.iter() {
			val_ptrs.push(unsafe { val.as_ptr() });
		}
		unsafe {
			INSDictionary::from_ptrs(key_ptrs.as_slice(), val_ptrs.as_slice())
		}
	}
}

#[deriving(Clone)]
pub struct NSDictionary<K, V> {
	ptr: Id,
}

impl<K, V> Messageable for NSDictionary<K, V> {
	unsafe fn as_ptr(&self) -> *Object {
		self.ptr.as_ptr()
	}
}

impl<K, V> FromId for NSDictionary<K, V> {
	unsafe fn from_id(id: Id) -> NSDictionary<K, V> {
		NSDictionary { ptr: id }
	}
}

impl<K, V> INSObject for NSDictionary<K, V> {
	fn class_name() -> ClassName<NSDictionary<K, V>> {
		ClassName::from_str("NSDictionary")
	}
}

impl<K: Messageable, V: INSObject> INSDictionary<K, V> for NSDictionary<K, V> { }

impl<K: Messageable, V: INSObject> Collection for NSDictionary<K, V> {
	fn len(&self) -> uint {
		self.count()
	}
}
