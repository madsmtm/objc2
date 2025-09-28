//! # Bindings to the `Network` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/network/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-network/0.3.1")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;
mod generated;
#[macro_use]
pub(crate) mod macros;
mod object;
mod retained;

use core::{
    cell::UnsafeCell,
    marker::{PhantomData, PhantomPinned},
};

#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
pub use self::object::NWObject;
pub use self::retained::NWRetained;

// Helper type
type OpaqueData = UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>;

// fixme: Generated as ArrayUnknownABI
#[allow(non_camel_case_types)]
pub type nw_ethernet_address_t = [core::ffi::c_uchar; 6];
