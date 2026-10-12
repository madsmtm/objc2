#![allow(non_upper_case_globals)]
use core::fmt;
use core::num::NonZeroI32;
#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, OptionEncode, RefEncode};

/// CoreVideo-specific result type.
///
/// This has the same memory layout and ABI as [`i32`].
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/cvreturn?language=objc)
pub type CVReturn = Result<(), CVError>;

/// Function executed successfully without errors.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturnsuccess?language=objc)
pub const kCVReturnSuccess: CVReturn = Ok(());

/// CoreVideo-specific errors.
///
/// This is used in [`CVReturn`] to make it work well with Rust's [`Result`].
///
/// See also [Apple's Swift documentation](https://developer.apple.com/documentation/corevideo/cverror)
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CVError(pub NonZeroI32);

#[cfg(feature = "objc2")]
unsafe impl Encode for CVError {
    const ENCODING: Encoding = NonZeroI32::ENCODING;
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for CVError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(feature = "objc2")]
unsafe impl OptionEncode for CVError {}

impl CVError {
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

    /// Placeholder to mark the beginning of the range of CVReturn codes.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturnfirst?language=objc)
    #[doc(alias = "kCVReturnFirst")]
    pub const First: Self = Self::Error;

    /// An otherwise undefined error occurred.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturnerror?language=objc)
    #[doc(alias = "kCVReturnError")]
    pub const Error: Self = Self(new(-6660));
    /// At least one of the arguments passed in is not valid. Either out of range or the wrong type.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturninvalidargument?language=objc)
    #[doc(alias = "kCVReturnInvalidArgument")]
    pub const InvalidArgument: Self = Self(new(-6661));
    /// The allocation for a buffer or buffer pool failed. Most likely because of lack of resources.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturnallocationfailed?language=objc)
    #[doc(alias = "kCVReturnAllocationFailed")]
    pub const AllocationFailed: Self = Self(new(-6662));
    /// [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturnunsupported?language=objc)
    #[doc(alias = "kCVReturnUnsupported")]
    pub const Unsupported: Self = Self(new(-6663));
    /// A CVDisplayLink cannot be created for the given DisplayRef.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturninvaliddisplay?language=objc)
    #[doc(alias = "kCVReturnInvalidDisplay")]
    pub const InvalidDisplay: Self = Self(new(-6670));
    /// The CVDisplayLink is already started and running.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturndisplaylinkalreadyrunning?language=objc)
    #[doc(alias = "kCVReturnDisplayLinkAlreadyRunning")]
    pub const DisplayLinkAlreadyRunning: Self = Self(new(-6671));
    /// The CVDisplayLink has not been started.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturndisplaylinknotrunning?language=objc)
    #[doc(alias = "kCVReturnDisplayLinkNotRunning")]
    pub const DisplayLinkNotRunning: Self = Self(new(-6672));
    /// The output callback is not set.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturndisplaylinkcallbacksnotset?language=objc)
    #[doc(alias = "kCVReturnDisplayLinkCallbacksNotSet")]
    pub const DisplayLinkCallbacksNotSet: Self = Self(new(-6673));
    /// The requested pixelformat is not supported for the CVBuffer type.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturninvalidpixelformat?language=objc)
    #[doc(alias = "kCVReturnInvalidPixelFormat")]
    pub const InvalidPixelFormat: Self = Self(new(-6680));
    /// The requested size (most likely too big) is not supported for the CVBuffer type.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturninvalidsize?language=objc)
    #[doc(alias = "kCVReturnInvalidSize")]
    pub const InvalidSize: Self = Self(new(-6681));
    /// A CVBuffer cannot be created with the given attributes.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturninvalidpixelbufferattributes?language=objc)
    #[doc(alias = "kCVReturnInvalidPixelBufferAttributes")]
    pub const InvalidPixelBufferAttributes: Self = Self(new(-6682));
    /// The Buffer cannot be used with OpenGL as either its size, pixelformat or attributes are not supported by OpenGL.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturnpixelbuffernotopenglcompatible?language=objc)
    #[doc(alias = "kCVReturnPixelBufferNotOpenGLCompatible")]
    pub const PixelBufferNotOpenGLCompatible: Self = Self(new(-6683));
    /// The Buffer cannot be used with Metal as either its size, pixelformat or attributes are not supported by Metal.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturnpixelbuffernotmetalcompatible?language=objc)
    #[doc(alias = "kCVReturnPixelBufferNotMetalCompatible")]
    pub const PixelBufferNotMetalCompatible: Self = Self(new(-6684));
    /// The allocation request failed because it would have exceeded a specified allocation threshold (see kCVPixelBufferPoolAllocationThresholdKey).
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturnwouldexceedallocationthreshold?language=objc)
    #[doc(alias = "kCVReturnWouldExceedAllocationThreshold")]
    pub const WouldExceedAllocationThreshold: Self = Self(new(-6689));
    /// The allocation for the buffer pool failed. Most likely because of lack of resources. Check if your parameters are in range.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturnpoolallocationfailed?language=objc)
    #[doc(alias = "kCVReturnPoolAllocationFailed")]
    pub const PoolAllocationFailed: Self = Self(new(-6690));
    /// A CVBufferPool cannot be created with the given attributes.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturninvalidpoolattributes?language=objc)
    #[doc(alias = "kCVReturnInvalidPoolAttributes")]
    pub const InvalidPoolAttributes: Self = Self(new(-6691));
    /// a scan hasn't completely traversed the CVBufferPool due to a concurrent operation. The client can retry the scan.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturnretry?language=objc)
    #[doc(alias = "kCVReturnRetry")]
    pub const Retry: Self = Self(new(-6692));

    /// Placeholder to mark the end of the range of CVReturn codes.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/corevideo/kcvreturnlast?language=objc)
    #[doc(alias = "kCVReturnLast")]
    pub const Last: Self = Self(new(-6699));

    const fn as_str(self) -> Option<&'static str> {
        Some(match self {
            Self::Error => "an otherwise undefined error occurred",
            Self::InvalidArgument => "at least one of the arguments passed in is not valid, either out of range or the wrong type",
            Self::AllocationFailed => "the allocation for a buffer or buffer pool failed, most likely because of lack of resources",
            Self::Unsupported => "unsupported",
            Self::InvalidDisplay => "a CVDisplayLink cannot be created for the given DisplayRef",
            Self::DisplayLinkAlreadyRunning => "the CVDisplayLink is already started and running",
            Self::DisplayLinkNotRunning => "the CVDisplayLink has not been started",
            Self::DisplayLinkCallbacksNotSet => "the output callback is not set",
            Self::InvalidPixelFormat => "the requested pixelformat is not supported for the CVBuffer type",
            Self::InvalidSize => "the requested size (most likely too big) is not supported for the CVBuffer type",
            Self::InvalidPixelBufferAttributes => "a CVBuffer cannot be created with the given attributes",
            Self::PixelBufferNotOpenGLCompatible => "the Buffer cannot be used with OpenGL as either its size, pixelformat or attributes are not supported by OpenGL",
            Self::PixelBufferNotMetalCompatible => "the Buffer cannot be used with Metal as either its size, pixelformat or attributes are not supported by Metal",
            Self::WouldExceedAllocationThreshold => "the allocation request failed because it would have exceeded a specified allocation threshold",
            Self::PoolAllocationFailed => "the allocation for the buffer pool failed, most likely because of lack of resources, check if your parameters are in range",
            Self::InvalidPoolAttributes => "a CVBufferPool cannot be created with the given attributes",
            Self::Retry => "a scan hasn't completely traversed the CVBufferPool due to a concurrent operation, the client can retry the scan",
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

impl fmt::Debug for CVError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Usually, for normal enums, we'd prefer to derive `Debug`, but for
        // `CVError`, that's a bit too opaque, it just prints the error code.
        //
        // So instead, we enrich this with the error string. This makes
        // `.unwrap()` and similar work much better.
        if let Some(s) = self.as_str() {
            write!(f, "{s} ({})", self.0)
        } else {
            write!(f, "unknown CVReturn ({})", self.0)
        }
    }
}

impl fmt::Display for CVError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(s) = self.as_str() {
            write!(f, "{s}")
        } else {
            write!(f, "unknown CVReturn ({})", self.0)
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CVError {}
