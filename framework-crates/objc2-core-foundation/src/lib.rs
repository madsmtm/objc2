//! # Bindings to the `CoreFoundation` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/corefoundation/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-coerce-pointee", feature(derive_coerce_pointee))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-core-foundation/0.3.0")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[doc(hidden)]
pub mod __cf_macro_helpers;
#[cfg(feature = "CFBase")]
mod base;
#[cfg(feature = "CFBundle")]
mod bundle;
mod cf_type;
#[cfg(feature = "CFData")]
mod data;
#[cfg(feature = "CFDate")]
mod date;
#[cfg(feature = "CFError")]
mod error;
mod generated;
#[cfg(feature = "CFCGTypes")]
mod geometry;
#[cfg(feature = "CFNumber")]
mod number;
mod retained;
#[cfg(feature = "CFString")]
mod string;
#[cfg(feature = "CFTimeZone")]
mod timezone;
mod type_traits;
#[cfg(feature = "CFUUID")]
mod uuid;

#[cfg(feature = "CFBase")]
pub use self::base::*;
#[cfg(feature = "CFBundle")]
pub use self::bundle::CFBundleRefNum;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
#[cfg(feature = "CFCGTypes")]
pub use self::geometry::*;
pub use self::retained::CFRetained;
pub use self::type_traits::{ConcreteType, Type};

// MacTypes.h
#[allow(dead_code)]
mod mac_types {
    pub(crate) type Boolean = u8; // unsigned char
    pub(crate) type ConstStr255Param = *const core::ffi::c_char;
    pub(crate) type ConstStringPtr = *const core::ffi::c_char;
    pub(crate) type FourCharCode = u32;
    pub(crate) type LangCode = i16;
    pub(crate) type OSType = FourCharCode;
    pub(crate) type RegionCode = i16;
    pub(crate) type ResType = FourCharCode;
    pub(crate) type StringPtr = *mut core::ffi::c_char;
    pub(crate) type UniChar = u16;
    pub(crate) type UTF32Char = u32; // Or maybe Rust's char?
}

#[allow(unused_imports)]
pub(crate) use self::mac_types::*;
