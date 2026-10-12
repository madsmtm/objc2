#![allow(non_upper_case_globals)]
use core::fmt;
use core::num::NonZeroI32;
#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, OptionEncode, RefEncode};

/// No error occurred.
pub const kAXErrorSuccess: Result<(), AXError> = Ok(());

/// Accessibility Error Codes
///
/// Error codes returned by accessibility functions.
///
/// NOTE: This is _not_ ABI-compatible with C's `AXError`, since it contains
/// [`NonZeroI32`]. Use `Result<(), AXError>` instead in FFI function
/// signatures, that has the same ABI as [`i32`]!
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/applicationservices/axerror?language=objc)
// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AXError(pub NonZeroI32);

#[cfg(feature = "objc2")]
unsafe impl Encode for AXError {
    const ENCODING: Encoding = NonZeroI32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for AXError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "objc2")]
unsafe impl OptionEncode for AXError {}

impl AXError {
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

    /// A system error occurred, such as the failure to allocate an object.
    #[doc(alias = "kAXErrorFailure")]
    pub const Failure: Self = Self(new(-25200));

    /// An illegal argument was passed to the function.
    #[doc(alias = "kAXErrorIllegalArgument")]
    pub const IllegalArgument: Self = Self(new(-25201));

    /// The AXUIElementRef passed to the function is invalid.
    #[doc(alias = "kAXErrorInvalidUIElement")]
    pub const InvalidUIElement: Self = Self(new(-25202));

    /// The AXObserverRef passed to the function is not a valid observer.
    #[doc(alias = "kAXErrorInvalidUIElementObserver")]
    pub const InvalidUIElementObserver: Self = Self(new(-25203));

    /// The function cannot complete because messaging failed in some way or because the application with which the function is communicating is busy or unresponsive.
    #[doc(alias = "kAXErrorCannotComplete")]
    pub const CannotComplete: Self = Self(new(-25204));

    /// The attribute is not supported by the AXUIElementRef.
    #[doc(alias = "kAXErrorAttributeUnsupported")]
    pub const AttributeUnsupported: Self = Self(new(-25205));

    /// The action is not supported by the AXUIElementRef.
    #[doc(alias = "kAXErrorActionUnsupported")]
    pub const ActionUnsupported: Self = Self(new(-25206));

    /// The notification is not supported by the AXUIElementRef.
    #[doc(alias = "kAXErrorNotificationUnsupported")]
    pub const NotificationUnsupported: Self = Self(new(-25207));

    /// Indicates that the function or method is not implemented (this can be returned if a process does not support the accessibility API).
    #[doc(alias = "kAXErrorNotImplemented")]
    pub const NotImplemented: Self = Self(new(-25208));

    /// This notification has already been registered for.
    #[doc(alias = "kAXErrorNotificationAlreadyRegistered")]
    pub const NotificationAlreadyRegistered: Self = Self(new(-25209));

    /// Indicates that a notification is not registered yet.
    #[doc(alias = "kAXErrorNotificationNotRegistered")]
    pub const NotificationNotRegistered: Self = Self(new(-25210));

    /// The accessibility API is disabled (as when, for example, the user deselects "Enable access for assistive devices" in Universal Access Preferences).
    #[doc(alias = "kAXErrorAPIDisabled")]
    pub const APIDisabled: Self = Self(new(-25211));

    /// The requested value or AXUIElementRef does not exist.
    #[doc(alias = "kAXErrorNoValue")]
    pub const NoValue: Self = Self(new(-25212));

    /// The parameterized attribute is not supported by the AXUIElementRef.
    #[doc(alias = "kAXErrorParameterizedAttributeUnsupported")]
    pub const ParameterizedAttributeUnsupported: Self = Self(new(-25213));

    /// Not enough precision.
    #[doc(alias = "kAXErrorNotEnoughPrecision")]
    pub const NotEnoughPrecision: Self = Self(new(-25214));

    const fn as_str(self) -> Option<&'static str> {
        Some(match self {
            Self::Failure => "a system error occurred, such as the failure to allocate an object",
            Self::IllegalArgument => "an illegal argument was passed to the function",
            Self::InvalidUIElement => "the AXUIElementRef passed to the function is invalid",
            Self::InvalidUIElementObserver => "the AXObserverRef passed to the function is not a valid observer",
            Self::CannotComplete => "the function cannot complete because messaging failed in some way or because the application with which the function is communicating is busy or unresponsive",
            Self::AttributeUnsupported => "the attribute is not supported by the AXUIElementRef",
            Self::ActionUnsupported => "the action is not supported by the AXUIElementRef",
            Self::NotificationUnsupported => "the notification is not supported by the AXUIElementRef",
            Self::NotImplemented => "indicates that the function or method is not implemented",
            Self::NotificationAlreadyRegistered => "this notification has already been registered for",
            Self::NotificationNotRegistered => "indicates that a notification is not registered yet",
            Self::APIDisabled => "the accessibility API is disabled",
            Self::NoValue => "the requested value or AXUIElementRef does not exist",
            Self::ParameterizedAttributeUnsupported => "the parameterized attribute is not supported by the AXUIElementRef",
            Self::NotEnoughPrecision => "not enough precision",
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

impl fmt::Debug for AXError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Usually, for normal enums, we'd prefer to derive `Debug`, but for
        // `AXError`, that's a bit too opaque, it just prints the error code.
        //
        // So instead, we enrich this with the error string. This makes
        // `.unwrap()` and similar work much better.
        if let Some(s) = self.as_str() {
            write!(f, "{s} ({})", self.0)
        } else {
            write!(f, "unknown AXError ({})", self.0)
        }
    }
}

impl fmt::Display for AXError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(s) = self.as_str() {
            write!(f, "{s}")
        } else {
            write!(f, "unknown AXError ({})", self.0)
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for AXError {}
