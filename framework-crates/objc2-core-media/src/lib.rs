//! # Bindings to the `CoreMedia` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/coremedia/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-core-media/0.3.1")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "CMBase")]
mod base;
mod generated;
#[cfg(feature = "CMBase")]
pub use self::base::{CMBaseClassVersion, CMStructVersion};
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

// MacTypes.h
#[allow(dead_code)]
pub(crate) type Boolean = u8;
#[allow(dead_code)]
pub(crate) type OSType = u32;
#[allow(dead_code)]
pub(crate) type OSStatus = i32;
#[allow(dead_code)]
pub(crate) type FourCharCode = u32;
