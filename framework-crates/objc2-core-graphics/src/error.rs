#![allow(non_upper_case_globals)]
use core::fmt;
use core::num::NonZeroI32;
#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, OptionEncode, RefEncode};

/// The requested operation was completed successfully.
pub const kCGErrorSuccess: Result<(), CGError> = Ok(());

/// CoreGraphics-specific errors.
///
/// NOTE: This is _not_ ABI-compatible with C's `CGError`, since it contains
/// [`NonZeroI32`]. Use `Result<(), CGError>` instead in FFI function
/// signatures, that has the same ABI as [`i32`]!
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/coregraphics/cgerror?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CGError(pub NonZeroI32);

#[cfg(feature = "objc2")]
unsafe impl Encode for CGError {
    const ENCODING: Encoding = NonZeroI32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CGError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "objc2")]
unsafe impl OptionEncode for CGError {}

impl CGError {
    #[inline]
    pub const fn from_raw(value: i32) -> Option<Self> {
        if let Some(value) = NonZeroI32::new(value) {
            Some(Self(value))
        } else {
            None
        }
    }

    #[inline]
    pub const fn get(self) -> i32 {
        self.0.get()
    }

    /// A general failure occurred.
    #[doc(alias = "kCGErrorFailure")]
    pub const Failure: Self = Self(new(1000));

    /// One or more of the parameters passed to a function are invalid. Check for NULL pointers.
    #[doc(alias = "kCGErrorIllegalArgument")]
    pub const IllegalArgument: Self = Self(new(1001));

    /// The parameter representing a connection to the window server is invalid.
    #[doc(alias = "kCGErrorInvalidConnection")]
    pub const InvalidConnection: Self = Self(new(1002));

    /// The `CPSProcessSerNum` or context identifier parameter is not valid.
    #[doc(alias = "kCGErrorInvalidContext")]
    pub const InvalidContext: Self = Self(new(1003));

    /// The requested operation is inappropriate for the parameters passed in, or the current system state.
    #[doc(alias = "kCGErrorCannotComplete")]
    pub const CannotComplete: Self = Self(new(1004));

    /// Return value from obsolete function stubs present for binary compatibility, but not typically called.
    #[doc(alias = "kCGErrorNotImplemented")]
    pub const NotImplemented: Self = Self(new(1006));

    /// A parameter passed in has a value that is inappropriate, or which does not map to a useful operation or value.
    #[doc(alias = "kCGErrorRangeCheck")]
    pub const RangeCheck: Self = Self(new(1007));

    /// A data type or token was encountered that did not match the expected type or token.
    #[doc(alias = "kCGErrorTypeCheck")]
    pub const TypeCheck: Self = Self(new(1008));

    /// The requested operation is not valid for the parameters passed in, or the current system state.
    #[doc(alias = "kCGErrorInvalidOperation")]
    pub const InvalidOperation: Self = Self(new(1010));

    /// The requested operation could not be completed as the indicated resources were not found.
    #[doc(alias = "kCGErrorNoneAvailable")]
    pub const NoneAvailable: Self = Self(new(1011));

    const fn as_str(self) -> Option<&'static str> {
        Some(match self {
            Self::Failure => "a general failure occurred",
            Self::IllegalArgument => "one or more of the parameters passed to a function are invalid",
            Self::InvalidConnection => "the parameter representing a connection to the window server is invalid",
            Self::InvalidContext => "the `CPSProcessSerNum` or context identifier parameter is not valid",
            Self::CannotComplete => "the requested operation is inappropriate for the parameters passed in, or the current system state",
            Self::NotImplemented => "not implemented",
            Self::RangeCheck => "a parameter passed in has a value that is inappropriate, or which does not map to a useful operation or value",
            Self::TypeCheck => "a data type or token was encountered that did not match the expected type or token",
            Self::InvalidOperation => "the requested operation is not valid for the parameters passed in, or the current system state",
            Self::NoneAvailable => "the requested operation could not be completed as the indicated resources were not found",
            _ => return None,
        })
    }
}

/// Helper to construct `NonZeroI32` in `const`.
///
/// TODO(MSRV): Replace with inline `NonZeroI32::new(x).unwrap()`.
const fn new(x: i32) -> NonZeroI32 {
    if let Some(x) = NonZeroI32::new(x) {
        x
    } else {
        panic!("was given a constant of 0")
    }
}

impl fmt::Debug for CGError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Usually, for normal enums, we'd prefer to derive `Debug`, but for
        // `CGError`, that's a bit too opaque, it just prints the error code.
        //
        // So instead, we enrich this with the error string. This makes
        // `.unwrap()` and similar work much better.
        if let Some(s) = self.as_str() {
            write!(f, "{s} ({})", self.0)
        } else {
            write!(f, "unknown CGError ({})", self.0)
        }
    }
}

impl fmt::Display for CGError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(s) = self.as_str() {
            write!(f, "{s}")
        } else {
            write!(f, "unknown CGError ({})", self.0)
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CGError {}
