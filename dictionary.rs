use std::cmp::min;
use std::mem;

use runtime::{Class, Messageable, Object, Sel, objc_msgSend};
use id::{Id, FromId};
use super::{INSCopying, INSObject};

pub trait INSDictionary<K: Messageable, V: FromId> : INSObject {
	fn count(&self) -> uint {
		let count = Sel::register("count");
		unsafe {
			let result = objc_msgSend(self.as_ptr(), count);
			mem::transmute(result)
		}
	}

	fn object_for(&self, key: &K) -> Option<V> {
		let object_for = Sel::register("objectForKey:");
		unsafe {
			let obj = objc_msgSend(self.as_ptr(), object_for, key.as_ptr());
			FromId::maybe_from_ptr(obj)
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

impl<K, V> INSObject for NSDictionary<K, V> { }

impl<K: Messageable, V: FromId> INSDictionary<K, V> for NSDictionary<K, V> { }

impl<K, V> NSDictionary<K, V> {
	fn class() -> Class {
		Class::get("NSDictionary")
	}

	pub fn new() -> NSDictionary<K, V> {
		let class = NSDictionary::<K, V>::class();
		let alloc = Sel::register("alloc");
		let init = Sel::register("init");
		unsafe {
			let obj = objc_msgSend(class.as_ptr(), alloc);
			let obj = objc_msgSend(obj, init);
			FromId::from_retained_ptr(obj)
		}
	}

	unsafe fn from_ptrs(keys: &[*Object], vals: &[*Object]) -> NSDictionary<K, V> {
		let class = NSDictionary::<K, V>::class();
		let alloc = Sel::register("alloc");
		let init = Sel::register("initWithObjects:forKeys:count:");
		let count = min(keys.len(), vals.len());

		let obj = objc_msgSend(class.as_ptr(), alloc);
		let obj = objc_msgSend(obj, init, vals.as_ptr(), keys.as_ptr(), count);
		FromId::from_retained_ptr(obj)
	}
}

impl<K, V: Messageable> NSDictionary<K, V> {
	pub fn from_keys_and_objects<T: INSCopying<K>>(keys: &[T], vals: &[V]) -> NSDictionary<K, V> {
		let mut key_ptrs = Vec::with_capacity(keys.len());
		for key in keys.iter() {
			key_ptrs.push(unsafe { key.as_ptr() });
		}
		let mut val_ptrs = Vec::with_capacity(vals.len());
		for val in vals.iter() {
			val_ptrs.push(unsafe { val.as_ptr() });
		}
		unsafe {
			NSDictionary::from_ptrs(key_ptrs.as_slice(), val_ptrs.as_slice())
		}
	}
}

impl<K: Messageable, V: FromId> Collection for NSDictionary<K, V> {
	fn len(&self) -> uint {
		self.count()
	}
}
