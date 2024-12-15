use core::ffi::c_float;

use objc2::encode::{Encode, Encoding, RefEncode};

/// [Apple's documentation](https://developer.apple.com/documentation/metalperformanceshaders/mpspackedfloat3?language=objc)
///
/// This is similar to `MTLPackedFloat3`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MPSPackedFloat3 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

unsafe impl Encode for MPSPackedFloat3 {
    const ENCODING: Encoding = Encoding::Struct(
        "_MPSPackedFloat3",
        &[Encoding::Union(
            "?",
            &[
                Encoding::Struct(
                    "?",
                    &[c_float::ENCODING, c_float::ENCODING, c_float::ENCODING],
                ),
                Encoding::Array(3, &c_float::ENCODING),
            ],
        )],
    );
}

unsafe impl RefEncode for MPSPackedFloat3 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
