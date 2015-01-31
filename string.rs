use std::mem;
use std::slice;
use std::str;

use objc::{Id, ShareId};

use INSObject;

pub trait INSCopying : INSObject {
    type Output: INSObject;

    fn copy(&self) -> ShareId<Self::Output> {
        unsafe {
            let obj: *mut Self::Output = msg_send![self, copy];
            Id::from_retained_ptr(obj)
        }
    }
}

pub trait INSMutableCopying : INSObject {
    type Output: INSObject;

    fn mutable_copy(&self) -> Id<Self::Output> {
        unsafe {
            let obj: *mut Self::Output = msg_send![self, mutableCopy];
            Id::from_retained_ptr(obj)
        }
    }
}

static UTF8_ENCODING: usize = 4;

pub trait INSString : INSObject {
    fn len(&self) -> usize {
        unsafe {
            msg_send![self, lengthOfBytesUsingEncoding:UTF8_ENCODING]
        }
    }

    fn as_str(&self) -> &str {
        let bytes = unsafe {
            let bytes: *const i8 = msg_send![self, UTF8String];
            bytes as *const u8
        };
        let len = self.len();
        unsafe {
            let bytes = slice::from_raw_buf(&bytes, len);
            let s = str::from_utf8(bytes).unwrap();
            mem::transmute(s)
        }
    }

    fn from_str(string: &str) -> Id<Self> {
        let cls = <Self as INSObject>::class();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithBytes:string.as_ptr()
                                                       length:string.len()
                                                     encoding:UTF8_ENCODING];
            Id::from_retained_ptr(obj)
        }
    }
}

object_struct!(NSString);

impl INSString for NSString { }

impl INSCopying for NSString {
    type Output = NSString;
}

impl Str for NSString {
    fn as_slice(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use objc::Id;
    use super::{INSCopying, INSString, NSString};

    #[test]
    fn test_utf8() {
        let expected = "ประเทศไทย中华Việt Nam";
        let s: Id<NSString> = INSString::from_str(expected);
        assert!(s.len() == expected.len());
        assert!(s.as_str() == expected);
    }

    #[test]
    fn test_interior_nul() {
        let expected = "Hello\0World";
        let s: Id<NSString> = INSString::from_str(expected);
        assert!(s.len() == expected.len());
        assert!(s.as_str() == expected);
    }

    #[test]
    fn test_copy() {
        let s: Id<NSString> = INSString::from_str("Hello!");
        let copied = s.copy();
        assert!(copied.as_str() == s.as_str());
    }
}
