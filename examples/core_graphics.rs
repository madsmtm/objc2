extern crate objc_encode;

use objc_encode::{Encode, Encoding};

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
    const CODE: Encoding<'static> = Encoding::Struct("CGPoint", &[CGFloat::CODE, CGFloat::CODE]);
}

#[repr(C)]
struct CGSize {
    width: CGFloat,
    height: CGFloat,
}

unsafe impl Encode for CGSize {
    const CODE: Encoding<'static> = Encoding::Struct("CGSize", &[CGFloat::CODE, CGFloat::CODE]);
}

#[repr(C)]
struct CGRect {
    origin: CGPoint,
    size: CGSize,
}

unsafe impl Encode for CGRect {
    const CODE: Encoding<'static> = Encoding::Struct("CGRect", &[CGPoint::CODE, CGSize::CODE]);
}

fn main() {
    println!("{}", CGRect::CODE);
}
