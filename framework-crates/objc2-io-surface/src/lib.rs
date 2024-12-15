//! # Bindings to the `IOSurface` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/iosurface/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-io-surface/0.2.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

// MacTypes.h
#[allow(dead_code)]
pub(crate) type Boolean = u8;
#[allow(dead_code)]
pub(crate) type OSType = u32;

/// [Apple's documentation](https://developer.apple.com/documentation/iosurface/iosurfaceref?language=objc)
#[cfg(feature = "IOSurfaceRef")]
pub type IOSurfaceRef = *mut core::ffi::c_void;