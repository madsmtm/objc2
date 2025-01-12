//! # Bindings to the `VideoToolbox` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/videotoolbox/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-video-toolbox/0.2.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "VTErrors")]
mod errors;
mod generated;
#[cfg(feature = "VTErrors")]
pub use self::errors::VTDecodeFrameFlags;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

// MacTypes.h
#[allow(dead_code)]
pub(crate) type Boolean = u8;
#[allow(dead_code)]
pub(crate) type OSStatus = i32;
