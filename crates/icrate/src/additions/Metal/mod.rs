//! # Bindings to the `Metal` framework
//!
//! Metal has tools for validating that you're using it correctly, using these
//! is highly recommended! See [Apple's documentation][apple-doc], or run
//! `man MetalValidation` to get information on environment variables.
//!
//! [apple-doc]: https://developer.apple.com/documentation/xcode/validating-your-apps-metal-api-usage/.
//!
//!
//! ## Example
//!
//! Drawing a rotating triangle.
//!
//! ```ignore
#![doc = include_str!("../../../examples/metal.rs")]
//! ```

#[cfg(feature = "Metal_MTLCaptureManager")]
mod capture;
#[cfg(feature = "Metal_MTLDevice")]
mod device;
#[cfg(feature = "Metal_MTLAccelerationStructureTypes")]
mod packed;
#[cfg(feature = "unstable-private")]
mod private;
#[cfg(feature = "Metal_MTLResource")]
mod resource;
mod slice;

#[allow(unreachable_pub)]
#[allow(unused_imports)]
pub use crate::generated::Metal::*;

#[cfg(feature = "Metal_MTLAccelerationStructureTypes")]
pub use self::packed::{MTLPackedFloat3, __MTLPackedFloat3};
#[cfg(feature = "unstable-private")]
pub use self::private::MTLDevicePrivate;
#[cfg(feature = "Metal_MTLResource")]
pub use self::resource::*;
#[cfg(all(
    feature = "Metal_MTLRenderCommandEncoder",
    feature = "Metal_MTLCommandEncoder"
))]
pub use self::slice::MTLRenderCommandEncoderSliceExt;

// CoreFoundation
#[allow(dead_code)]
pub(crate) type CFTimeInterval = std::os::raw::c_double;
