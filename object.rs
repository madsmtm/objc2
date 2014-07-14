use runtime::Messageable;
use {class, ClassName, Id};
use super::NSString;

pub trait INSObject : Messageable {
	fn class_name() -> ClassName<Self>;

	fn hash_code(&self) -> uint {
		let result = unsafe {
			msg_send![self hash]
		};
		result as uint
	}

	fn is_equal<T: INSObject>(&self, other: &T) -> bool {
		let result = unsafe {
			msg_send![self isEqual:other.as_ptr()]
		};
		!result.is_null()
	}

	fn description(&self) -> Id<NSString> {
		unsafe {
			let result = msg_send![self description];
			Id::from_ptr(result as *NSString)
		}
	}

	fn new() -> Id<Self> {
		let cls = class::<Self>();
		unsafe {
			let obj = msg_send![cls alloc];
			let obj = msg_send![obj init];
			Id::from_retained_ptr(obj as *Self)
		}
	}
}

object_struct!(NSObject)

#[cfg(test)]
mod tests {
	use {ClassName, Id};
	use foundation::INSString;
	use super::{INSObject, NSObject};

	#[test]
	fn test_class_name() {
		let name: ClassName<NSObject> = INSObject::class_name();
		assert!(name.as_str() == "NSObject");
	}

	#[test]
	fn test_is_equal() {
		let obj1: Id<NSObject> = INSObject::new();
		assert!(obj1.is_equal(obj1.deref()));

		let obj2: Id<NSObject> = INSObject::new();
		assert!(!obj1.is_equal(obj2.deref()));
	}

	#[test]
	fn test_hash_code() {
		let obj: Id<NSObject> = INSObject::new();
		assert!(obj.hash_code() == obj.hash_code());
	}

	#[test]
	fn test_description() {
		let obj: Id<NSObject> = INSObject::new();
		let description = obj.description();
		let expected = format!("<NSObject: {}>", obj.deref() as *NSObject);
		assert!(description.as_str() == expected.as_slice());
	}
}
