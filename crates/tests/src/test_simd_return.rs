use core::ffi::{c_char, c_float};

use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods, mutability, ClassType, Encode, Encoding};

extern_class!(
    struct TestSimdReturn;

    unsafe impl ClassType for TestSimdReturn {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

macro_rules! methods {
    ($(
        $name:ident: $ty:ty { $expr:expr }
    )*) => {
        extern_methods!(
            unsafe impl TestSimdReturn {$(
                #[method($name)]
                fn $name() -> $ty;
            )*}
        );

        $(
            #[test]
            fn $name() {
                let res = TestSimdReturn::$name();
                assert_eq!(res, $expr);
            }
        )*
    };
}

macro_rules! encode_none {
    ($ty:ty) => {
        unsafe impl Encode for $ty {
            const ENCODING: Encoding = Encoding::None;
        }
    };
}

#[repr(simd)]
#[derive(PartialEq, Debug)]
struct Float2([f32; 2]);
encode_none!(Float2);

#[repr(simd)]
#[derive(PartialEq, Debug)]
struct Float3([f32; 3]);
encode_none!(Float3);

#[repr(simd)]
#[derive(PartialEq, Debug)]
struct Float4([f32; 4]);
encode_none!(Float4);

#[repr(transparent)]
#[derive(PartialEq, Debug)]
struct Float8([f32; 8]);
encode_none!(Float8);

#[repr(transparent)]
#[derive(PartialEq, Debug)]
struct Float16([f32; 16]);
encode_none!(Float16);

#[repr(simd)]
#[derive(PartialEq, Debug)]
struct Char2([i8; 2]);
encode_none!(Char2);

#[repr(simd)]
#[derive(PartialEq, Debug)]
struct Char3([i8; 3]);
encode_none!(Char3);

#[repr(simd)]
#[derive(PartialEq, Debug)]
struct Char4([i8; 4]);
encode_none!(Char4);

#[repr(simd)]
#[derive(PartialEq, Debug)]
struct Char8([i8; 8]);
encode_none!(Char8);

#[repr(simd)]
#[derive(PartialEq, Debug)]
struct Char16([i8; 16]);
encode_none!(Char16);

#[repr(transparent)]
#[derive(PartialEq, Debug)]
struct Char32([i8; 32]);
encode_none!(Char32);

#[repr(transparent)]
#[derive(PartialEq, Debug)]
struct Char64([i8; 64]);
encode_none!(Char64);

#[repr(C)]
#[derive(PartialEq, Debug)]
struct Quatf(Float4);
unsafe impl Encode for Quatf {
    const ENCODING: Encoding = Encoding::Struct("?", &[Encoding::None]);
}

#[repr(C)]
#[derive(PartialEq, Debug)]
struct Float2x2([Float2; 2]);
unsafe impl Encode for Float2x2 {
    const ENCODING: Encoding = Encoding::Struct("?", &[Encoding::Array(2, &Encoding::None)]);
}

#[repr(C)]
#[derive(PartialEq, Debug)]
struct Float2x4([Float4; 2]);
unsafe impl Encode for Float2x4 {
    const ENCODING: Encoding = Encoding::Struct("?", &[Encoding::Array(2, &Encoding::None)]);
}

#[repr(C)]
#[derive(PartialEq, Debug)]
struct Float4x4([Float4; 4]);
unsafe impl Encode for Float4x4 {
    const ENCODING: Encoding = Encoding::Struct("?", &[Encoding::Array(4, &Encoding::None)]);
}

methods! {
    float1: c_float { 42.0 }
    float2: Float2 { Float2([42.0; 2]) }
    float3: Float3 { Float3([42.0; 3]) }
    float4: Float4 { Float4([42.0; 4]) }
    float8: Float8 { Float8([42.0; 8]) }
    float16: Float16 { Float16([42.0; 16]) }

    char1: c_char { 42 }
    char2: Char2 { Char2([42; 2]) }
    char3: Char3 { Char3([42; 3]) }
    char4: Char4 { Char4([42; 4]) }
    char8: Char8 { Char8([42; 8]) }
    char16: Char16 { Char16([42; 16]) }
    char32: Char32 { Char32([42; 32]) }
    char64: Char64 { Char64([42; 64]) }

    quatf: Quatf { Quatf(Float4([42.0; 4])) }

    float2x2: Float2x2 { Float2x2([Float2([42.0; 2]), Float2([42.0; 2])]) }
    float2x4: Float2x4 { Float2x4([Float4([42.0; 4]), Float4([42.0; 4])]) }
    float4x4: Float4x4 { Float4x4([Float4([42.0; 4]), Float4([42.0; 4]), Float4([42.0; 4]), Float4([42.0; 4])]) }
}
