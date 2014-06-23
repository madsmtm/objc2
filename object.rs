use std::fmt;
use std::hash;

use runtime::{Messageable, Object, Sel, objc_msgSend};
use id::{class, ClassName, Id, FromId};
use super::{INSString, NSString};

pub trait INSObject : Messageable + FromId {
	fn class_name() -> ClassName<Self>;

	fn hash_code(&self) -> uint {
		let hash = Sel::register("hash");
		let result = unsafe {
			objc_msgSend(self.as_ptr(), hash)
		};
		result as uint
	}

	fn is_equal<T: INSObject>(&self, other: &T) -> bool {
		let is_equal = Sel::register("isEqual:");
		let result = unsafe {
			objc_msgSend(self.as_ptr(), is_equal, other.as_ptr())
		};
		!result.is_null()
	}

	fn description(&self) -> NSString {
		let description = Sel::register("description");
		unsafe {
			let result = objc_msgSend(self.as_ptr(), description);
			FromId::from_ptr(result)
		}
	}

	fn new() -> Self {
		let cls = class::<Self>();
		let alloc = Sel::register("alloc");
		let init = Sel::register("init");
		unsafe {
			let obj = objc_msgSend(cls.as_ptr(), alloc);
			let obj = objc_msgSend(obj, init);
			FromId::from_retained_ptr(obj)
		}
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

impl INSObject for NSObject {
	fn class_name() -> ClassName<NSObject> {
		ClassName::from_str("NSObject")
	}
}

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

impl fmt::Show for NSObject {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		self.description().as_str().fmt(f)
	}
}
