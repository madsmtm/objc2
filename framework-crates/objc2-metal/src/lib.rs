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
//! NOTE: To use [`MTLCreateSystemDefaultDevice()`] you need to link to
//! `CoreGraphics`, this can be done by using `objc2-core-graphics`, or by
//! doing:
//! ```rust
//! #[link(name = "CoreGraphics", kind = "framework")]
//! extern "C" {}
//! ```
//!
#![cfg_attr(
    not(feature = "MTLDevice"),
    doc = "[`MTLCreateSystemDefaultDevice`]: #needs-MTLDevice-feature"
)]
#![recursion_limit = "256"]
#![allow(non_snake_case)]
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-metal/0.3.1")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "MTLAccelerationStructureTypes")]
mod acceleration_structure_types;
#[cfg(feature = "MTLCaptureManager")]
mod capture;
#[cfg(feature = "MTLCounters")]
mod counters;
#[cfg(feature = "MTLDevice")]
mod device;
mod generated;
#[cfg(feature = "unstable-private")]
mod private;
#[cfg(feature = "MTLRasterizationRate")]
mod rasterization_rate;
#[cfg(feature = "MTLResource")]
mod resource;
mod slice;
#[cfg(feature = "MTLTexture")]
mod texture;
#[cfg(feature = "MTLTypes")]
mod types;

#[cfg(feature = "MTLAccelerationStructureTypes")]
pub use self::acceleration_structure_types::MTLPackedFloat3;
#[cfg(feature = "MTLCounters")]
pub use self::counters::*;
#[cfg(feature = "MTLDevice")]
pub use self::device::*;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[cfg(feature = "unstable-private")]
pub use self::private::MTLDevicePrivate;
#[cfg(feature = "MTLResource")]
pub use self::resource::*;
#[cfg(all(feature = "MTLRenderCommandEncoder", feature = "MTLCommandEncoder"))]
pub use self::slice::MTLRenderCommandEncoderSliceExt;
#[cfg(feature = "MTLTexture")]
pub use self::texture::*;
