//! # Bindings to the `IOBluetooth` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/iobluetooth/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-io-bluetooth/0.3.2")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
mod macros;

#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[allow(unused_imports, unreachable_pub)]
pub use self::macros::*;

// IOKit/IOReturn.h
#[allow(dead_code)]
pub(crate) type IOReturn = core::ffi::c_int; // kern_return_t

// IOKit/IOTypes.h
#[allow(dead_code)]
pub(crate) type IOItemCount = u32; // UInt32

// MacTypes.h
#[allow(dead_code)]
pub(crate) type Boolean = u8;
#[allow(dead_code)]
pub(crate) type ByteCount = core::ffi::c_ulong;
