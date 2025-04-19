//! # Bindings to the `ApplicationServices` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/applicationservices/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-application-services/0.3.1")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;

// MacTypes.h
#[allow(dead_code)]
mod mac_types {
    #[cfg(feature = "objc2")]
    use objc2::encode::{Encode, Encoding, RefEncode};

    pub(crate) type OSStatus = u32;
    pub(crate) type Boolean = u8;
    pub(crate) type FourCharCode = u32;
    pub(crate) type OSType = FourCharCode;
    pub(crate) type ResType = FourCharCode;
    pub(crate) type OptionBits = u32;

    pub(crate) type Ptr = *mut core::ffi::c_char;
    pub(crate) type Handle = *mut Ptr;

    pub(crate) type OSErr = i16;
    pub(crate) type ItemCount = core::ffi::c_ulong;

    pub(crate) type StringPtr = *mut core::ffi::c_char;
    pub(crate) type ConstStr255Param = *const core::ffi::c_char;
    pub(crate) type Str255 = [core::ffi::c_uchar; 255];
    pub(crate) type Str63 = [core::ffi::c_uchar; 64];
    pub(crate) type Str32 = [core::ffi::c_uchar; 33];
    pub(crate) type Str31 = [core::ffi::c_uchar; 32];

    pub(crate) type Style = core::ffi::c_uchar;

    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[allow(unreachable_pub)] // Intentionally don't make this truly public
    pub struct ProcessSerialNumber {
        high_long_of_psn: u32,
        low_long_of_psn: u32,
    }

    #[cfg(feature = "objc2")]
    unsafe impl Encode for ProcessSerialNumber {
        const ENCODING: Encoding =
            Encoding::Struct("ProcessSerialNumber", &[<u32>::ENCODING, <u32>::ENCODING]);
    }

    #[cfg(feature = "objc2")]
    unsafe impl RefEncode for ProcessSerialNumber {
        const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
    }

    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[allow(unreachable_pub)] // Intentionally don't make this truly public
    pub struct Point {
        v: core::ffi::c_short,
        h: core::ffi::c_short,
    }

    #[cfg(feature = "objc2")]
    unsafe impl Encode for Point {
        const ENCODING: Encoding = Encoding::Struct(
            "Point",
            &[
                <core::ffi::c_short>::ENCODING,
                <core::ffi::c_short>::ENCODING,
            ],
        );
    }

    #[cfg(feature = "objc2")]
    unsafe impl RefEncode for Point {
        const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
    }
}

#[allow(unused_imports)]
pub(crate) use self::mac_types::*;
