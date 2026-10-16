use core::ffi::c_float;

use objc2::encode::{Encode, Encoding, RefEncode};

/// [Apple's documentation](https://developer.apple.com/documentation/glkit/glkmatrix2?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct GLKMatrix2 {
    pub m: [c_float; 4],
}

unsafe impl Encode for GLKMatrix2 {
    const ENCODING: Encoding = Encoding::Union(
        "_GLKMatrix2",
        &[
            Encoding::Struct(
                "?",
                &[
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                ],
            ),
            <[[c_float; 2]; 2]>::ENCODING,
            <[c_float; 4]>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GLKMatrix2 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/glkit/glkmatrix3?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct GLKMatrix3 {
    pub m: [c_float; 9],
}

unsafe impl Encode for GLKMatrix3 {
    const ENCODING: Encoding = Encoding::Union(
        "_GLKMatrix3",
        &[
            Encoding::Struct(
                "?",
                &[
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                ],
            ),
            <[c_float; 9]>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GLKMatrix3 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/glkit/glkmatrix4?language=objc)
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct GLKMatrix4 {
    pub m: [c_float; 16],
}

unsafe impl Encode for GLKMatrix4 {
    const ENCODING: Encoding = Encoding::Union(
        "_GLKMatrix4",
        &[
            Encoding::Struct(
                "?",
                &[
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                ],
            ),
            <[c_float; 16]>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GLKMatrix4 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/glkit/glkvector2?language=objc)
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct GLKVector2 {
    pub v: [c_float; 2],
}

unsafe impl Encode for GLKVector2 {
    const ENCODING: Encoding = Encoding::Union(
        "_GLKVector2",
        &[
            Encoding::Struct("?", &[<c_float>::ENCODING, <c_float>::ENCODING]),
            Encoding::Struct("?", &[<c_float>::ENCODING, <c_float>::ENCODING]),
            <[c_float; 2]>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GLKVector2 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/glkit/glkvector3?language=objc)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct GLKVector3 {
    pub v: [c_float; 3],
}

unsafe impl Encode for GLKVector3 {
    const ENCODING: Encoding = Encoding::Union(
        "_GLKVector3",
        &[
            Encoding::Struct(
                "?",
                &[
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                ],
            ),
            Encoding::Struct(
                "?",
                &[
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                ],
            ),
            Encoding::Struct(
                "?",
                &[
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                ],
            ),
            <[c_float; 3]>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GLKVector3 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/glkit/glkvector4?language=objc)
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct GLKVector4 {
    pub v: [c_float; 4],
}

unsafe impl Encode for GLKVector4 {
    const ENCODING: Encoding = Encoding::Union(
        "_GLKVector4",
        &[
            Encoding::Struct(
                "?",
                &[
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                ],
            ),
            Encoding::Struct(
                "?",
                &[
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                ],
            ),
            Encoding::Struct(
                "?",
                &[
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                ],
            ),
            <[c_float; 4]>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GLKVector4 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// [Apple's documentation](https://developer.apple.com/documentation/glkit/glkquaternion?language=objc)
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct GLKQuaternion {
    pub q: [c_float; 4],
}

unsafe impl Encode for GLKQuaternion {
    const ENCODING: Encoding = Encoding::Union(
        "_GLKQuaternion",
        &[
            Encoding::Struct(
                "?",
                &[
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                    <c_float>::ENCODING,
                ],
            ),
            Encoding::Struct("?", &[<GLKVector3>::ENCODING, <c_float>::ENCODING]),
            <[c_float; 4]>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for GLKQuaternion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
#[cfg(feature = "alloc")]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn encoding() {
        assert_eq!(
            GLKMatrix2::ENCODING.to_string(),
            "(_GLKMatrix2={?=ffff}[2[2f]][4f])"
        );
        assert_eq!(
            GLKMatrix3::ENCODING.to_string(),
            "(_GLKMatrix3={?=fffffffff}[9f])"
        );
        assert_eq!(
            GLKMatrix4::ENCODING.to_string(),
            "(_GLKMatrix4={?=ffffffffffffffff}[16f])"
        );
        assert_eq!(
            GLKVector2::ENCODING.to_string(),
            "(_GLKVector2={?=ff}{?=ff}[2f])"
        );
        assert_eq!(
            GLKVector3::ENCODING.to_string(),
            "(_GLKVector3={?=fff}{?=fff}{?=fff}[3f])"
        );
        assert_eq!(
            GLKVector4::ENCODING.to_string(),
            "(_GLKVector4={?=ffff}{?=ffff}{?=ffff}[4f])"
        );
        assert_eq!(
            GLKQuaternion::ENCODING.to_string(),
            "(_GLKQuaternion={?=ffff}{?=(_GLKVector3={?=fff}{?=fff}{?=fff}[3f])f}[4f])"
        );
    }
}
