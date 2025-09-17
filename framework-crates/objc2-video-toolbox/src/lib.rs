//! # Bindings to the `VideoToolbox` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/videotoolbox/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-video-toolbox/0.3.1")]

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
pub(crate) type OSStatus = i32;
#[allow(dead_code)]
pub(crate) type OSType = u32;

// NOTE: `VTRAWProcessingSessionRef` is marked as `CM_SWIFT_NONSENDABLE`, but
// `$(xcrun --show-sdk-path)/usr/lib/swift/VideoToolbox.swiftmodule/*`
// includes an `@unchecked Swift.Sendable` extension, so might be safe to
// mark it as sendable?
