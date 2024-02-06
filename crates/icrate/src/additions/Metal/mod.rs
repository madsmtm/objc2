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

mod capture;
mod device;
mod packed;
#[cfg(feature = "unstable-private")]
mod private;
mod shift_mask;
mod slice;

pub use crate::generated::Metal::*;

pub use self::packed::{MTLPackedFloat3, __MTLPackedFloat3};
pub use self::shift_mask::*;

pub type CFTimeInterval = std::os::raw::c_double;
