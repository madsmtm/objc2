use core::ffi::c_short;

use crate::{GDHandle, Handle, PixMapHandle, Rect};

#[cfg(feature = "objc2")]
use objc2::encode::{Encode, Encoding, RefEncode};

/// [Apple's documentation](https://developer.apple.com/documentation/applicationservices/gdevice?language=objc)
#[repr(C, packed(2))]
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_snake_case)]
pub struct GDevice {
    pub gdRefNum: c_short,
    pub gdID: c_short,
    pub gdType: c_short,
    pub gdITable: Handle,
    pub gdResPref: c_short,
    pub gdSearchProc: Handle,
    pub gdCompProc: Handle,
    pub gdFlags: c_short,
    pub gdPMap: PixMapHandle,
    pub gdRefCon: i32,
    pub gdNextGD: GDHandle,
    pub gdRect: Rect,
    pub gdMode: i32,
    pub gdCCBytes: c_short,
    pub gdCCDepth: c_short,
    pub gdCCXData: Handle,
    pub gdCCXMask: Handle,
    pub gdExt: Handle,
}

#[cfg(feature = "objc2")]
unsafe impl Encode for GDevice {
    const ENCODING: Encoding = Encoding::Struct(
        "GDevice",
        &[
            <c_short>::ENCODING,
            <c_short>::ENCODING,
            <c_short>::ENCODING,
            <Handle>::ENCODING,
            <c_short>::ENCODING,
            <Handle>::ENCODING,
            <Handle>::ENCODING,
            <c_short>::ENCODING,
            <PixMapHandle>::ENCODING,
            <i32>::ENCODING,
            Encoding::Pointer(&Encoding::Pointer(&Encoding::Struct("GDevice", &[]))),
            <Rect>::ENCODING,
            <i32>::ENCODING,
            <c_short>::ENCODING,
            <c_short>::ENCODING,
            <Handle>::ENCODING,
            <Handle>::ENCODING,
            <Handle>::ENCODING,
        ],
    );
}

#[cfg(feature = "objc2")]
unsafe impl RefEncode for GDevice {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
