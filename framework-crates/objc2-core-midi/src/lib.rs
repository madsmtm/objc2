//! # Bindings to the `CoreMIDI` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/coremidi/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-core-midi/0.3.1")]

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
pub(crate) type Byte = u8;
#[allow(dead_code)]
pub(crate) type ByteCount = core::ffi::c_ulong;
#[allow(dead_code)]
pub(crate) type ItemCount = core::ffi::c_ulong;
