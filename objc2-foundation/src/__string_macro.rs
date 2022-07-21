//! Macro for making a static NSString.
//!
//! This basically does what clang does, see:
//! - Apple: <https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CodeGenModule.cpp#L5057-L5249>
//! - GNUStep 2.0 (not yet supported): <https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CGObjCGNU.cpp#L973-L1118>
//! - Other (not yet supported): <https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CGObjCGNU.cpp#L2471-L2507>
//!
//! Note that this uses the `CFString` static, while `clang` has support for
//! generating a pure `NSString`. We don't support that yet (since I don't
//! know the use-case), but we definitely could!
//! See: <https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CGObjCMac.cpp#L2007-L2068>
#![cfg(feature = "apple")]
use core::ffi::c_void;

use objc2::runtime::Class;

use crate::NSString;

// This is defined in CoreFoundation, but we don't emit a link attribute
// here because it is already linked via Foundation.
//
// Although this is a "private" (underscored) symbol, it is directly
// referenced in Objective-C binaries. So it's safe for us to reference.
extern "C" {
    pub static __CFConstantStringClassReference: Class;
}

// From `CFString.c`:
// https://github.com/apple-oss-distributions/CF/blob/CF-1153.18/CFString.c#L184-L212
// > !!! Note: Constant CFStrings use the bit patterns:
// > C8 (11001000 = default allocator, not inline, not freed contents; 8-bit; has NULL byte; doesn't have length; is immutable)
// > D0 (11010000 = default allocator, not inline, not freed contents; Unicode; is immutable)
// > The bit usages should not be modified in a way that would effect these bit patterns.
//
// The 7 byte is the `CFTypeID` of `CFStringRef`.
const FLAGS_ASCII: usize = 0x07_C8;
const FLAGS_UTF16: usize = 0x07_D0;

#[repr(C)]
pub struct CFConstString {
    isa: &'static Class,
    flags: usize,
    data: *const c_void,
    len: usize,
}

// Required to place in a `static`.
unsafe impl Sync for CFConstString {}

impl CFConstString {
    pub const unsafe fn new_ascii(isa: &'static Class, data: &'static [u8]) -> Self {
        Self {
            isa,
            flags: FLAGS_ASCII,
            data: data.as_ptr().cast(),
            // The length does not include the trailing NUL.
            len: data.len() - 1,
        }
    }

    pub const unsafe fn new_utf16(isa: &'static Class, data: &'static [u16]) -> Self {
        Self {
            isa,
            flags: FLAGS_UTF16,
            data: data.as_ptr().cast(),
            // The length does not include the trailing NUL.
            len: data.len() - 1,
        }
    }

    #[inline]
    pub const fn as_nsstring(&self) -> &NSString {
        unsafe { &*(self as *const Self as *const NSString) }
    }
}

/// Returns `true` if `bytes` is entirely ASCII with no interior NULs.
pub const fn is_ascii_no_nul(bytes: &[u8]) -> bool {
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        if !byte.is_ascii() || byte == b'\0' {
            return false;
        }
        i += 1;
    }
    true
}

pub struct Utf16Char {
    pub repr: [u16; 2],
    pub len: usize,
}

impl Utf16Char {
    const fn encode(ch: u32) -> Self {
        if ch <= 0xffff {
            Self {
                repr: [ch as u16, 0],
                len: 1,
            }
        } else {
            let payload = ch - 0x10000;
            let hi = (payload >> 10) | 0xd800;
            let lo = (payload & 0x3ff) | 0xdc00;
            Self {
                repr: [hi as u16, lo as u16],
                len: 2,
            }
        }
    }

    #[cfg(test)]
    fn as_slice(&self) -> &[u16] {
        &self.repr[..self.len]
    }
}

pub struct EncodeUtf16Iter {
    str: &'static [u8],
    index: usize,
}

impl EncodeUtf16Iter {
    pub const fn new(str: &'static [u8]) -> Self {
        Self { str, index: 0 }
    }

    pub const fn next(self) -> Option<(Self, Utf16Char)> {
        if self.index >= self.str.len() {
            None
        } else {
            let (index, ch) = decode_utf8(self.str, self.index);
            Some((Self { index, ..self }, Utf16Char::encode(ch)))
        }
    }
}

