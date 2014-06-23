use runtime::{Messageable, Sel, objc_msgSend};
use id::{class, ClassName, FromId};
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

object_struct!(NSObject)
