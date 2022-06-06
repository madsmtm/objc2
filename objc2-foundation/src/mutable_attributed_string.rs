use objc2::msg_send;
use objc2::rc::{DefaultId, Id, Owned, Shared};

use crate::{NSAttributedString, NSCopying, NSMutableCopying, NSObject, NSString};

object! {
    /// A mutable string that has associated attributes.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmutableattributedstring?language=objc).
    unsafe pub struct NSMutableAttributedString: NSAttributedString, NSObject;
}

// TODO: SAFETY
unsafe impl Sync for NSMutableAttributedString {}
unsafe impl Send for NSMutableAttributedString {}

/// Creating mutable attributed strings.
impl NSMutableAttributedString {
    unsafe_def_fn! {
        /// Construct an empty mutable attributed string.
        pub fn new -> Owned;
    }

    // TODO: new_with_attributes

    #[doc(alias = "initWithString:")]
    pub fn from_nsstring(string: &NSString) -> Id<Self, Owned> {
        unsafe {
            let obj: *mut Self = msg_send![Self::class(), alloc];
            let obj: *mut Self = msg_send![obj, initWithString: string];
            Id::new(obj).unwrap()
        }
    }

    #[doc(alias = "initWithAttributedString:")]
    pub fn from_attributed_nsstring(attributed_string: &NSAttributedString) -> Id<Self, Owned> {
        unsafe {
            let obj: *mut Self = msg_send![Self::class(), alloc];
            let obj: *mut Self = msg_send![obj, initWithAttributedString: attributed_string];
            Id::new(obj).unwrap()
        }
    }
}

/// Modifying the attributed string.
impl NSMutableAttributedString {
    // TODO
    // - mutableString
    // - replaceCharactersInRange:withString:
    // - setAttributes:range:

    /// Replaces the entire attributed string.
    #[doc(alias = "setAttributedString:")]
    pub fn replace(&mut self, attributed_string: &NSAttributedString) {
        unsafe { msg_send![self, setAttributedString: attributed_string] }
    }
}

impl DefaultId for NSMutableAttributedString {
    type Ownership = Owned;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

unsafe impl NSCopying for NSMutableAttributedString {
    type Ownership = Shared;
    type Output = NSAttributedString;
}

unsafe impl NSMutableCopying for NSMutableAttributedString {
    type Output = NSMutableAttributedString;
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_new() {
        let s = NSAttributedString::new();
        assert_eq!(&s.string().to_string(), "");
    }

    #[test]
    fn test_copy() {
        let s1 = NSMutableAttributedString::from_nsstring(&NSString::from_str("abc"));
        let s2 = s1.copy();
        assert_ne!(Id::as_ptr(&s1) as *mut NSAttributedString, Id::as_ptr(&s2));
        assert!(s2.is_kind_of(NSAttributedString::class()));

        let s3 = s1.mutable_copy();
        assert_ne!(Id::as_ptr(&s1), Id::as_ptr(&s3));
        assert!(s3.is_kind_of(NSMutableAttributedString::class()));
    }
}