// (&str bytes, index) -> (new index, decoded char)
const fn decode_utf8(s: &[u8], i: usize) -> (usize, u32) {
    let b0 = s[i];
    match b0 {
        // one-byte seq
        0b0000_0000..=0b0111_1111 => {
            let decoded = b0 as u32;
            (i + 1, decoded)
        }
        // two-byte seq
        0b1100_0000..=0b1101_1111 => {
            let decoded = ((b0 as u32 & 0x1f) << 6) | (s[i + 1] as u32 & 0x3f);
            (i + 2, decoded)
        }
        // 3 byte seq
        0b1110_0000..=0b1110_1111 => {
            let decoded = ((b0 as u32 & 0x0f) << 12)
                | ((s[i + 1] as u32 & 0x3f) << 6)
                | (s[i + 2] as u32 & 0x3f);
            (i + 3, decoded)
        }
        // 4 byte seq
        0b1111_0000..=0b1111_0111 => {
            let decoded = ((b0 as u32 & 0x07) << 18)
                | ((s[i + 1] as u32 & 0x3f) << 12)
                | ((s[i + 2] as u32 & 0x3f) << 6)
                | (s[i + 3] as u32 & 0x3f);
            (i + 4, decoded)
        }
        // continuation bytes, or never-valid bytes.
        0b1000_0000..=0b1011_1111 | 0b1111_1000..=0b1111_1111 => {
            panic!("Encountered invalid bytes")
        }
    }
}

