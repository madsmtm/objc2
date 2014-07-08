use runtime::Messageable;
use id::{class, ClassName, Id};
use super::NSString;

pub trait INSObject : Messageable {
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

	fn description(&self) -> Id<NSString> {
		unsafe {
			let result = msg_send![self.as_ptr() description];
			Id::from_ptr(result as *NSString)
		}
	}

	fn new() -> Id<Self> {
		let cls = class::<Self>();
		unsafe {
			let obj = msg_send![cls.as_ptr() alloc];
			let obj = msg_send![obj init];
			Id::from_retained_ptr(obj as *Self)
		}
	}
}

object_struct!(NSObject)
