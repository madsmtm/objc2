extern crate objc_encode;

use objc_encode::Encode;
use objc_encode::encoding::{Primitive, Struct};

#[cfg(target_pointer_width = "32")]
type CGFloat = f32;

#[cfg(target_pointer_width = "64")]
type CGFloat = f64;

#[repr(C)]
struct CGPoint {
    x: CGFloat,
    y: CGFloat,
}

unsafe impl Encode for CGPoint {
    type Encoding = Struct<&'static str, (Primitive, Primitive)>;

    fn encode() -> Self::Encoding {
        Struct::new("CGPoint", (CGFloat::encode(), CGFloat::encode()))
    }
}

#[repr(C)]
struct CGSize {
    width: CGFloat,
    height: CGFloat,
}

unsafe impl Encode for CGSize {
    type Encoding = Struct<&'static str, (Primitive, Primitive)>;

    fn encode() -> Self::Encoding {
        Struct::new("CGSize", (CGFloat::encode(), CGFloat::encode()))
    }
}

#[repr(C)]
struct CGRect {
    origin: CGPoint,
    size: CGSize,
}

unsafe impl Encode for CGRect {
    type Encoding = Struct<&'static str, (<CGPoint as Encode>::Encoding,
                                          <CGSize as Encode>::Encoding)>;

    fn encode() -> Self::Encoding {
        Struct::new("CGRect", (CGPoint::encode(), CGSize::encode()))
    }
}

fn main() {
    println!("{}", CGRect::encode());
}
