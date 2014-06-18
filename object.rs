use std::hash;
use std::mem;

use runtime::{Class, Messageable, Object, Sel, objc_msgSend};
use id::{Id, FromId};

pub trait INSObject : Messageable {
	fn hash_code(&self) -> uint {
		let hash = Sel::register("hash");
		unsafe {
			let result = objc_msgSend(self.as_ptr(), hash);
			mem::transmute(result)
		}
	}

	fn is_equal<T: INSObject>(&self, other: &T) -> bool {
		let is_equal = Sel::register("isEqual:");
		let result = unsafe {
			objc_msgSend(self.as_ptr(), is_equal, other.as_ptr())
		};
		!result.is_null()
	}
}

#[deriving(Clone)]
pub struct NSObject {
	ptr: Id,
}

impl Messageable for NSObject {
	unsafe fn as_ptr(&self) -> *Object {
		self.ptr.as_ptr()
	}
}

impl FromId for NSObject {
	unsafe fn from_id(id: Id) -> NSObject {
		NSObject { ptr: id }
	}
}

impl INSObject for NSObject { }

impl PartialEq for NSObject {
	fn eq(&self, other: &NSObject) -> bool {
		self.is_equal(other)
	}
}

impl Eq for NSObject { }

impl<S: hash::Writer> hash::Hash<S> for NSObject {
	fn hash(&self, state: &mut S) {
		self.hash_code().hash(state);
	}
}

impl NSObject {
	fn class() -> Class {
		Class::get("NSObject")
	}

	pub fn new() -> NSObject {
		let class = NSObject::class();
		let alloc = Sel::register("alloc");
		let init = Sel::register("init");
		unsafe {
			let obj = objc_msgSend(class.as_ptr(), alloc);
			let obj = objc_msgSend(obj, init);
			FromId::from_retained_ptr(obj)
		}
	}
}
