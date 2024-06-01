use objc2::encode::{Encode, Encoding, RefEncode};

// Define manually, `objc2_foundation::CGFloat` is feature-gated behind the
// `NSGeometry` feature.
#[cfg(target_pointer_width = "64")]
type CGFloat = f64;
#[cfg(not(target_pointer_width = "64"))]
type CGFloat = f32;

/// Vector.
///
/// This technically belongs to the `CoreGraphics` framework, but we define it
/// here for convenience.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cgvector?language=objc).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct CGVector {
    pub dx: CGFloat,
    pub dy: CGFloat,
}

unsafe impl Encode for CGVector {
    const ENCODING: Encoding =
        Encoding::Struct("CGVector", &[CGFloat::ENCODING, CGFloat::ENCODING]);
}

unsafe impl RefEncode for CGVector {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl CGVector {
    #[inline]
    #[doc(alias = "CGVectorMake")]
    pub const fn new(dx: CGFloat, dy: CGFloat) -> Self {
        Self { dx, dy }
    }
}

/// Affine transform.
///
/// This technically belongs to the `CoreGraphics` framework, but we define it
/// here for convenience.
///
/// See [Apple's documentation](https://developer.apple.com/documentation/corefoundation/cgaffinetransform?language=objc).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct CGAffineTransform {
    pub a: CGFloat,
    pub b: CGFloat,
    pub c: CGFloat,
    pub d: CGFloat,
    pub tx: CGFloat,
    pub ty: CGFloat,
}

unsafe impl Encode for CGAffineTransform {
    const ENCODING: Encoding = Encoding::Struct(
        "CGAffineTransform",
        &[
            CGFloat::ENCODING,
            CGFloat::ENCODING,
            CGFloat::ENCODING,
            CGFloat::ENCODING,
            CGFloat::ENCODING,
            CGFloat::ENCODING,
        ],
    );
}

unsafe impl RefEncode for CGAffineTransform {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
