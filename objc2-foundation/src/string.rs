use core::cmp;
use core::ffi::c_void;
use core::fmt;
use core::ptr::NonNull;
use core::slice;
use core::str;
use std::os::raw::c_char;

use alloc::borrow::ToOwned;
use objc2::ffi;
use objc2::msg_send;
use objc2::rc::DefaultId;
use objc2::rc::{autoreleasepool, AutoreleasePool};
use objc2::rc::{Id, Shared};
use objc2::runtime::{Bool, Class, Object};

use crate::{NSComparisonResult, NSCopying, NSMutableCopying, NSMutableString, NSObject};

#[cfg(apple)]
const UTF8_ENCODING: usize = 4;
#[cfg(gnustep)]
const UTF8_ENCODING: i32 = 4;

#[allow(unused)]
#[allow(non_upper_case_globals)]
const NSNotFound: ffi::NSInteger = ffi::NSIntegerMax;

object! {
    /// A static, plain-text Unicode string object.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsstring?language=objc).
    unsafe pub struct NSString: NSObject;
    // TODO: Use isEqualToString: for comparison (instead of just isEqual:)
    // The former is more performant

    // TODO: Use more performant Debug implementation.

    // TODO: Check if performance of NSSelectorFromString is worthwhile
}

// TODO: SAFETY
unsafe impl Sync for NSString {}
unsafe impl Send for NSString {}

impl NSString {
    unsafe_def_fn! {
        /// Construct an empty NSString.
        pub fn new -> Shared;
    }

    /// The number of UTF-8 code units in `self`.
    #[doc(alias = "lengthOfBytesUsingEncoding")]
    #[doc(alias = "lengthOfBytesUsingEncoding:")]
    pub fn len(&self) -> usize {
        unsafe { msg_send![self, lengthOfBytesUsingEncoding: UTF8_ENCODING] }
    }

    /// The number of UTF-16 code units in `self`.
    ///
    /// See also [`NSString::len`].
    #[doc(alias = "length")]
    // TODO: Finish this
    fn len_utf16(&self) -> usize {
        unsafe { msg_send![self, length] }
    }

    pub fn is_empty(&self) -> bool {
        // TODO: lengthOfBytesUsingEncoding: might sometimes return 0 for
        // other reasons, so this is not really correct!
        self.len() == 0
    }

    /// Get the [`str`](`prim@str`) representation of this string if it can be
    /// done efficiently.
    ///
    /// Returns [`None`] if the internal storage does not allow this to be
    /// done efficiently. Use [`NSString::as_str`] or `NSString::to_string`
    /// if performance is not an issue.
    #[doc(alias = "CFStringGetCStringPtr")]
    #[allow(unused)]
    // TODO: Finish this
    fn as_str_wip(&self) -> Option<&str> {
        type CFStringEncoding = u32;
        #[allow(non_upper_case_globals)]
        // https://developer.apple.com/documentation/corefoundation/cfstringbuiltinencodings/kcfstringencodingutf8?language=objc
        const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;
        extern "C" {
            // https://developer.apple.com/documentation/corefoundation/1542133-cfstringgetcstringptr?language=objc
            fn CFStringGetCStringPtr(s: &NSString, encoding: CFStringEncoding) -> *const c_char;
        }
        let bytes = unsafe { CFStringGetCStringPtr(self, kCFStringEncodingUTF8) };
        NonNull::new(bytes as *mut u8).map(|bytes| {
            let len = self.len();
            let bytes: &[u8] = unsafe { slice::from_raw_parts(bytes.as_ptr(), len) };
            str::from_utf8(bytes).unwrap()
        })
    }

    /// Get an [UTF-16] string slice if it can be done efficiently.
    ///
    /// Returns [`None`] if the internal storage of `self` does not allow this
    /// to be returned efficiently.
    ///
    /// See [`as_str`](Self::as_str) for the UTF-8 equivalent.
    ///
    /// [UTF-16]: https://en.wikipedia.org/wiki/UTF-16
    #[allow(unused)]
    // TODO: Finish this
    fn as_utf16(&self) -> Option<&[u16]> {
        extern "C" {
            // https://developer.apple.com/documentation/corefoundation/1542939-cfstringgetcharactersptr?language=objc
            fn CFStringGetCharactersPtr(s: &NSString) -> *const u16;
        }
        let ptr = unsafe { CFStringGetCharactersPtr(self) };
        NonNull::new(ptr as *mut u16)
            .map(|ptr| unsafe { slice::from_raw_parts(ptr.as_ptr(), self.len_utf16()) })
    }

