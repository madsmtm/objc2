//! # Bindings to the `Carbon` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/carbon/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-carbon/0.3.2")]

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

    pub(crate) type Ptr = *mut core::ffi::c_char;
    pub(crate) type Handle = *mut Ptr;
    pub(crate) type Size = core::ffi::c_long;

    pub(crate) type OSErr = i16;
    pub(crate) type OSStatus = i32;
    pub(crate) type LogicalAddress = *mut core::ffi::c_void;
    pub(crate) type ConstLogicalAddress = *const core::ffi::c_void;
    pub(crate) type PhysicalAddress = *mut core::ffi::c_void;
    pub(crate) type BytePtr = *mut u8;
    pub(crate) type ByteCount = core::ffi::c_ulong;
    pub(crate) type ByteOffset = core::ffi::c_ulong;
    pub(crate) type Duration = i32;
    pub(crate) type AbsoluteTime = i32;
    pub(crate) type OptionBits = u32;
    pub(crate) type ItemCount = core::ffi::c_ulong;
    pub(crate) type PBVersion = u32;
    pub(crate) type ScriptCode = i16;
    pub(crate) type LangCode = i16;
    pub(crate) type RegionCode = i16;
    pub(crate) type FourCharCode = u32;
    pub(crate) type OSType = FourCharCode;
    pub(crate) type ResType = FourCharCode;
    pub(crate) type OSTypePtr = *mut OSType;
    pub(crate) type ResTypePtr = *mut ResType;

    pub(crate) type Boolean = u8;

    pub(crate) type PRefCon = *mut core::ffi::c_void;
    #[cfg(target_pointer_width = "64")]
    pub(crate) type URefCon = *mut core::ffi::c_void;
    #[cfg(target_pointer_width = "64")]
    pub(crate) type SRefCon = *mut core::ffi::c_void;
    #[cfg(target_pointer_width = "32")]
    pub(crate) type URefCon = u32;
    #[cfg(target_pointer_width = "32")]
    pub(crate) type SRefCon = i32;

    pub(crate) type UniChar = u16;
    pub(crate) type UniCharCount = core::ffi::c_ulong;
    pub(crate) type Str255 = [core::ffi::c_uchar; 255];
    pub(crate) type Str63 = [core::ffi::c_uchar; 64];
    pub(crate) type Str32 = [core::ffi::c_uchar; 33];
    pub(crate) type Str31 = [core::ffi::c_uchar; 32];
    pub(crate) type Str27 = [core::ffi::c_uchar; 28];
    pub(crate) type Str15 = [core::ffi::c_uchar; 16];
    pub(crate) type Str32Field = [core::ffi::c_uchar; 34];
    pub(crate) type StrFileName = Str63;
    pub(crate) type StringPtr = *mut core::ffi::c_char;
    pub(crate) type ConstStringPtr = *const core::ffi::c_char;
    pub(crate) type ConstStr255Param = *const core::ffi::c_char;
    pub(crate) type ConstStr63Param = *const core::ffi::c_char;
    pub(crate) type ConstStrFileNameParam = ConstStr63Param;

    pub(crate) type Style = core::ffi::c_uchar;
    pub(crate) type StyleParameter = core::ffi::c_short;
    pub(crate) type StyleField = Style;

    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Default)]
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

    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Default)]
    #[allow(unreachable_pub)] // Intentionally don't make this truly public
    pub struct Rect {
        top: core::ffi::c_short,
        left: core::ffi::c_short,
        bottom: core::ffi::c_short,
        right: core::ffi::c_short,
    }
    #[cfg(feature = "objc2")]
    unsafe impl Encode for Rect {
        const ENCODING: Encoding = Encoding::Struct(
            "Rect",
            &[
                <core::ffi::c_short>::ENCODING,
                <core::ffi::c_short>::ENCODING,
                <core::ffi::c_short>::ENCODING,
                <core::ffi::c_short>::ENCODING,
            ],
        );
    }
    #[cfg(feature = "objc2")]
    unsafe impl RefEncode for Rect {
        const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
    }
}

#[allow(unused_imports)]
pub(crate) use self::mac_types::*;
