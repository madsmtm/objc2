#![allow(non_upper_case_globals)]
use core::ffi::c_int;
use core::fmt;
use core::num::NonZeroI32;
#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, OptionEncode, RefEncode};

/// DiskArbitration-specific result type.
///
/// This has the same memory layout and ABI as [`c_int`].
///
/// [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/dareturn?language=objc)
pub type DAReturn = Result<(), DAError>;

/// [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/kdareturnsuccess?language=objc)
pub const kDAReturnSuccess: DAReturn = Ok(());

/// DiskArbitration-specific errors.
///
/// This is used in [`DAReturn`] to make it work well with Rust's [`Result`].
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// mach_error_t => kern_return_t => int => i32
// TODO(MSRV): Use NonZero<c_int>.
pub struct DAError(pub NonZeroI32);

#[cfg(feature = "objc2")]
unsafe impl Encode for DAError {
    const ENCODING: Encoding = NonZeroI32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for DAError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "objc2")]
unsafe impl OptionEncode for DAError {}

impl DAError {
    #[inline]
    pub const fn from_raw(value: c_int) -> Option<Self> {
        if let Some(value) = NonZeroI32::new(value) {
            Some(Self(value))
        } else {
            None
        }
    }

    #[inline]
    pub const fn get(self) -> c_int {
        self.0.get()
    }

    /// [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/kdareturnerror?language=objc)
    #[doc(alias = "kDAReturnError")]
    pub const Error: Self = Self(new(-119930879)); // 0xF8DA0001
    /// [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/kdareturnbusy?language=objc)
    #[doc(alias = "kDAReturnBusy")]
    pub const Busy: Self = Self(new(-119930878)); // 0xF8DA0002
    /// [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/kdareturnbadargument?language=objc)
    #[doc(alias = "kDAReturnBadArgument")]
    pub const BadArgument: Self = Self(new(-119930877)); // 0xF8DA0003
    /// [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/kdareturnexclusiveaccess?language=objc)
    #[doc(alias = "kDAReturnExclusiveAccess")]
    pub const ExclusiveAccess: Self = Self(new(-119930876)); // 0xF8DA0004
    /// [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/kdareturnnoresources?language=objc)
    #[doc(alias = "kDAReturnNoResources")]
    pub const NoResources: Self = Self(new(-119930875)); // 0xF8DA0005
    /// [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/kdareturnnotfound?language=objc)
    #[doc(alias = "kDAReturnNotFound")]
    pub const NotFound: Self = Self(new(-119930874)); // 0xF8DA0006
    /// [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/kdareturnnotmounted?language=objc)
    #[doc(alias = "kDAReturnNotMounted")]
    pub const NotMounted: Self = Self(new(-119930873)); // 0xF8DA0007
    /// [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/kdareturnnotpermitted?language=objc)
    #[doc(alias = "kDAReturnNotPermitted")]
    pub const NotPermitted: Self = Self(new(-119930872)); // 0xF8DA0008
    /// [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/kdareturnnotprivileged?language=objc)
    #[doc(alias = "kDAReturnNotPrivileged")]
    pub const NotPrivileged: Self = Self(new(-119930871)); // 0xF8DA0009
    /// [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/kdareturnnotready?language=objc)
    #[doc(alias = "kDAReturnNotReady")]
    pub const NotReady: Self = Self(new(-119930870)); // 0xF8DA000A
    /// [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/kdareturnnotwritable?language=objc)
    #[doc(alias = "kDAReturnNotWritable")]
    pub const NotWritable: Self = Self(new(-119930869)); // 0xF8DA000B
    /// [Apple's documentation](https://developer.apple.com/documentation/diskarbitration/kdareturnunsupported?language=objc)
    #[doc(alias = "kDAReturnUnsupported")]
    pub const Unsupported: Self = Self(new(-119930868)); // 0xF8DA000C

    const fn as_str(self) -> Option<&'static str> {
        Some(match self {
            Self::Error => "error",
            Self::Busy => "busy",
            Self::BadArgument => "bad argument",
            Self::ExclusiveAccess => "exclusive access",
            Self::NoResources => "no resources",
            Self::NotFound => "not found",
            Self::NotMounted => "not mounted",
            Self::NotPermitted => "not permitted",
            Self::NotPrivileged => "not privileged",
            Self::NotReady => "not ready",
            Self::NotWritable => "not writable",
            Self::Unsupported => "unsupported",
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

impl fmt::Debug for DAError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Usually, for normal enums, we'd prefer to derive `Debug`, but for
        // `DAError`, that's a bit too opaque, it just prints the error code.
        //
        // So instead, we enrich this with the error string. This makes
        // `.unwrap()` and similar work much better.
        if let Some(s) = self.as_str() {
            write!(f, "{s} ({})", self.0)
        } else {
            write!(f, "unknown DAReturn ({})", self.0)
        }
    }
}

impl fmt::Display for DAError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(s) = self.as_str() {
            write!(f, "{s}")
        } else {
            write!(f, "unknown DAReturn ({})", self.0)
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DAError {}
