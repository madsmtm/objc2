use crate::{Encode, Encoding, RefEncode};
use core::fmt;

/// The Objective-C `BOOL` type.
///
/// To convert an Objective-C `BOOL` into a Rust [`bool`], call the one of the
/// [`Bool::is_false`] or [`Bool::is_true`] methods.
///
/// This is FFI-safe and can be used in directly with
/// [`msg_send!`][`crate::msg_send`].
#[repr(transparent)]
// TODO: Might have to implement some of these manually, in case someone puts
// something that is not 0 or 1 into the Bool?
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Bool {
    value: objc2_sys::BOOL,
}

impl Bool {
    /// The equivalent of [`true`] for Objective-C's `BOOL` type.
    pub const YES: Self = Self::from_raw(objc2_sys::YES);

    /// The equivalent of [`false`] for Objective-C's `BOOL` type.
    pub const NO: Self = Self::from_raw(objc2_sys::NO);

    /// Creates an Objective-C boolean from a Rust boolean.
    #[inline]
    pub const fn new(value: bool) -> Self {
        // true as u8 => 1
        // false as u8 => 0
        let value = value as objc2_sys::BOOL;
        Self { value }
    }

    /// Creates this from a boolean value received from a raw Objective-C API.
    #[inline]
    pub const fn from_raw(value: objc2_sys::BOOL) -> Self {
        Self { value }
    }

    /// Retrieves the inner `objc2_sys` boolean type, to be used in raw
    /// Objective-C APIs.
    #[inline]
    pub const fn as_raw(self) -> objc2_sys::BOOL {
        self.value
    }

    /// Returns `true` if `self` is [`NO`][`Self::NO`].
    #[inline]
    pub const fn is_false(self) -> bool {
        // Always compare with 0
        // This is what happens with the `!` operator in C.
        self.value as u8 == 0
    }

    /// Returns `true` if `self` is the opposite of [`NO`][`Self::NO`].
    #[inline]
    pub const fn is_true(self) -> bool {
        // Always compare with 0
        // This is what happens when using `if` in C.
        self.value as u8 != 0
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
        b.is_true()
    }
}

impl fmt::Debug for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(if self.is_true() { "YES" } else { "NO" })
    }
}

unsafe impl Encode for Bool {
    const ENCODING: Encoding<'static> = objc2_sys::BOOL::ENCODING;
}

unsafe impl RefEncode for Bool {
    const ENCODING_REF: Encoding<'static> = Encoding::Pointer(&Self::ENCODING);
}
