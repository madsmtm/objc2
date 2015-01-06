use objc::runtime::{Class, Object};
use objc::{Id, Message};

use NSString;

pub trait INSObject : Message {
    fn class_name() -> ClassName<Self>;

    fn class(&self) -> &Class {
        let obj = unsafe { &*(self as *const _ as *const Object) };
        obj.class()
    }

    fn hash_code(&self) -> uint {
        let result = unsafe {
            msg_send![self hash]
        };
        result as uint
    }

    fn is_equal<T: INSObject>(&self, other: &T) -> bool {
        let result = unsafe {
            msg_send![self isEqual:other]
        };
        !result.is_null()
    }

    fn description(&self) -> Id<NSString> {
        unsafe {
            let result = msg_send![self description];
            Id::from_ptr(result as *mut NSString)
        }
    }

    fn is_kind_of(&self, cls: &Class) -> bool {
        let result = unsafe {
            msg_send![self isKindOfClass:cls]
        };
        !result.is_null()
    }

    fn as_object<T: INSObject>(&self) -> Option<&T> {
        let cls = class::<T>();
        if self.is_kind_of(cls) {
            let ptr = self as *const _ as *const T;
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

object_struct!(NSObject);

pub struct ClassName<T>(pub &'static str);

pub fn class<T: INSObject>() -> &'static Class {
    let ClassName(name): ClassName<T> = INSObject::class_name();
    match Class::get(name) {
        Some(cls) => cls,
        None => panic!("Class {} not found", name),
    }
}

#[cfg(test)]
mod tests {
    use objc::Id;
    use {INSString, NSString};
    use super::{class, ClassName, INSObject, NSObject};

    #[test]
    fn test_class_name() {
        let ClassName(name): ClassName<NSObject> = INSObject::class_name();
        assert!(name == "NSObject");
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
        let expected = format!("<NSObject: {}>", &*obj as *const NSObject);
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
