//! # Bindings to the `Metal` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/metal/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
//!
//! Metal has tools for validating that you're using it correctly, using these
//! is highly recommended! See [Apple's documentation on it][apple-doc], or
//! run `man MetalValidation` to get information on environment variables.
//!
//! [apple-doc]: https://developer.apple.com/documentation/xcode/validating-your-apps-metal-api-usage/.
//!
//!
//! ## Example
//!
//! Drawing a rotating triangle.
//!
//! ```ignore
#![doc = include_str!("../examples/triangle.rs")]
//! ```
#![recursion_limit = "256"]
#![allow(non_snake_case)]
#![no_std]
#![cfg_attr(feature = "unstable-docsrs", feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-metal/0.2.0")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "Metal_MTLCaptureManager")]
mod capture;
#[cfg(feature = "Metal_MTLDevice")]
mod device;
mod generated;
#[cfg(feature = "Metal_MTLAccelerationStructureTypes")]
mod packed;
#[cfg(feature = "unstable-private")]
mod private;
#[cfg(feature = "Metal_MTLResource")]
mod resource;
mod slice;

#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
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
