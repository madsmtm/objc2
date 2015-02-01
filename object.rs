use objc::runtime::Class;
use objc::{Id, Message, ShareId};

use NSString;

pub trait INSObject : Message {
    fn class() -> &'static Class;

    fn hash_code(&self) -> usize {
        unsafe {
            msg_send![self, hash]
        }
    }

    fn is_equal<T>(&self, other: &T) -> bool where T: INSObject {
        let result: i8 = unsafe {
            msg_send![self, isEqual:other]
        };
        result != 0
    }

    fn description(&self) -> ShareId<NSString> {
        unsafe {
            let result: *mut NSString = msg_send![self, description];
            Id::from_ptr(result)
        }
    }

    fn is_kind_of(&self, cls: &Class) -> bool {
        let result: i8 = unsafe {
            msg_send![self, isKindOfClass:cls]
        };
        result != 0
    }

    fn new() -> Id<Self> {
        let cls = <Self as INSObject>::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, init];
            Id::from_retained_ptr(obj)
        }
    }
}

object_struct!(NSObject);

#[cfg(test)]
mod tests {
    use objc::Id;
    use {INSString, NSString};
    use super::{INSObject, NSObject};

    #[test]
    fn test_is_equal() {
        let obj1: Id<NSObject> = INSObject::new();
        assert!(obj1.is_equal(&*obj1));

        let obj2: Id<NSObject> = INSObject::new();
        assert!(!obj1.is_equal(&*obj2));
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
        let expected = format!("<NSObject: {:?}>", &*obj as *const NSObject);
        assert!(description.as_str() == expected.as_slice());
    }

    #[test]
    fn test_is_kind_of() {
        let obj: Id<NSObject> = INSObject::new();
        assert!(obj.is_kind_of(<NSObject as INSObject>::class()));
        assert!(!obj.is_kind_of(<NSString as INSObject>::class()));
    }
}
