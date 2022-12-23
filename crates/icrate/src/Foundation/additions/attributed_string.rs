use core::panic::{RefUnwindSafe, UnwindSafe};

use objc2::rc::{DefaultId, Id, Shared};
use objc2::runtime::Object;
use objc2::{extern_methods, ClassType};

use crate::Foundation::{
    NSAttributedString, NSAttributedStringKey, NSCopying, NSDictionary, NSMutableAttributedString,
    NSMutableCopying, NSString,
};

// SAFETY: `NSAttributedString` is immutable and `NSMutableAttributedString`
// can only be mutated from `&mut` methods.
unsafe impl Sync for NSAttributedString {}
unsafe impl Send for NSAttributedString {}

// Same reasoning as `NSString`.
impl UnwindSafe for NSAttributedString {}
impl RefUnwindSafe for NSAttributedString {}

extern_methods!(
    /// Creating attributed strings.
    unsafe impl NSAttributedString {
        /// Construct an empty attributed string.
        #[method_id(new)]
        pub fn new() -> Id<Self, Shared>;

        /// Creates a new attributed string from the given string and attributes.
        ///
        /// The attributes are associated with every UTF-16 code unit in the
        /// string.
        ///
        /// # Safety
        ///
        /// The attributes must be valid.
        #[doc(alias = "initWithString:")]
        pub unsafe fn new_with_attributes(
            string: &NSString,
            attributes: &NSDictionary<NSAttributedStringKey, Object>,
        ) -> Id<Self, Shared> {
            unsafe { Self::initWithString_attributes(Self::alloc(), string, Some(attributes)) }
        }

        /// Creates a new attributed string without any attributes.
        #[doc(alias = "initWithString:")]
        pub fn from_nsstring(string: &NSString) -> Id<Self, Shared> {
            Self::initWithString(Self::alloc(), string)
        }
    }
);

impl DefaultId for NSAttributedString {
    type Ownership = Shared;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

unsafe impl NSCopying for NSAttributedString {
    type Ownership = Shared;
    type Output = NSAttributedString;
}

unsafe impl NSMutableCopying for NSAttributedString {
    type Output = NSMutableAttributedString;
}

impl alloc::borrow::ToOwned for NSAttributedString {
    type Owned = Id<NSAttributedString, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use alloc::{format, vec};

    use objc2::rc::{autoreleasepool, Owned};

    use super::*;
    use crate::Foundation::NSObject;

    #[test]
    fn test_new() {
        let s = NSAttributedString::new();
        assert_eq!(&s.string().to_string(), "");
    }

    #[test]
    fn test_string_bound_to_attributed() {
        let attr_s = {
            let source = NSString::from_str("Hello world!");
            NSAttributedString::from_nsstring(&source)
        };
        let s = autoreleasepool(|_| attr_s.string());
        assert_eq!(s.len(), 12);
    }

    #[test]
    fn test_from_nsstring() {
        let s = NSAttributedString::from_nsstring(&NSString::from_str("abc"));
        assert_eq!(&s.string().to_string(), "abc");
    }

    #[test]
    fn test_copy() {
        let s1 = NSAttributedString::from_nsstring(&NSString::from_str("abc"));
        let s2 = s1.copy();
        // NSAttributedString performs this optimization in GNUStep's runtime,
        // but not in Apple's; so we don't test for it!
        // assert_eq!(Id::as_ptr(&s1), Id::as_ptr(&s2));
        assert!(s2.is_kind_of::<NSAttributedString>());

        let s3 = s1.mutable_copy();
        assert_ne!(Id::as_ptr(&s1), Id::as_ptr(&s3).cast());
        assert!(s3.is_kind_of::<NSMutableAttributedString>());
    }

    #[test]
    fn test_debug() {
        let s = NSAttributedString::from_nsstring(&NSString::from_str("abc"));
        let expected = if cfg!(feature = "gnustep-1-7") {
            "abc{}"
        } else {
            "abc{\n}"
        };
        assert_eq!(format!("{s:?}"), expected);

        let obj: Id<Object, Owned> = unsafe { Id::cast(NSObject::new()) };
        let ptr: *const Object = &*obj;
        let s = unsafe {
            NSAttributedString::new_with_attributes(
                &NSString::from_str("abc"),
                &NSDictionary::from_keys_and_objects(&[&*NSString::from_str("test")], vec![obj]),
            )
        };
        let expected = if cfg!(feature = "gnustep-1-7") {
            format!("abc{{test = \"<NSObject: {ptr:?}>\"; }}")
        } else {
            format!("abc{{\n    test = \"<NSObject: {ptr:?}>\";\n}}")
        };
        assert_eq!(format!("{s:?}"), expected);
    }
}
