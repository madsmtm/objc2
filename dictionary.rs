use runtime::{Class, Messageable, Object, Sel, objc_msgSend};
use id::{Id, FromId};
use super::INSObject;

pub trait INSDictionary<K: Messageable, V: FromId> : INSObject {
	fn object_for(&self, key: &K) -> Option<V> {
		let object_for = Sel::register("objectForKey:");
		let obj = unsafe {
			objc_msgSend(self.as_ptr(), object_for, key.as_ptr())
		};
		if obj.is_null() {
			None
		} else {
			Some(unsafe { FromId::from_ptr(obj) })
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
		println!("{}", class.name());
		let alloc = Sel::register("alloc");
		let init = Sel::register("init");
		unsafe {
			let obj = objc_msgSend(class.as_ptr(), alloc);
		println!("{}", obj);
			let obj = objc_msgSend(obj, init);
		println!("{}", obj);
			FromId::from_retained_ptr(obj)
		}
	}
}
