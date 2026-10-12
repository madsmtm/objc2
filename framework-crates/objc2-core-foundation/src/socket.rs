#![allow(non_upper_case_globals)]
use core::fmt;
use core::num::NonZeroIsize;
#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, OptionEncode, RefEncode};

use crate::CFIndex;

/// The socket operation succeeded.
pub const kCFSocketSuccess: Result<(), CFSocketError> = Ok(());

/// Errors for many CFSocket functions.
///
/// NOTE: This is _not_ ABI-compatible with C's `CFSocketError`, since it
/// contains [`NonZeroIsize`]. Use `Result<(), CFSocketError>` instead in FFI
/// function signatures, that has the same ABI as [`CFIndex`]!
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cfsocketerror?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CFSocketError(pub NonZeroIsize); // CFIndex

#[cfg(feature = "objc2")]
unsafe impl Encode for CFSocketError {
    const ENCODING: Encoding = NonZeroIsize::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CFSocketError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "objc2")]
unsafe impl OptionEncode for CFSocketError {}

impl CFSocketError {
    #[inline]
    pub const fn from_raw(value: CFIndex) -> Option<Self> {
        if let Some(value) = NonZeroIsize::new(value) {
            Some(Self(value))
        } else {
            None
        }
    }

    #[inline]
    pub const fn get(self) -> CFIndex {
        self.0.get()
    }

    /// The socket operation failed.
    #[doc(alias = "kCFSocketError")]
    pub const Error: Self = Self(new(-1));

    /// The socket operation timed out.
    #[doc(alias = "kCFSocketTimeout")]
    pub const Timeout: Self = Self(new(-2));

    const fn as_str(self) -> Option<&'static str> {
        Some(match self {
            Self::Error => "the socket operation failed",
            Self::Timeout => "the socket operation timed out",
            _ => return None,
        })
    }
}

/// Helper to construct `NonZeroIsize` in `const`.
///
/// TODO(MSRV): Replace with inline `NonZeroIsize::new(x).unwrap()`.
const fn new(x: CFIndex) -> NonZeroIsize {
    if let Some(x) = NonZeroIsize::new(x) {
        x
    } else {
        panic!("was given a constant of 0")
    }
}

impl fmt::Debug for CFSocketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Usually, for normal enums, we'd prefer to derive `Debug`, but for
        // `CFSocketError`, that's a bit too opaque, it just prints the error
        // code.
        //
        // So instead, we enrich this with the error string. This makes
        // `.unwrap()` and similar work much better.
        if let Some(s) = self.as_str() {
            write!(f, "{s} ({})", self.0)
        } else {
            write!(f, "unknown CFSocketError ({})", self.0)
        }
    }
}

impl fmt::Display for CFSocketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(s) = self.as_str() {
            write!(f, "{s}")
        } else {
            write!(f, "unknown CFSocketError ({})", self.0)
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CFSocketError {}
