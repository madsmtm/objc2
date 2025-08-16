//! Matrix Math Utilities.
//!
//! In the future, these will likely be much easier with portable_simd and
//! ffi_simd, but for now, they're required (at least if you use matrix types
//! in your Metal shaders).

use std::arch::aarch64::{
    float32x4_t, float32x4x4_t, uint32x4_t, vdupq_n_f32, vdupq_n_u32, vgetq_lane_f32, vld1q_f32,
    vmlaq_n_f32, vmulq_n_f32,
};

use objc2::{Encode, Encoding};

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Float3(pub float32x4_t);

impl Float3 {
    pub fn splat(val: f32) -> Self {
        Self(unsafe { vdupq_n_f32(val) })
    }
}

unsafe impl Encode for Float3 {
    const ENCODING: Encoding = Encoding::None;
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct UInt3(pub uint32x4_t);

impl UInt3 {
    pub fn splat(val: u32) -> Self {
        Self(unsafe { vdupq_n_u32(val) })
    }
}

unsafe impl Encode for UInt3 {
    const ENCODING: Encoding = Encoding::None;
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Float4x4(pub float32x4x4_t);

impl Float4x4 {
    pub fn new(a: [f32; 4], b: [f32; 4], c: [f32; 4], d: [f32; 4]) -> Self {
        fn new_f4(arr: [f32; 4]) -> float32x4_t {
            unsafe { vld1q_f32(arr.as_ptr()) }
        }

        Self(float32x4x4_t(new_f4(a), new_f4(b), new_f4(c), new_f4(d)))
    }
}

pub fn matrix_translation(tx: f32, ty: f32, tz: f32) -> Float4x4 {
    Float4x4::new(
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [tx, ty, tz, 1.0],
    )
}

fn normalize(v: (f32, f32, f32)) -> (f32, f32, f32) {
    let (x, y, z) = v;
    let len = (x * x + y * y + z * z).sqrt();

    if len == 0.0 {
        (0.0, 0.0, 0.0) // handle zero-length vector safely
    } else {
        (x / len, y / len, z / len)
    }
}

pub fn matrix_rotation(radians: f32, axis: (f32, f32, f32)) -> Float4x4 {
    let axis = normalize(axis);
    let ct = radians.cos();
    let st = radians.sin();
    let ci = 1.0 - ct;
    let x = axis.0;
    let y = axis.1;
    let z = axis.2;

    #[rustfmt::skip]
    return Float4x4::new(
        [    ct + x * x * ci, y * x * ci + z * st, z * x * ci - y * st, 0.0],
        [x * y * ci - z * st,     ct + y * y * ci, z * y * ci + x * st, 0.0],
        [x * z * ci + y * st, y * z * ci - x * st,     ct + z * z * ci, 0.0],
        [                0.0,                 0.0,                 0.0, 1.0],
    );
}

pub fn matrix_perspective_right_hand(
    fovy_radians: f32,
    aspect: f32,
    near_z: f32,
    far_z: f32,
) -> Float4x4 {
    let ys = 1.0 / (fovy_radians * 0.5).tan();
    let xs = ys / aspect;
    let zs = far_z / (near_z - far_z);

    Float4x4::new(
        [xs, 0.0, 0.0, 0.0],
        [0.0, ys, 0.0, 0.0],
        [0.0, 0.0, zs, -1.0],
        [0.0, 0.0, near_z * zs, 0.0],
    )
}

pub fn matrix_multiply(a: Float4x4, b: Float4x4) -> Float4x4 {
    let a = a.0;
    let b = b.0;
    unsafe {
        let mut r0 = vmulq_n_f32(b.0, vgetq_lane_f32(a.0, 0));
        r0 = vmlaq_n_f32(r0, b.1, vgetq_lane_f32(a.0, 1));
        r0 = vmlaq_n_f32(r0, b.2, vgetq_lane_f32(a.0, 2));
        r0 = vmlaq_n_f32(r0, b.3, vgetq_lane_f32(a.0, 3));

        let mut r1 = vmulq_n_f32(b.0, vgetq_lane_f32(a.1, 0));
        r1 = vmlaq_n_f32(r1, b.1, vgetq_lane_f32(a.1, 1));
        r1 = vmlaq_n_f32(r1, b.2, vgetq_lane_f32(a.1, 2));
        r1 = vmlaq_n_f32(r1, b.3, vgetq_lane_f32(a.1, 3));

        let mut r2 = vmulq_n_f32(b.0, vgetq_lane_f32(a.2, 0));
        r2 = vmlaq_n_f32(r2, b.1, vgetq_lane_f32(a.2, 1));
        r2 = vmlaq_n_f32(r2, b.2, vgetq_lane_f32(a.2, 2));
        r2 = vmlaq_n_f32(r2, b.3, vgetq_lane_f32(a.2, 3));

        let mut r3 = vmulq_n_f32(b.0, vgetq_lane_f32(a.3, 0));
        r3 = vmlaq_n_f32(r3, b.1, vgetq_lane_f32(a.3, 1));
        r3 = vmlaq_n_f32(r3, b.2, vgetq_lane_f32(a.3, 2));
        r3 = vmlaq_n_f32(r3, b.3, vgetq_lane_f32(a.3, 3));

        Float4x4(float32x4x4_t(r0, r1, r2, r3))
    }
}
