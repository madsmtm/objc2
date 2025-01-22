//! # Bindings to the `CoreGraphics` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/coregraphics/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-core-graphics/0.3.0")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[cfg(feature = "CGImage")]
mod image;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

#[allow(dead_code)]
pub(crate) type UniCharCount = core::ffi::c_ulong;
#[allow(dead_code)]
pub(crate) type UniChar = u16;
