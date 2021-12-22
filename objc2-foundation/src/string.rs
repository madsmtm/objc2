use core::ffi::c_void;
use core::fmt;
use core::ptr::NonNull;
use core::slice;
use core::str;
use std::os::raw::c_char;

use alloc::borrow::ToOwned;
use objc2::ffi;
use objc2::msg_send;
use objc2::rc::{autoreleasepool, AutoreleasePool};
use objc2::rc::{Id, Shared};

use super::{INSCopying, INSObject};

#[cfg(apple)]
const UTF8_ENCODING: usize = 4;
#[cfg(gnustep)]
const UTF8_ENCODING: i32 = 4;

#[allow(unused)]
#[allow(non_upper_case_globals)]
const NSNotFound: ffi::NSInteger = ffi::NSIntegerMax;

pub unsafe trait INSString: INSObject {
    fn len(&self) -> usize {
        unsafe { msg_send![self, lengthOfBytesUsingEncoding: UTF8_ENCODING] }
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// TODO
    ///
    /// ```compile_fail
    /// # use objc2::rc::autoreleasepool;
    /// # use objc2_foundation::{INSObject, INSString, NSString};
    /// autoreleasepool(|pool| {
    ///     let ns_string = NSString::new();
    ///     let s = ns_string.as_str(pool);
    ///     drop(ns_string);
    ///     println!("{}", s);
    /// });
    /// ```
    ///
    /// ```compile_fail
    /// # use objc2::rc::autoreleasepool;
    /// # use objc2_foundation::{INSObject, INSString, NSString};
    /// let ns_string = NSString::new();
    /// let s = autoreleasepool(|pool| ns_string.as_str(pool));
    /// ```
    fn as_str<'r, 's: 'r, 'p: 'r>(&'s self, pool: &'p AutoreleasePool) -> &'r str {
        // This is necessary until `auto` types stabilizes.
        pool.__verify_is_inner();

        // The documentation on `UTF8String` is a bit sparse, but with
        // educated guesses and testing I've determined that NSString stores
        // a pointer to the string data, sometimes with an UTF-8 encoding,
        // (usual for ascii data), sometimes in other encodings (UTF-16?).
        //
        // `UTF8String` then checks the internal encoding:
        // - If the data is UTF-8 encoded, it returns the internal pointer.
        // - If the data is in another encoding, it creates a new allocation,
        //   writes the UTF-8 representation of the string into it,
        //   autoreleases the allocation and returns a pointer to it.
        //
        // So the lifetime of the returned pointer is either the same as the
        // NSString OR the lifetime of the innermost @autoreleasepool.
        let bytes: *const c_char = unsafe { msg_send![self, UTF8String] };
        let bytes = bytes as *const u8;
        let len = self.len();

        // SAFETY:
        // The held AutoreleasePool is the innermost, and the reference is
        // constrained both by the pool and the NSString.
        //
        // `len` is the length of the string in the UTF-8 encoding.
        //
        // `bytes` is a null-terminated C string (with length = len + 1), so
        // it is never a NULL pointer.
        let bytes: &'r [u8] = unsafe { slice::from_raw_parts(bytes, len) };

        // TODO: Always UTF-8, so should we use `from_utf8_unchecked`?
        str::from_utf8(bytes).unwrap()
    }

    fn from_str(string: &str) -> Id<Self, Shared> {
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

object!(unsafe pub struct NSString);

// TODO: SAFETY
unsafe impl Sync for NSString {}
unsafe impl Send for NSString {}

impl NSString {
    unsafe_def_fn!(pub fn new -> Shared);
}

unsafe impl INSString for NSString {}

unsafe impl INSCopying for NSString {
    type Ownership = Shared;
    type Output = NSString;
}

impl fmt::Display for NSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The call to `to_owned` is unfortunate, but is required to work
        // around `f` not being AutoreleaseSafe.
        // TODO: Fix this!
        let s = autoreleasepool(|pool| self.as_str(pool).to_owned());
        fmt::Display::fmt(&s, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[cfg(gnustep)]
    #[test]
    fn ensure_linkage() {
        unsafe { objc2::__gnustep_hack::get_class_to_force_linkage() };
    }

    #[test]
    fn test_equality() {
        let s1 = NSString::from_str("abc");
        let s2 = NSString::from_str("abc");
        assert_eq!(s1, s1);
        assert_eq!(s1, s2);

        let s3 = NSString::from_str("def");
        assert_ne!(s1, s3);
    }

    #[test]
    fn test_debug_display() {
        let s = "xyz123";
        let obj = NSString::from_str(s);
        assert_eq!(format!("{:?}", obj), format!("{:?}", s));
        assert_eq!(format!("{}", obj), format!("{}", s));
    }

    #[test]
    fn test_empty() {
        let s1 = NSString::from_str("");
        let s2 = NSString::new();

        assert_eq!(s1.len(), 0);
        assert_eq!(s2.len(), 0);

        assert_eq!(s1, s2);

        autoreleasepool(|pool| {
            assert_eq!(s1.as_str(pool), "");
            assert_eq!(s2.as_str(pool), "");
        });
    }

    #[test]
    fn test_utf8() {
        let expected = "ประเทศไทย中华Việt Nam";
        let s = NSString::from_str(expected);
        assert_eq!(s.len(), expected.len());
        autoreleasepool(|pool| {
            assert_eq!(s.as_str(pool), expected);
        });
    }

    #[test]
    fn test_interior_nul() {
        let expected = "Hello\0World";
        let s = NSString::from_str(expected);
        assert_eq!(s.len(), expected.len());
        autoreleasepool(|pool| {
            assert_eq!(s.as_str(pool), expected);
        });
    }

    #[test]
    fn test_copy() {
        let s = NSString::from_str("Hello!");
        let copied = s.copy();
        autoreleasepool(|pool| {
            assert_eq!(copied.as_str(pool), s.as_str(pool));
        });
    }

    #[test]
    fn test_copy_nsstring_is_same() {
        let string1 = NSString::from_str("Hello, world!");
        let string2 = string1.copy();

        let s1: *const NSString = &*string1;
        let s2: *const NSString = &*string2;

        assert_eq!(s1, s2, "Cloned NSString didn't have the same address");
    }

    #[test]
    /// Apparently NSString does this for some reason?
    fn test_strips_first_leading_zero_width_no_break_space() {
        let ns_string = NSString::from_str("\u{feff}");
        let expected = "";
        autoreleasepool(|pool| {
            assert_eq!(ns_string.as_str(pool), expected);
        });
        assert_eq!(ns_string.len(), 0);

        let s = "\u{feff}\u{feff}a\u{feff}";

        // Huh, this difference might be a GNUStep bug?
        #[cfg(apple)]
        let expected = "\u{feff}a\u{feff}";
        #[cfg(gnustep)]
        let expected = "a\u{feff}";

        let ns_string = NSString::from_str(s);
        autoreleasepool(|pool| {
            assert_eq!(ns_string.as_str(pool), expected);
        });
        assert_eq!(ns_string.len(), expected.len());
    }
}
