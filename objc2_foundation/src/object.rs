use core::ptr::NonNull;

use objc2::msg_send;
use objc2::rc::{Id, Owned, Ownership, Shared};
use objc2::runtime::{Bool, Class};
use objc2::Message;

use super::NSString;

/*
The Sized bound is unfortunate; ideally, Objective-C objects would not be
treated as Sized. However, rust won't allow casting a dynamically-sized type
pointer to an Object pointer, because dynamically-sized types can have fat
pointers (two words) instead of real pointers.
*/
pub unsafe trait INSObject: Sized + Message {
    /// Indicates whether the type is mutable or immutable.
    ///
    /// [`Shared`] means that only a shared [`Id`] can ever be held to this
    /// object. This is important for immutable types like `NSString`, because
    /// sending the `copy` message (and others) does not create a new
    /// instance, but instead just retains the instance.
    ///
    /// Most objects are mutable and hence can return [`Owned`] [`Id`]s.
    type Ownership: Ownership;

    fn class() -> &'static Class;

    fn hash_code(&self) -> usize {
        unsafe { msg_send![self, hash] }
    }

    fn is_equal<T: INSObject>(&self, other: &T) -> bool {
        let result: Bool = unsafe { msg_send![self, isEqual: other] };
        result.is_true()
    }

    fn description(&self) -> Id<NSString, Shared> {
        unsafe {
            let result: *mut NSString = msg_send![self, description];
            // TODO: Verify that description always returns a non-null string
            Id::retain(NonNull::new_unchecked(result))
        }
    }

    fn is_kind_of(&self, cls: &Class) -> bool {
        let result: Bool = unsafe { msg_send![self, isKindOfClass: cls] };
        result.is_true()
    }
}

object_struct!(unsafe NSObject, Owned);

impl NSObject {
    unsafe_def_fn!(pub fn new);
}

#[cfg(test)]
mod tests {
    use super::{INSObject, NSObject};
    use crate::{INSString, NSString};
    use alloc::format;
    use objc2::rc::autoreleasepool;

    #[test]
    fn test_is_equal() {
        let obj1 = NSObject::new();
        assert!(obj1.is_equal(&*obj1));
        assert_eq!(obj1, obj1); // Using forwarding impl on Id

        let obj2 = NSObject::new();
        assert!(!obj1.is_equal(&*obj2));
        assert_ne!(obj1, obj2);
    }

    #[test]
    fn test_hash_code() {
        let obj = NSObject::new();
        assert_eq!(obj.hash_code(), obj.hash_code());
    }

    #[test]
    fn test_description() {
        let obj = NSObject::new();
        let description = obj.description();
        let expected = format!("<NSObject: {:p}>", &*obj);
        autoreleasepool(|pool| {
            assert_eq!(description.as_str(pool), &*expected);
        });

        let expected = format!("\"<NSObject: {:p}>\"", &*obj);
        assert_eq!(format!("{:?}", obj), expected);
    }

    #[test]
    fn test_is_kind_of() {
        let obj = NSObject::new();
        assert!(obj.is_kind_of(NSObject::class()));
        assert!(!obj.is_kind_of(NSString::class()));
    }
}
