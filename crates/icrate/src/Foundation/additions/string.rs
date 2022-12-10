use alloc::borrow::ToOwned;
use core::cmp;
use core::ffi::c_void;
use core::fmt;
use core::panic::RefUnwindSafe;
use core::panic::UnwindSafe;
#[cfg(feature = "apple")]
use core::ptr::NonNull;
#[cfg(feature = "apple")]
use core::slice;
use core::str;
#[cfg(feature = "apple")]
use std::os::raw::c_char;

use objc2::rc::{autoreleasepool, AutoreleasePool, DefaultId, Id, Shared};
use objc2::runtime::__nsstring::{nsstring_len, nsstring_to_str, UTF8_ENCODING};
use objc2::runtime::{Class, Object};
use objc2::{msg_send, ClassType};

use crate::Foundation::{NSCopying, NSMutableCopying, NSMutableString, NSString};

// SAFETY: `NSString` is immutable and `NSMutableString` can only be mutated
// from `&mut` methods.
unsafe impl Sync for NSString {}
unsafe impl Send for NSString {}

// Even if an exception occurs inside a string method, the state of the string
// (should) still be perfectly safe to access.
impl UnwindSafe for NSString {}
impl RefUnwindSafe for NSString {}

impl NSString {
    /// Construct an empty NSString.
    pub fn new() -> Id<Self, Shared> {
        Self::init(Self::alloc())
    }

    /// The number of UTF-8 code units in `self`.
    #[doc(alias = "lengthOfBytesUsingEncoding")]
    #[doc(alias = "lengthOfBytesUsingEncoding:")]
    pub fn len(&self) -> usize {
        // SAFETY: This is an instance of `NSString`
        unsafe { nsstring_len(self) }
    }

    /// The number of UTF-16 code units in the string.
    ///
    /// See also [`NSString::len`].
    #[doc(alias = "length")]
    pub fn len_utf16(&self) -> usize {
        self.length()
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
    #[cfg(feature = "apple")]
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
    #[cfg(feature = "apple")]
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
        // SAFETY: This is an instance of `NSString`
        unsafe { nsstring_to_str(self, pool) }
    }

    // TODO: Allow usecases where the NUL byte from `UTF8String` is kept?

    /// Creates an immutable `NSString` by copying the given string slice.
    ///
    /// Prefer using the [`ns_string!`] macro when possible.
    ///
    /// [`ns_string!`]: crate::ns_string
    #[doc(alias = "initWithBytes")]
    #[doc(alias = "initWithBytes:length:encoding:")]
    #[allow(clippy::should_implement_trait)] // Not really sure of a better name
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
}

pub(crate) fn from_str(cls: &Class, string: &str) -> *mut Object {
    let bytes: *const c_void = string.as_ptr().cast();
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
        self.compare(other).into()
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

impl ToOwned for NSString {
    type Owned = Id<NSString, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
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

impl fmt::Debug for NSString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The call to `to_owned` is unfortunate, but is required to work
        // around `f` not being AutoreleaseSafe.
        // TODO: Fix this!
        let s = autoreleasepool(|pool| self.as_str(pool).to_owned());
        fmt::Debug::fmt(&s, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;
    use core::ptr;

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
    fn display_debug() {
        let s = NSString::from_str("xyz\"123");
        assert_eq!(format!("{s}"), "xyz\"123");
        assert_eq!(format!("{s:?}"), r#""xyz\"123""#);
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
        assert_eq!(Id::as_ptr(&s1), Id::as_ptr(&s2));
        assert!(s2.is_kind_of::<NSString>());

        let s3 = s1.mutable_copy();
        assert_ne!(Id::as_ptr(&s1), Id::as_ptr(&s3).cast());
        assert!(s3.is_kind_of::<NSMutableString>());
    }

    #[test]
    fn test_copy_nsstring_is_same() {
        let string1 = NSString::from_str("Hello, world!");
        let string2 = string1.copy();
        assert!(
            ptr::eq(&*string1, &*string2),
            "Cloned NSString didn't have the same address"
        );
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
        #[cfg(feature = "apple")]
        let expected = "\u{feff}a\u{feff}";
        #[cfg(feature = "gnustep-1-7")]
        let expected = "a\u{feff}";

        let ns_string = NSString::from_str(s);
        autoreleasepool(|pool| {
            assert_eq!(ns_string.as_str(pool), expected);
        });
        assert_eq!(ns_string.len(), expected.len());
    }

    #[test]
    fn test_hash() {
        use core::hash::Hasher;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;

        let s1 = NSString::from_str("example string goes here");
        let s2 = NSString::from_str("example string goes here");

        let mut hashstate = DefaultHasher::new();
        let mut hashstate2 = DefaultHasher::new();

        s1.hash(&mut hashstate);
        s2.hash(&mut hashstate2);

        assert_eq!(hashstate.finish(), hashstate2.finish());
    }

    #[test]
    fn test_prefix_suffix() {
        let s = NSString::from_str("abcdef");
        let prefix = NSString::from_str("abc");
        let suffix = NSString::from_str("def");
        assert!(s.hasPrefix(&prefix));
        assert!(s.hasSuffix(&suffix));
        assert!(!s.hasPrefix(&suffix));
        assert!(!s.hasSuffix(&prefix));
    }

    #[test]
    #[allow(clippy::nonminimal_bool)]
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

    #[test]
    fn test_append() {
        let error_tag = NSString::from_str("Error: ");
        let error_string = NSString::from_str("premature end of file.");
        let error_message = error_tag.stringByAppendingString(&error_string);
        assert_eq!(
            error_message,
            NSString::from_str("Error: premature end of file.")
        );

        let extension = NSString::from_str("scratch.tiff");
        assert_eq!(
            NSString::from_str("/tmp").stringByAppendingPathComponent(&extension),
            NSString::from_str("/tmp/scratch.tiff")
        );
        assert_eq!(
            NSString::from_str("/tmp/").stringByAppendingPathComponent(&extension),
            NSString::from_str("/tmp/scratch.tiff")
        );
        assert_eq!(
            NSString::from_str("/").stringByAppendingPathComponent(&extension),
            NSString::from_str("/scratch.tiff")
        );
        assert_eq!(
            NSString::from_str("").stringByAppendingPathComponent(&extension),
            NSString::from_str("scratch.tiff")
        );
    }
}
