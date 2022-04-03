use objc2::msg_send;
use objc2::rc::{DefaultId, Id, Owned, Shared};
use objc2::runtime::{Bool, Class, Object};

use super::NSString;

object! {
    @__inner
    unsafe pub struct NSObject<>: Object {}
}

impl NSObject {
    unsafe_def_fn!(pub fn new -> Owned);

    pub fn hash_code(&self) -> usize {
        unsafe { msg_send![self, hash] }
    }

    pub fn is_equal(&self, other: &NSObject) -> bool {
        let result: Bool = unsafe { msg_send![self, isEqual: other] };
        result.as_bool()
    }

    pub fn description(&self) -> Id<NSString, Shared> {
        unsafe {
            let result: *mut NSString = msg_send![self, description];
            // TODO: Verify that description always returns a non-null string
            Id::retain_autoreleased(result).unwrap()
        }
    }

    pub fn is_kind_of(&self, cls: &Class) -> bool {
        let result: Bool = unsafe { msg_send![self, isKindOfClass: cls] };
        result.as_bool()
    }
}

impl DefaultId for NSObject {
    type Ownership = Owned;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::NSObject;
    use crate::NSString;
    use alloc::format;

    #[test]
    fn test_equality() {
        let obj1 = NSObject::new();
        assert_eq!(obj1, obj1);

        let obj2 = NSObject::new();
        assert_ne!(obj1, obj2);
    }

    #[test]
    fn test_hash_code() {
        let obj = NSObject::new();
        assert_eq!(obj.hash_code(), obj.hash_code());
    }

    #[test]
    fn test_debug() {
        let obj = NSObject::new();
        let expected = format!("<NSObject: {:p}>", &*obj);
        assert_eq!(format!("{:?}", obj), format!("{:?}", expected));
    }

    #[test]
    fn test_is_kind_of() {
        let obj = NSObject::new();
        assert!(obj.is_kind_of(NSObject::class()));
        assert!(!obj.is_kind_of(NSString::class()));
    }
}
