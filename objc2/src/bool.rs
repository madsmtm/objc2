use crate::{ffi, Encode, Encoding, RefEncode};
use core::fmt;

/// The Objective-C `BOOL` type.
///
/// This is a thin wrapper-type over [`objc_sys::BOOL`]. It is intended that
/// you convert this into a Rust [`bool`] with the [`Bool::as_bool`] method as
/// soon as possible.
///
/// This is FFI-safe and can be used in directly with
/// [`msg_send!`][`crate::msg_send`].
///
/// Note that this is able to contain more states than `bool` on some
/// platforms, but these cases should not be relied on!
///
/// # Example
///
/// ```no_run
/// use objc2::{class, msg_send_bool, msg_send_id};
/// use objc2::rc::{Id, Shared};
/// use objc2::runtime::{Object, Bool};
/// let ns_value: Id<Object, Shared> = unsafe {
///     msg_send_id![class!(NSNumber), numberWithBool: Bool::YES]
/// };
/// assert!(unsafe { msg_send_bool![&ns_value, boolValue] });
/// ```
#[repr(transparent)]
// We don't implement comparison traits because they could be implemented with
// two slightly different semantics:
// - `self.as_bool().cmp(other.as_bool())`
// - `self.value.cmp(other.value)`
// And it is not immediately clear for users which one was chosen.
#[derive(Copy, Clone, Default)]
pub struct Bool {
    value: ffi::BOOL,
}

impl Bool {
    /// The equivalent of [`true`] for Objective-C's `BOOL` type.
    pub const YES: Self = Self::from_raw(ffi::YES);

    /// The equivalent of [`false`] for Objective-C's `BOOL` type.
    pub const NO: Self = Self::from_raw(ffi::NO);

    /// Creates an Objective-C boolean from a Rust boolean.
    #[inline]
    pub const fn new(value: bool) -> Self {
        // true as u8 => 1
        // false as u8 => 0
        let value = value as ffi::BOOL;
        Self { value }
    }

    /// Creates this from a boolean value received from a raw Objective-C API.
    #[inline]
    pub const fn from_raw(value: ffi::BOOL) -> Self {
        Self { value }
    }

    /// Retrieves the inner [`objc_sys::BOOL`] boolean type, to be used in raw
    /// Objective-C APIs.
    #[inline]
    pub const fn as_raw(self) -> ffi::BOOL {
        self.value
    }

    /// Returns `true` if `self` is [`NO`][`Self::NO`].
    ///
    /// You should prefer using [`as_bool`][`Self::as_bool`].
    #[inline]
    pub const fn is_false(self) -> bool {
        // Always compare with 0
        // This is what happens with the `!` operator in C.
        self.value as u8 == 0
    }

    /// Returns `true` if `self` is the opposite of [`NO`][`Self::NO`].
    ///
    /// You should prefer using [`as_bool`][`Self::as_bool`].
    #[inline]
    pub const fn is_true(self) -> bool {
        // Always compare with 0
        // This is what happens when using `if` in C.
        self.value as u8 != 0
    }

    /// Converts this into the [`bool`] equivalent.
    #[inline]
    pub const fn as_bool(self) -> bool {
        self.is_true()
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
    // TODO: Fix this!
    const ENCODING: Encoding = ffi::BOOL::ENCODING;
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

    // Can't really do this test since it won't compile on platforms where
    // type BOOL = bool.
    //
    // #[test]
    // fn test_outside_normal() {
    //     let b = Bool::from_raw(42);
    //     assert!(b.is_true());
    //     assert!(!b.is_false());
    //     assert_eq!(b.as_raw(), 42);
    // }
}
