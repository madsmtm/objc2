use runtime::Messageable;
use id::{class, ClassName, FromId};
use super::{INSString, NSString};

pub trait INSObject : Messageable + FromId {
	fn class_name() -> ClassName<Self>;

	fn hash_code(&self) -> uint {
		let result = unsafe {
			msg_send![self.as_ptr() hash]
		};
		result as uint
	}

	fn is_equal<T: INSObject>(&self, other: &T) -> bool {
		let result = unsafe {
			msg_send![self.as_ptr() isEqual:other.as_ptr()]
		};
		!result.is_null()
	}

	fn description(&self) -> NSString {
		unsafe {
			let result = msg_send![self.as_ptr() description];
			FromId::from_ptr(result)
		}
	}

	fn new() -> Self {
		let cls = class::<Self>();
		unsafe {
			let obj = msg_send![cls.as_ptr() alloc];
			let obj = msg_send![obj init];
			FromId::from_retained_ptr(obj)
		}
	}
}

object_struct!(NSObject)
