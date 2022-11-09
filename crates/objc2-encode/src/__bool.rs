//! This belongs in `objc2`, but it is put here to make `EncodeConvert` work
//! properly!
use core::fmt;

use crate::{Encode, EncodeConvert, Encoding, RefEncode};

/// The Objective-C `BOOL` type.
///
/// This is a thin wrapper-type over [`objc_sys::BOOL`]. It is intended that
/// you convert this into a Rust [`bool`] with the [`Bool::as_bool`] method as
/// soon as possible.
///
/// This is FFI-safe and can be used directly with `msg_send!` and `extern`
/// functions.
///
/// Note that this is able to contain more states than `bool` on some
/// platforms, but these cases should not be relied on!
#[repr(transparent)]
// We don't implement comparison traits because they could be implemented with
// two slightly different semantics:
// - `self.as_bool().cmp(other.as_bool())`
// - `self.value.cmp(other.value)`
// And it is not immediately clear for users which one was chosen.
#[derive(Copy, Clone, Default)]
pub struct Bool {
    value: objc_sys::BOOL,
}

impl Bool {
    /// The equivalent of [`true`] for Objective-C's `BOOL` type.
    pub const YES: Self = Self::from_raw(objc_sys::YES);

    /// The equivalent of [`false`] for Objective-C's `BOOL` type.
    pub const NO: Self = Self::from_raw(objc_sys::NO);

    /// Creates an Objective-C boolean from a Rust boolean.
    #[inline]
    pub const fn new(value: bool) -> Self {
        // true as BOOL => 1 (YES)
        // false as BOOL => 0 (NO)
        let value = value as objc_sys::BOOL;
        Self { value }
    }

    /// Creates this from a boolean value received from a raw Objective-C API.
    #[inline]
    pub const fn from_raw(value: objc_sys::BOOL) -> Self {
        Self { value }
    }

    /// Retrieves the inner [`objc_sys::BOOL`] boolean type, to be used in raw
    /// Objective-C APIs.
    #[inline]
    pub const fn as_raw(self) -> objc_sys::BOOL {
        self.value
    }

    /// Returns `true` if `self` is [`NO`][Self::NO].
    ///
    /// You should prefer using [`as_bool`][Self::as_bool].
    #[inline]
    pub const fn is_false(self) -> bool {
        !self.as_bool()
    }

    /// Returns `true` if `self` is not [`NO`][Self::NO].
    ///
    /// You should prefer using [`as_bool`][Self::as_bool].
    #[inline]
    pub const fn is_true(self) -> bool {
        self.as_bool()
    }

    /// Converts this into the [`bool`] equivalent.
    #[inline]
    pub const fn as_bool(self) -> bool {
        // Always compare with 0 (NO)
        // This is what happens with the `!` operator / when using `if` in C.
        self.value != objc_sys::NO
    }
}

impl From<bool> for Bool {
    #[inline]
    fn from(b: bool) -> Bool {
        Bool::new(b)
    }
}

impl From<Bool> for bool {
    #[inline]
    fn from(b: Bool) -> bool {
        b.as_bool()
    }
}

impl fmt::Debug for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(if self.as_bool() { "YES" } else { "NO" })
    }
}

// SAFETY: `Bool` is `repr(transparent)`.
unsafe impl Encode for Bool {
    // i8::__ENCODING == Encoding::Char
    // u8::__ENCODING == Encoding::UChar
    // bool::__ENCODING == Encoding::Bool
    // i32::__ENCODING == Encoding::Int
    const ENCODING: Encoding = objc_sys::BOOL::__ENCODING;
}

// Note that we shouldn't delegate to `BOOL`'s  `ENCODING_REF` since `BOOL` is
// sometimes `i8`/`u8`, and their `ENCODING_REF`s are `Encoding::String`,
// which is incorrect for `BOOL`:
//
// ```objc
// @encode(BOOL); // -> "c", "C" or "B"
// @encode(BOOL*); // -> "^c", "^C" or "^B"
// @encode(char); // -> "c" or "C"
// @encode(char*); // -> "*"
// ```
unsafe impl RefEncode for Bool {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;

    #[test]
    fn test_basic() {
        let b = Bool::new(true);
        assert!(b.as_bool());
        assert!(b.is_true());
        assert!(!b.is_false());
        assert!(bool::from(b));
        assert_eq!(b.as_raw() as usize, 1);

        let b = Bool::new(false);
        assert!(!b.as_bool());
        assert!(!b.is_true());
        assert!(b.is_false());
        assert!(!bool::from(b));
        assert_eq!(b.as_raw() as usize, 0);
    }

    #[test]
    fn test_associated_constants() {
        let b = Bool::YES;
        assert!(b.as_bool());
        assert!(b.is_true());
        assert_eq!(b.as_raw() as usize, 1);

        let b = Bool::NO;
        assert!(!b.as_bool());
        assert!(b.is_false());
        assert_eq!(b.as_raw() as usize, 0);
    }

    #[test]
    fn test_impls() {
        let b: Bool = Default::default();
        assert!(!b.as_bool());
        assert!(b.is_false());

        assert!(Bool::from(true).as_bool());
        assert!(Bool::from(true).is_true());
        assert!(Bool::from(false).is_false());

        assert!(Bool::from(true).is_true());
        assert!(Bool::from(false).is_false());
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Bool::from(true)), "YES");
        assert_eq!(format!("{:?}", Bool::from(false)), "NO");
    }

    #[test]
    // Test on platform where we know the type of BOOL
    #[cfg(all(feature = "apple", target_os = "macos", target_arch = "x86_64"))]
    fn test_outside_normal() {
        let b = Bool::from_raw(42);
        assert!(b.is_true());
        assert!(!b.is_false());
        assert_eq!(b.as_raw(), 42);
    }
}
