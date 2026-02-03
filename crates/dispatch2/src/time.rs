#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, RefEncode};

/// An abstract representation of time.
///
/// Zero means "now" and DISPATCH_TIME_FOREVER means "infinity" and every
/// value in between is an opaque encoding.
#[doc(alias = "dispatch_time_t")]
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct DispatchTime(pub u64);

#[cfg(feature = "objc2")]
// SAFETY: `DispatchTime` is `#[repr(transparent)]`.
unsafe impl Encode for DispatchTime {
    const ENCODING: Encoding = u64::ENCODING;
}

#[cfg(feature = "objc2")]
// SAFETY: Same as above.
unsafe impl RefEncode for DispatchTime {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl DispatchTime {
    /// The current time.
    #[doc(alias = "DISPATCH_TIME_NOW")]
    pub const NOW: Self = Self(0);

    /// A time in the distant future.
    #[doc(alias = "DISPATCH_TIME_FOREVER")]
    // TODO: Swift calls this `distantFuture`
    pub const FOREVER: Self = Self(u64::MAX);

    /// The current time relative to the clock used in `gettimeofday(3)`.
    #[doc(alias = "DISPATCH_WALLTIME_NOW")]
    pub const WALLTIME_NOW: Self = Self(!1);
}

// TODO: Expand this with inspiration from Time.swift in:
// https://github.com/swiftlang/swift-corelibs-libdispatch/blob/swift-6.1-RELEASE/src/swift/Time.swift
//
// Needs to invoke `mach_timebase_info`.
