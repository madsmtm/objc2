#![cfg(feature = "CFBase")]
use core::cmp::Ordering;
use core::ffi::{c_char, CStr};
use core::fmt::Write;
use core::ptr::NonNull;
use core::{fmt, str};

use crate::{
    kCFAllocatorNull, CFRange, CFRetained, CFString, CFStringBuiltInEncodings, CFStringCompare,
    CFStringCompareFlags, CFStringCreateWithBytes, CFStringCreateWithBytesNoCopy, CFStringGetBytes,
    CFStringGetCStringPtr, CFStringGetLength,
};

#[track_caller]
unsafe fn debug_checked_utf8_unchecked(bytes: &[u8]) -> &str {
    if cfg!(debug_assertions) {
        match str::from_utf8(bytes) {
            Ok(s) => s,
            Err(err) => panic!(
                "unsafe precondition violated: CF function did not return valid UTF-8: {err}"
            ),
        }
    } else {
        // SAFETY: Checked by caller
        unsafe { str::from_utf8_unchecked(bytes) }
    }
}

impl CFString {
    /// Creates a new `CFString` from a [`str`][prim@str].
    #[doc(alias = "CFStringCreateWithBytes")]
    #[allow(clippy::should_implement_trait)] // Not really sure of a better name
    pub fn from_str(string: &str) -> CFRetained<CFString> {
        let len = string.len().try_into().expect("string too large");
        let s = unsafe {
            CFStringCreateWithBytes(
                None,
                string.as_ptr(),
                len,
                CFStringBuiltInEncodings::EncodingUTF8.0,
                false,
            )
        };
        // Should only fail if the string is not UTF-u (which we know it is)
        // or perhaps on allocation error.
        s.expect("failed creating CFString")
    }

    /// Creates a new `CFString` from a `'static` [`str`][prim@str].
    ///
    /// This may be slightly more efficient than [`CFString::from_str`], as it
    /// may be able to re-use the existing buffer (since we know it won't be
    /// deallocated).
    #[doc(alias = "CFStringCreateWithBytesNoCopy")]
    pub fn from_static_str(string: &'static str) -> CFRetained<CFString> {
        let len = string.len().try_into().expect("string too large");
        // SAFETY: The string is used as a backing store, and thus must
        // potentially live forever, since we don't know how long the returned
        // CFString will be alive for. This is ensured by the `'static`
        // requirement.
        let s = unsafe {
            CFStringCreateWithBytesNoCopy(
                None,
                string.as_ptr(),
                len,
                CFStringBuiltInEncodings::EncodingUTF8.0,
                false,
                kCFAllocatorNull,
            )
        };
        s.expect("failed creating CFString")
    }

    /// Get the [`str`](`prim@str`) representation of this string if it can be
    /// done efficiently.
    ///
    /// Returns [`None`] if the internal storage does not allow this to be
    /// done efficiently. Use `CFString::to_string` if performance is not an
    /// issue.
    ///
    ///
    /// # Safety
    ///
    /// The `CFString` must not be mutated for the lifetime of the returned
    /// string.
    ///
    /// Warning: This is very difficult to ensure in generic contexts, e.g. it
    /// cannot even be used inside `Debug::fmt`, since `Formatter` uses `dyn`
    /// internally, and can thus mutate the string inside there.
    #[doc(alias = "CFStringGetCStringPtr")]
    // NOTE: This is NOT public, since it's completely broken for differently
    // encoded strings, see the `as_str_broken` test below.
    #[allow(dead_code)]
    unsafe fn as_str_unchecked(&self) -> Option<&str> {
        let bytes =
            unsafe { CFStringGetCStringPtr(self, CFStringBuiltInEncodings::EncodingUTF8.0) };
        NonNull::new(bytes as *mut c_char).map(|bytes| {
            // SAFETY: The pointer is valid for as long as the CFString is not
            // mutated (which the caller ensures it isn't for the lifetime of
            // the reference).
            //
            // We won't accidentally truncate the string here, since
            // `CFStringGetCStringPtr` makes sure that there are no internal
            // NUL bytes in the string.
            //
            // TODO: Verify this claim with a test.
            let cstr = unsafe { CStr::from_ptr(bytes.as_ptr()) };
            // SAFETY: `CFStringGetCStringPtr` is (very likely) implemented
            // correctly, and won't return non-UTF8 strings.
            unsafe { debug_checked_utf8_unchecked(cstr.to_bytes()) }
        })
    }
}

impl fmt::Display for CFString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Copy UTF-8 bytes from the CFString to the formatter in a loop, to
        // avoid allocating.
        //
        // We have to do this instead of using `CFStringGetCStringPtr`, as
        // that will be invalidated if the string is mutated while in use, and
        // `fmt::Formatter` contains `dyn Write` which may very theoretically
        // do exactly that.

        // Somewhat reasonably sized stack buffer.
        // TODO: Do performance testing, and tweak this value.
        //
        // Should be at least 4 (as that's the minimum size of `char`).
        let mut buf = [0u8; 32];

        let mut location_utf16 = 0;

        loop {
            let len_utf16 = unsafe { CFStringGetLength(self) };
            let mut read_utf8 = 0;
            let read_utf16 = unsafe {
                CFStringGetBytes(
                    self,
                    CFRange {
                        location: location_utf16,
                        length: len_utf16 - location_utf16,
                    },
                    CFStringBuiltInEncodings::EncodingUTF8.0,
                    0, // No conversion character
                    false,
                    buf.as_mut_ptr(),
                    buf.len() as _,
                    &mut read_utf8,
                )
            };
            if read_utf16 <= 0 {
                if location_utf16 < len_utf16 {
                    // We're not done reading the entire string yet; emit
                    // replacement character, advance one character, and try again.
                    f.write_char(char::REPLACEMENT_CHARACTER)?;
                    location_utf16 += 1;
                    continue;
                }
                break;
            }
            location_utf16 += read_utf16;

            // SAFETY: `CFStringGetBytes` is (very likely) implemented
            // correctly, and won't return non-UTF8 strings.
            //
            // Even if a string contains an UTF-8 char on a boundary, it won't
            // split it up when returning UTF-8.
            let s = unsafe { debug_checked_utf8_unchecked(&buf[0..read_utf8 as usize]) };

            // NOTE: May unwind, and may invalidate the string contents.
            f.write_str(s)?;
        }

        Ok(())
    }
}

