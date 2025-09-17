//! Run test with:
//! ```sh
//! cargo test -ptests --features unstable-simd,objc2/unstable-static-class,objc2/disable-encoding-assertions
//! ```
use core::ffi::{c_char, c_float};

use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods, Encode, Encoding};

extern_class!(
    #[unsafe(super(NSObject))]
    struct TestSimdReturn;
);

macro_rules! methods {
    ($(
        $(#[$($m:tt)*])*
        $name:ident $(($padding:expr))?: $ty:ty { $expr:expr }
    )*) => {$(
        #[test]
        $(#[$($m)*])*
        fn $name() {
            #[allow(non_local_definitions)]
            impl TestSimdReturn {
                extern_methods!(
                    #[unsafe(method($name))]
                    fn $name() -> $ty;
                );
            }

            let res = TestSimdReturn::$name();
            #[allow(unnecessary_transmutes)]
            let res_bytes = unsafe { core::mem::transmute_copy::<$ty, [u8; {size_of::<$ty>() $(- $padding)?}]>(&res) };
            #[allow(unnecessary_transmutes)]
            let expr_bytes = unsafe { core::mem::transmute_copy::<$ty, [u8; {size_of::<$ty>() $(- $padding)?}]>(&$expr) };
            assert_eq!(res_bytes, expr_bytes);
        }
    )*};
}

macro_rules! encode_none {
    ($ty:ty) => {
        unsafe impl Encode for $ty {
            const ENCODING: Encoding = Encoding::None;
        }
    };
}

#[repr(simd)]
struct Float2([f32; 2]);
encode_none!(Float2);

#[repr(simd)]
struct Float3([f32; 3]);
encode_none!(Float3);

#[repr(simd)]
struct Float4([f32; 4]);
encode_none!(Float4);

#[repr(transparent)]
struct Float8([f32; 8]);
encode_none!(Float8);

#[repr(transparent)]
struct Float16([f32; 16]);
encode_none!(Float16);

#[repr(simd)]
struct Char2([i8; 2]);
encode_none!(Char2);

#[repr(simd)]
struct Char3([i8; 3]);
encode_none!(Char3);

#[repr(simd)]
struct Char4([i8; 4]);
encode_none!(Char4);

#[repr(simd)]
struct Char8([i8; 8]);
encode_none!(Char8);

#[repr(simd)]
struct Char16([i8; 16]);
encode_none!(Char16);

#[repr(transparent)]
struct Char32([i8; 32]);
encode_none!(Char32);

#[repr(transparent)]
struct Char64([i8; 64]);
encode_none!(Char64);

#[repr(C)]
struct Quatf(Float4);
unsafe impl Encode for Quatf {
    const ENCODING: Encoding = Encoding::Struct("?", &[Encoding::None]);
}

#[repr(C)]
struct Float2x2([Float2; 2]);
unsafe impl Encode for Float2x2 {
    const ENCODING: Encoding = Encoding::Struct("?", &[Encoding::Array(2, &Encoding::None)]);
}

#[repr(C)]
struct Float2x4([Float4; 2]);
unsafe impl Encode for Float2x4 {
    const ENCODING: Encoding = Encoding::Struct("?", &[Encoding::Array(2, &Encoding::None)]);
}

#[repr(C)]
struct Float4x4([Float4; 4]);
unsafe impl Encode for Float4x4 {
    const ENCODING: Encoding = Encoding::Struct("?", &[Encoding::Array(4, &Encoding::None)]);
}

// Note: Most of these are currently ignored, as Rust does not yet have
// support for SIMD types in FFI:
// <https://github.com/rust-lang/rust/issues/63068>
//
// Additionally, we're not currently handling them in message sending.
methods! {
    float1: c_float { 42.0 }
    #[cfg_attr(target_pointer_width = "32", ignore = "Rust does not yet support SIMD in FFI")]
    float2: Float2 { Float2([42.0; 2]) }
    #[cfg_attr(target_pointer_width = "32", ignore = "Rust does not yet support SIMD in FFI")]
    float3 (4): Float3 { Float3([42.0; 3]) }
    #[cfg_attr(target_pointer_width = "32", ignore = "Rust does not yet support SIMD in FFI")]
    float4: Float4 { Float4([42.0; 4]) }
    #[cfg_attr(not(target_arch = "aarch64"), ignore = "Rust does not yet support SIMD in FFI")]
    float8: Float8 { Float8([42.0; 8]) }
    #[cfg_attr(not(target_arch = "aarch64"), ignore = "Rust does not yet support SIMD in FFI")]
    float16: Float16 { Float16([42.0; 16]) }

    char1: c_char { 42 }
    char2: Char2 { Char2([42; 2]) }
    #[cfg_attr(target_arch = "x86", ignore = "Rust does not yet support SIMD in FFI")]
    #[cfg_attr(target_arch = "x86_64", ignore = "Rust does not yet support SIMD in FFI")]
    char3 (1): Char3 { Char3([42; 3]) }
    #[cfg_attr(target_arch = "x86", ignore = "Rust does not yet support SIMD in FFI")]
    #[cfg_attr(target_arch = "x86_64", ignore = "Rust does not yet support SIMD in FFI")]
    char4: Char4 { Char4([42; 4]) }
    #[cfg_attr(target_pointer_width = "32", ignore = "Rust does not yet support SIMD in FFI")]
    char8: Char8 { Char8([42; 8]) }
    #[cfg_attr(target_pointer_width = "32", ignore = "Rust does not yet support SIMD in FFI")]
    char16: Char16 { Char16([42; 16]) }
    #[cfg_attr(not(target_arch = "aarch64"), ignore = "Rust does not yet support SIMD in FFI")]
    char32: Char32 { Char32([42; 32]) }
    #[cfg_attr(not(target_arch = "aarch64"), ignore = "Rust does not yet support SIMD in FFI")]
    char64: Char64 { Char64([42; 64]) }

    #[cfg_attr(target_arch = "arm", ignore = "Rust does not yet support SIMD in FFI")]
    quatf: Quatf { Quatf(Float4([42.0; 4])) }

    #[cfg_attr(target_arch = "arm", ignore = "Rust does not yet support SIMD in FFI")]
    float2x2: Float2x2 { Float2x2([Float2([42.0; 2]), Float2([42.0; 2])]) }
    #[cfg_attr(target_arch = "arm", ignore = "Rust does not yet support SIMD in FFI")]
    #[cfg_attr(target_arch = "aarch64", ignore = "Rust does not yet support SIMD in FFI")]
    float2x4: Float2x4 { Float2x4([Float4([42.0; 4]), Float4([42.0; 4])]) }
    #[cfg_attr(target_arch = "arm", ignore = "Rust does not yet support SIMD in FFI")]
    #[cfg_attr(target_arch = "aarch64", ignore = "Rust does not yet support SIMD in FFI")]
    float4x4: Float4x4 { Float4x4([Float4([42.0; 4]), Float4([42.0; 4]), Float4([42.0; 4]), Float4([42.0; 4])]) }
}