    /// Get the [`str`](`prim@str`) representation of this.
    ///
    /// TODO: Further explain this.
    #[doc(alias = "UTF8String")]
    pub fn as_str<'r, 's: 'r, 'p: 'r>(&'s self, pool: &'p AutoreleasePool) -> &'r str {
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
        //
        // https://developer.apple.com/documentation/foundation/nsstring/1411189-utf8string?language=objc
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

    // TODO: Allow usecases where the NUL byte from `UTF8String` is kept?

    /// Creates an immutable `NSString` by copying the given string slice.
    #[doc(alias = "initWithBytes")]
    #[doc(alias = "initWithBytes:length:encoding:")]
    pub fn from_str(string: &str) -> Id<Self, Shared> {
        unsafe {
            let obj = from_str(Self::class(), string);
            Id::new(obj.cast()).unwrap()
        }
    }

    // TODO: initWithBytesNoCopy:, maybe add lifetime parameter to NSString?
    // See https://github.com/nvzqz/fruity/blob/320efcf715c2c5fbd2f3084f671f2be2e03a6f2b/src/foundation/ns_string/mod.rs#L350-L381
    // Might be quite difficult, as Objective-C code might assume the NSString
    // is always alive?
    // See https://github.com/drewcrawford/foundationr/blob/b27683417a35510e8e5d78a821f081905b803de6/src/nsstring.rs

    /// Whether the given string matches the beginning characters of this
    /// string.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsstring/1410309-hasprefix?language=objc).
    #[doc(alias = "hasPrefix")]
    #[doc(alias = "hasPrefix:")]
    pub fn has_prefix(&self, prefix: &NSString) -> bool {
        let res: Bool = unsafe { msg_send![self, hasPrefix: prefix] };
        res.is_true()
    }

    /// Whether the given string matches the ending characters of this string.
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/foundation/nsstring/1416529-hassuffix?language=objc).
    #[doc(alias = "hasSuffix")]
    #[doc(alias = "hasSuffix:")]
    pub fn has_suffix(&self, suffix: &NSString) -> bool {
        let res: Bool = unsafe { msg_send![self, hasSuffix: suffix] };
        res.is_true()
    }

    // pub fn from_nsrange(range: NSRange) -> Id<Self, Shared>
    // https://developer.apple.com/documentation/foundation/1415155-nsstringfromrange?language=objc
}

pub(crate) fn from_str(cls: &Class, string: &str) -> *mut Object {
    let bytes = string.as_ptr() as *const c_void;
    unsafe {
        let obj: *mut Object = msg_send![cls, alloc];
        msg_send![
            obj,
            initWithBytes: bytes,
            length: string.len(),
            encoding: UTF8_ENCODING,
        ]
    }
}

impl PartialOrd for NSString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NSString {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let res: NSComparisonResult = unsafe { msg_send![self, compare: other] };
        // TODO: Other comparison methods:
        // - compare:options:
        // - compare:options:range:
        // - compare:options:range:locale:
        // - localizedCompare:
        // - caseInsensitiveCompare:
        // - localizedCaseInsensitiveCompare:
        // - localizedStandardCompare:
        res.into()
    }
}

// TODO: PartialEq and PartialOrd against &str
// See `fruity`'s implementation:
// https://github.com/nvzqz/fruity/blob/320efcf715c2c5fbd2f3084f671f2be2e03a6f2b/src/foundation/ns_string/mod.rs#L69-L163

impl DefaultId for NSString {
    type Ownership = Shared;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new()
    }
}

unsafe impl NSCopying for NSString {
    type Ownership = Shared;
    type Output = NSString;
}

unsafe impl NSMutableCopying for NSString {
    type Output = NSMutableString;
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
    fn test_nul() {
        let expected = "\0";
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
        let s1 = NSString::from_str("abc");
        let s2 = s1.copy();
        // An optimization that NSString makes, since it is immutable
        assert_eq!(s1.as_ptr(), s2.as_ptr());
        assert!(s2.is_kind_of(NSString::class()));

        let s3 = s1.mutable_copy();
        assert_ne!(s1.as_ptr(), s3.as_ptr() as *mut NSString);
        assert!(s3.is_kind_of(NSMutableString::class()));
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

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;

        let s1 = NSString::from_str("example string goes here");
        let s2 = NSString::from_str("example string goes here");

        let mut hashstate = DefaultHasher::new();
        let mut hashstate2 = DefaultHasher::new();
        assert_eq!(s1.hash(&mut hashstate), s2.hash(&mut hashstate2));
    }

    #[test]
    fn test_prefix_suffix() {
        let s = NSString::from_str("abcdef");
        let prefix = NSString::from_str("abc");
        let suffix = NSString::from_str("def");
        assert!(s.has_prefix(&prefix));
        assert!(s.has_suffix(&suffix));
        assert!(!s.has_prefix(&suffix));
        assert!(!s.has_suffix(&prefix));
    }

    #[test]
    fn test_cmp() {
        let s1 = NSString::from_str("aa");
        assert!(s1 <= s1);
        assert!(s1 >= s1);
        let s2 = NSString::from_str("ab");
        assert!(s1 < s2);
        assert!(!(s1 > s2));
        assert!(s1 <= s2);
        assert!(!(s1 >= s2));
        let s3 = NSString::from_str("ba");
        assert!(s1 < s3);
        assert!(!(s1 > s3));
        assert!(s1 <= s3);
        assert!(!(s1 >= s3));
        assert!(s2 < s3);
        assert!(!(s2 > s3));
        assert!(s2 <= s3);
        assert!(!(s2 >= s3));

        let s = NSString::from_str("abc");
        let shorter = NSString::from_str("a");
        let longer = NSString::from_str("abcdef");
        assert!(s > shorter);
        assert!(s < longer);
    }
}