impl PartialOrd for CFString {
    #[inline]
    #[doc(alias = "CFStringCompare")]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CFString {
    #[inline]
    #[doc(alias = "CFStringCompare")]
    fn cmp(&self, other: &Self) -> Ordering {
        // Request standard lexiographical ordering.
        let flags = CFStringCompareFlags::empty();
        unsafe { CFStringCompare(self, Some(other), flags) }.into()
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;
    use crate::{CFStringCreateWithCString, CFStringGetCString};

    #[test]
    fn basic_conversion() {
        let s = CFString::from_str("abc");
        assert_eq!(s.to_string(), "abc");
        let s = CFString::from_str("a♥😀");
        assert_eq!(s.to_string(), "a♥😀");
    }

    #[test]
    fn cstr_conversion() {
        let table = [
            (
                b"abc\xf8xyz\0" as &[u8],
                CFStringBuiltInEncodings::EncodingISOLatin1,
                "abcøxyz",
            ),
            (
                b"\x26\x65\0",
                CFStringBuiltInEncodings::EncodingUTF16BE,
                "♥",
            ),
            (
                b"\x65\x26\0",
                CFStringBuiltInEncodings::EncodingUTF16LE,
                "♥",
            ),
        ];
        for (cstr, encoding, expected) in table {
            let cstr = CStr::from_bytes_with_nul(cstr).unwrap();
            let s = unsafe { CFStringCreateWithCString(None, cstr.as_ptr(), encoding.0) }.unwrap();
            assert_eq!(s.to_string(), expected);
        }
    }

    #[test]
    fn from_incomplete() {
        let s = unsafe {
            CFStringCreateWithBytes(
                None,
                b"\xd8\x3d\xde".as_ptr(),
                3,
                CFStringBuiltInEncodings::EncodingUTF16BE.0,
                false,
            )
            .unwrap()
        };
        assert_eq!(s.to_string(), "�"); // Replacement character
        assert_eq!(unsafe { CFStringGetLength(&s) }, 1);
    }

    #[test]
    fn internal_nul_byte() {
        let s = CFString::from_str("a\0b\0c\0d");
        // Works with `CFStringGetBytes`.
        assert_eq!(s.to_string(), "a\0b\0c\0d");

        // Test `CFStringGetCString`.
        let mut buf = [0u8; 10];
        assert!(unsafe {
            CFStringGetCString(
                &s,
                buf.as_mut_ptr().cast(),
                buf.len() as _,
                CFStringBuiltInEncodings::EncodingUTF8.0,
            )
        });
        // All the data is copied to the buffer.
        assert_eq!(&buf[0..10], b"a\0b\0c\0d\0\0\0");

        // But subsequent usage of that as a CStr fails, since it contains
        // interior NUL bytes.
        let cstr = CStr::from_bytes_until_nul(&buf).unwrap();
        assert_eq!(cstr.to_bytes(), b"a");
    }

    #[test]
    fn utf8_on_boundary() {
        // Make the emoji lie across the 32 byte buffer size in Display::fmt.
        let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaa😀"; // 29 'a's
        assert_eq!(CFString::from_str(s).to_string(), s);
        let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa😀"; // 30 'a's
        assert_eq!(CFString::from_str(s).to_string(), s);
        let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa😀"; // 31 'a's
        assert_eq!(CFString::from_str(s).to_string(), s);
    }

    #[test]
    fn as_str_broken() {
        // A CFString that is supposed to contain a "♥" (the UTF-8 encoding of
        // that is the vastly different b"\xE2\x99\xA5").
        let s = unsafe {
            CFStringCreateWithCString(
                None,
                b"\x65\x26\0".as_ptr().cast(),
                CFStringBuiltInEncodings::EncodingUnicode.0,
            )
        }
        .unwrap();

        // `CFStringGetBytes` used in `fmt::Display` converts to UTF-8.
        assert_eq!(s.to_string(), "♥");

        // So does `CFStringGetCString`.
        let mut buf = [0u8; 20];
        assert!(unsafe {
            CFStringGetCString(
                &s,
                buf.as_mut_ptr().cast(),
                buf.len() as _,
                CFStringBuiltInEncodings::EncodingUTF8.0,
            )
        });
        let cstr = CStr::from_bytes_until_nul(&buf).unwrap();
        assert_eq!(cstr.to_bytes(), "♥".as_bytes());

        // But `CFStringGetCStringPtr` completely ignores the UTF-8 conversion
        // we asked it to do, i.e. a huge correctness footgun!
        assert_eq!(unsafe { s.as_str_unchecked() }, Some("e&"));
    }

    #[test]
    fn test_static() {
        let cf = CFString::from_static_str("xyz");
        assert_eq!(cf.to_string(), "xyz");
    }

    #[test]
    fn eq() {
        assert_eq!(CFString::from_str("abc"), CFString::from_str("abc"));
        assert_ne!(CFString::from_str("abc"), CFString::from_str("xyz"));
        // Cross-type comparison
        assert_ne!(
            **CFString::from_str("abc"),
            **unsafe { kCFAllocatorNull }.unwrap()
        );
    }

    // TODO: Test mutation while formatting.
}
