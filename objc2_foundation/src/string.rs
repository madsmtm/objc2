use core::ffi::c_void;
use core::fmt;
use core::ptr::NonNull;
use core::slice;
use core::str;
use std::os::raw::c_char;

use objc2::msg_send;
use objc2::rc::{Id, Owned, Shared};

use super::INSObject;

pub trait INSCopying: INSObject {
    /// This can be an [`Owned`] [`INSObject`] if and only if `copy` creates a
    /// new instance, see the following example:
    ///
    /// ```ignore
    /// let x: Id<MyObject, Owned> = MyObject::new();
    /// // This is valid only if `y` is a new instance. Otherwise `x` and `y`
    /// // would be able to create aliasing mutable references!
    /// let y: Id<MyObject, Owned> = x.copy();
    /// ```
    type Output: INSObject;

    fn copy(&self) -> Id<Self::Output, <Self::Output as INSObject>::Ownership> {
        unsafe {
            let obj: *mut Self::Output = msg_send![self, copy];
            Id::new(NonNull::new_unchecked(obj))
        }
    }
}

pub trait INSMutableCopying: INSObject {
    /// An [`Owned`] [`INSObject`] is required to be able to return an owned
    /// [`Id`].
    type Output: INSObject<Ownership = Owned>;

    fn mutable_copy(&self) -> Id<Self::Output, Owned> {
        unsafe {
            let obj: *mut Self::Output = msg_send![self, mutableCopy];
            Id::new(NonNull::new_unchecked(obj))
        }
    }
}

#[cfg(target_vendor = "apple")]
const UTF8_ENCODING: usize = 4;
#[cfg(not(target_vendor = "apple"))]
const UTF8_ENCODING: i32 = 4;

pub trait INSString: INSObject {
    fn len(&self) -> usize {
        unsafe { msg_send![self, lengthOfBytesUsingEncoding: UTF8_ENCODING] }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn as_str(&self) -> &str {
        let bytes = unsafe {
            let bytes: *const c_char = msg_send![self, UTF8String];
            bytes as *const u8
        };
        let len = self.len();
        unsafe {
            let bytes = slice::from_raw_parts(bytes, len);
            str::from_utf8(bytes).unwrap()
        }
    }

    fn from_str(string: &str) -> Id<Self, Self::Ownership> {
        let cls = Self::class();
        let bytes = string.as_ptr() as *const c_void;
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![
                obj,
                initWithBytes: bytes,
                length: string.len(),
                encoding: UTF8_ENCODING,
            ];
            Id::new(NonNull::new_unchecked(obj))
        }
    }
}

object_struct!(NSString, Shared);

impl INSString for NSString {}

impl INSCopying for NSString {
    type Output = NSString;
}

impl fmt::Display for NSString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

#[cfg(test)]
mod tests {
    use super::{INSCopying, INSString, NSString};

    #[cfg(not(target_vendor = "apple"))]
    #[test]
    fn ensure_linkage() {
        unsafe { objc2::__gnustep_hack::get_class_to_force_linkage() };
    }

    #[test]
    fn test_utf8() {
        let expected = "ประเทศไทย中华Việt Nam";
        let s = NSString::from_str(expected);
        assert!(s.len() == expected.len());
        assert!(s.as_str() == expected);
    }

    #[test]
    fn test_interior_nul() {
        let expected = "Hello\0World";
        let s = NSString::from_str(expected);
        assert!(s.len() == expected.len());
        assert!(s.as_str() == expected);
    }

    #[test]
    fn test_copy() {
        let s = NSString::from_str("Hello!");
        let copied = s.copy();
        assert!(copied.as_str() == s.as_str());
    }
}
