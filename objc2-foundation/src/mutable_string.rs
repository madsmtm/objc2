use core::cmp;
use core::fmt;
use core::ops::AddAssign;
use core::str;

use objc2::rc::{DefaultId, Id, Owned, Shared};
use objc2::{msg_send, msg_send_id};

use crate::{NSCopying, NSMutableCopying, NSObject, NSString};

extern_class! {
    /// A dynamic plain-text Unicode string object.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsmutablestring?language=objc).
    #[derive(PartialEq, Eq, Hash)]
    unsafe pub struct NSMutableString: NSString, NSObject;
}

// TODO: SAFETY
unsafe impl Sync for NSMutableString {}
unsafe impl Send for NSMutableString {}

/// Creating mutable strings.
impl NSMutableString {
    unsafe_def_fn! {
        /// Construct an empty [`NSMutableString`].
        pub fn new -> Owned;
    }

    /// Creates a new [`NSMutableString`] by copying the given string slice.
    #[doc(alias = "initWithBytes:length:encoding:")]
    #[allow(clippy::should_implement_trait)] // Not really sure of a better name
    pub fn from_str(string: &str) -> Id<Self, Owned> {
        unsafe {
            let obj = super::string::from_str(Self::class(), string);
            Id::new(obj.cast()).unwrap()
        }
    }

    /// Creates a new [`NSMutableString`] from the given [`NSString`].
    #[doc(alias = "initWithString:")]
    pub fn from_nsstring(string: &NSString) -> Id<Self, Owned> {
        unsafe {
            let obj = msg_send_id![Self::class(), alloc];
            msg_send_id![obj, initWithString: string].unwrap()
        }
    }

    #[doc(alias = "initWithCapacity:")]
    pub fn with_capacity(capacity: usize) -> Id<Self, Owned> {
        unsafe {
            let obj = msg_send_id![Self::class(), alloc];
            msg_send_id![obj, initWithCapacity: capacity].unwrap()
        }
    }
}

/// Mutating strings.
impl NSMutableString {
    /// Appends the given [`NSString`] onto the end of this.
    #[doc(alias = "appendString:")]
    pub fn push_nsstring(&mut self, nsstring: &NSString) {
        // SAFETY: The string is not nil
        unsafe { msg_send![self, appendString: nsstring] }
    }

    /// Replaces the entire string.
    #[doc(alias = "setString:")]
    pub fn replace(&mut self, nsstring: &NSString) {
        // SAFETY: The string is not nil
        unsafe { msg_send![self, setString: nsstring] }
    }

    // TODO:
    // - deleteCharactersInRange:
    // - replaceCharactersInRange:withString:
    // - insertString:atIndex:
    // Figure out how these work on character boundaries
}

impl DefaultId for NSMutableString {
    type Ownership = Owned;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

unsafe impl NSCopying for NSMutableString {
    type Ownership = Shared;
    type Output = NSString;
}

unsafe impl NSMutableCopying for NSMutableString {
    type Output = NSMutableString;
}

impl alloc::borrow::ToOwned for NSMutableString {
    type Owned = Id<NSMutableString, Owned>;
    fn to_owned(&self) -> Self::Owned {
        self.mutable_copy()
    }
}

impl AddAssign<&NSString> for NSMutableString {
    #[inline]
    fn add_assign(&mut self, other: &NSString) {
        self.push_nsstring(other)
    }
}

impl PartialEq<NSString> for NSMutableString {
    #[inline]
    fn eq(&self, other: &NSString) -> bool {
        PartialEq::eq(&**self, other)
    }
}

impl PartialEq<NSMutableString> for NSString {
    #[inline]
    fn eq(&self, other: &NSMutableString) -> bool {
        PartialEq::eq(self, &**other)
    }
}

impl PartialOrd for NSMutableString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
}

impl PartialOrd<NSString> for NSMutableString {
    #[inline]
    fn partial_cmp(&self, other: &NSString) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(&**self, other)
    }
}

impl PartialOrd<NSMutableString> for NSString {
    #[inline]
    fn partial_cmp(&self, other: &NSMutableString) -> Option<cmp::Ordering> {
        PartialOrd::partial_cmp(self, &**other)
    }
}

impl Ord for NSMutableString {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        Ord::cmp(&**self, &**other)
    }
}

impl fmt::Write for NSMutableString {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let nsstring = NSString::from_str(s);
        self.push_nsstring(&nsstring);
        Ok(())
    }
}

impl fmt::Display for NSMutableString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl fmt::Debug for NSMutableString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_from_nsstring() {
        let s = NSString::from_str("abc");
        let s = NSMutableString::from_nsstring(&s);
        assert_eq!(&s.to_string(), "abc");
    }

    #[test]
    fn test_push_nsstring() {
        let mut s = NSMutableString::from_str("abc");
        s.push_nsstring(&NSString::from_str("def"));
        *s += &NSString::from_str("ghi");
        assert_eq!(&s.to_string(), "abcdefghi");
    }

    #[test]
    fn test_replace() {
        let mut s = NSMutableString::from_str("abc");
        s.replace(&NSString::from_str("def"));
        assert_eq!(&s.to_string(), "def");
    }

    #[test]
    fn test_with_capacity() {
        let mut s = NSMutableString::with_capacity(3);
        *s += &NSString::from_str("abc");
        *s += &NSString::from_str("def");
        assert_eq!(&s.to_string(), "abcdef");
    }

    #[test]
    fn test_copy() {
        let s1 = NSMutableString::from_str("abc");
        let s2 = s1.copy();
        assert_ne!(Id::as_ptr(&s1), Id::as_ptr(&s2).cast());
        assert!(s2.is_kind_of(NSString::class()));

        let s3 = s1.mutable_copy();
        assert_ne!(Id::as_ptr(&s1), Id::as_ptr(&s3));
        assert!(s3.is_kind_of(NSMutableString::class()));
    }
}
