use runtime::{Class, Message, ToMessage, object_getClass};
use {class, ClassName, Id};
use super::NSString;

pub trait INSObject : Message {
	fn class_name() -> ClassName<Self>;

	fn class(&self) -> Class {
		unsafe {
			object_getClass(self.as_ptr())
		}
	}

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
			Id::from_ptr(result as *mut NSString)
		}
	}

	fn is_kind_of(&self, cls: Class) -> bool {
		let result = unsafe {
			msg_send![self isKindOfClass:cls]
		};
		!result.is_null()
	}

	fn as_object<T: INSObject>(&self) -> Option<&T> {
		let cls = class::<T>();
		if self.is_kind_of(cls) {
			let ptr = self as *const Self as *mut T;
			Some(unsafe { &*ptr })
		} else {
			None
		}
	}

	fn new() -> Id<Self> {
		let cls = class::<Self>();
		unsafe {
			let obj = msg_send![cls alloc];
			let obj = msg_send![obj init];
			Id::from_retained_ptr(obj as *mut Self)
		}
	}
}

object_struct!(NSObject)

#[cfg(test)]
mod tests {
	use {class, ClassName, Id};
	use foundation::{INSString, NSString};
	use super::{INSObject, NSObject};

	#[test]
	fn test_class_name() {
		let name: ClassName<NSObject> = INSObject::class_name();
		assert!(name.as_str() == "NSObject");
	}

	#[test]
	fn test_class() {
		let obj: Id<NSObject> = INSObject::new();
		let cls = obj.class();
		assert!(cls.name() == "NSObject");
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
		let expected = format!("<NSObject: {}>", obj.deref() as *const NSObject);
		assert!(description.as_str() == expected.as_slice());
	}

	#[test]
	fn test_is_kind_of() {
		let obj: Id<NSObject> = INSObject::new();
		assert!(obj.is_kind_of(class::<NSObject>()));
		assert!(!obj.is_kind_of(class::<NSString>()));
	}

	#[test]
	fn test_as_object() {
		let obj: Id<NSObject> = INSObject::new();
		let as_str: Option<&NSString> = obj.as_object();
		assert!(as_str.is_none());

		let string: Id<NSString> = INSObject::new();
		let as_obj: Option<&NSObject> = string.as_object();
		assert!(as_obj.is_some());
	}
}