/// Creates an [`NSString`][`crate::NSString`] from a static string.
///
/// Currently only supported on Apple targets.
///
///
/// # Examples
///
/// This macro takes a either a `"string"` literal or `const` string slice as
/// the argument:
///
/// ```
/// use objc2_foundation::ns_string;
/// let hello = ns_string!("hello");
/// assert_eq!(hello.to_string(), "hello");
///
/// const WORLD: &str = "world";
/// let world = ns_string!(WORLD);
/// assert_eq!(world.to_string(), WORLD);
/// ```
///
/// The result of this macro can even be used to create `static` values:
///
/// ```
/// # use objc2_foundation::{ns_string, NSString};
/// static WORLD: &NSString = ns_string!("world");
///
/// assert_eq!(WORLD.to_string(), "world");
/// ```
///
/// Note that the result cannot be used in a `const` because it refers to
/// static data outside of this library.
///
///
/// # Unicode Strings
///
/// An NSString can contain strings with many different encodings, including
/// ASCII, UTF-8, UTF-16, and so on. This macro automatically converts your
/// string to the most efficient encoding, you don't have to do anything!
///
/// ```
/// # use objc2_foundation::{ns_string, NSString};
/// static HELLO_RU: &NSString = ns_string!("Привет");
/// assert_eq!(HELLO_RU.to_string(), "Привет");
/// ```
///
/// Note that because this is implemented with `const` evaluation, massive
/// strings can increase compile time, and may even hit the `const` evaluation
/// limit.
///
///
/// # NUL handling
///
/// Strings containing ASCII NUL is allowed, the NUL is preserved as one would
/// expect:
///
/// ```
/// # use objc2_foundation::ns_string;
/// let example = ns_string!("example\0");
/// assert_eq!(example.to_string(), "example\0");
///
/// let example = ns_string!("exa\0mple");
/// assert_eq!(example.to_string(), "exa\0mple");
/// ```
///
///
/// # Runtime Cost
///
/// None.
///
/// The result is equivalent to `@"string"` syntax in Objective-C.
///
/// Because of that, this should be preferred over [`NSString::from_str`]
/// where possible.
///
/// [`NSString::from_str`]: crate::NSString::from_str
#[macro_export]
#[cfg(feature = "apple")] // To make `auto_doc_cfg` pick this up
macro_rules! ns_string {
    ($s:expr) => {{
        // Note: We create both the ASCII + NUL and the UTF-16 + NUL versions
        // of the string, since we can't conditionally create a static.
        //
        // Since we don't add the `#[used]` attribute, Rust can fairly easily
        // figure out that one of the variants are never used, and simply
        // exclude it.

        const INPUT: &[u8] = $s.as_bytes();

        // Convert the input slice to a C-style string with a NUL byte.
        //
        // The section is the same as what clang sets, see:
        // https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CodeGenModule.cpp#L5192
        #[link_section = "__TEXT,__cstring,cstring_literals"]
        static ASCII: [u8; INPUT.len() + 1] = {
            // Zero-fill with INPUT.len() + 1
            let mut res: [u8; INPUT.len() + 1] = [0; INPUT.len() + 1];
            let mut i = 0;
            // Fill with data from INPUT
            while i < INPUT.len() {
                res[i] = INPUT[i];
                i += 1;
            }
            // Now contains INPUT + '\0'
            res
        };

        // The full UTF-16 contents along with the written length.
        const UTF16_FULL: (&[u16; INPUT.len()], usize) = {
            let mut out = [0u16; INPUT.len()];
            let mut iter = $crate::__string_macro::EncodeUtf16Iter::new(INPUT);
            let mut written = 0;

            while let Some((state, chars)) = iter.next() {
                iter = state;
                out[written] = chars.repr[0];
                written += 1;

                if chars.len > 1 {
                    out[written] = chars.repr[1];
                    written += 1;
                }
            }

            (&{ out }, written)
        };

        // Convert the slice to an UTF-16 array + a final NUL byte.
        //
        // The section is the same as what clang sets, see:
        // https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CodeGenModule.cpp#L5193
        #[link_section = "__TEXT,__ustring"]
        static UTF16: [u16; UTF16_FULL.1 + 1] = {
            // Zero-fill with UTF16_FULL.1 + 1
            let mut res: [u16; UTF16_FULL.1 + 1] = [0; UTF16_FULL.1 + 1];
            let mut i = 0;
            // Fill with data from UTF16_FULL.0 up until UTF16_FULL.1
            while i < UTF16_FULL.1 {
                res[i] = UTF16_FULL.0[i];
                i += 1;
            }
            // Now contains UTF16_FULL.1 + NUL
            res
        };

        // Create the constant string structure, and store it in a static
        // within a special section.
        //
        // The section is the same as what clang sets, see:
        // https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CodeGenModule.cpp#L5243
        #[link_section = "__DATA,__cfstring"]
        static CFSTRING: $crate::__string_macro::CFConstString = unsafe {
            if $crate::__string_macro::is_ascii_no_nul(INPUT) {
                // This is technically an optimization (UTF-16 strings are
                // always valid), but it's a fairly important one!
                $crate::__string_macro::CFConstString::new_ascii(
                    &$crate::__string_macro::__CFConstantStringClassReference,
                    &ASCII,
                )
            } else {
                $crate::__string_macro::CFConstString::new_utf16(
                    &$crate::__string_macro::__CFConstantStringClassReference,
                    &UTF16,
                )
            }
        };

        // Return &'static NSString
        CFSTRING.as_nsstring()
    }};
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_is_ascii() {
        assert!(is_ascii_no_nul(b"a"));
        assert!(is_ascii_no_nul(b"abc"));

        assert!(!is_ascii_no_nul(b"\xff"));

        assert!(!is_ascii_no_nul(b"\0"));
        assert!(!is_ascii_no_nul(b"a\0b"));
        assert!(!is_ascii_no_nul(b"ab\0"));
        assert!(!is_ascii_no_nul(b"a\0b\0"));
    }

    #[test]
    fn test_decode_utf8() {
        for c in '\u{0}'..=core::char::MAX {
            let mut buf;
            for off in 0..4 {
                // Ensure we see garbage if we read outside bounds.
                buf = [0xff; 8];
                let len = c.encode_utf8(&mut buf[off..(off + 4)]).len();
                let (end_idx, decoded) = decode_utf8(&buf, off);
                assert_eq!(
                    (end_idx, decoded),
                    (off + len, c as u32),
                    "failed for U+{code:04X} ({ch:?}) encoded as {buf:#x?} over {range:?}",
                    code = c as u32,
                    ch = c,
                    buf = &buf[off..(off + len)],
                    range = off..(off + len),
                );
            }
        }
    }

    #[test]
    fn encode_utf16() {
        for c in '\u{0}'..=core::char::MAX {
            assert_eq!(
                c.encode_utf16(&mut [0u16; 2]),
                Utf16Char::encode(c as u32).as_slice(),
                "failed for U+{:04X} ({:?})",
                c as u32,
                c
            );
        }
    }

    #[test]
    fn ns_string() {
        macro_rules! test {
            ($($s:expr,)+) => {$({
                static STRING: &NSString = ns_string!($s);
                let s = NSString::from_str($s);

                assert_eq!(STRING, STRING);
                assert_eq!(STRING, &*s);

                assert_eq!(STRING.to_string(), $s);
                assert_eq!(s.to_string(), $s);
            })+};
        }

        test! {
            "",
            "asdf",
            "🦀",
            "🏳️‍🌈",
            "𝄞music",
            "abcd【e】fg",
            "abcd⒠fg",
            "ääääh",
            "lööps, bröther?",
            "\u{fffd} \u{fffd} \u{fffd}",
            "讓每個人都能打造出。",
            "\0",
            "\0\x01\x02\x03\x04\x05\x06\x07\x08\x09",
            // "\u{feff}", // TODO
            include_str!("__string_macro.rs"),
        }
    }

    #[test]
    fn ns_string_in_unsafe() {
        // Test that the `unused_unsafe` lint doesn't trigger
        let s = unsafe {
            let s: *const NSString = ns_string!("abc");
            &*s
        };
        assert_eq!(s.to_string(), "abc");
    }
}
