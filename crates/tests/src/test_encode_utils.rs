#![allow(non_snake_case)]
use core::ffi::CStr;
use core::ffi::*;
use core::fmt::Display;
use core::sync::atomic::{AtomicI32, AtomicPtr};
use objc2::ffi::{NSInteger, NSUInteger};
use objc2::runtime::{AnyClass, AnyObject, Bool, Sel};
use objc2::{Encode, Encoding};
use paste::paste;
use std::string::ToString;

use super::*;

unsafe fn assert_encoding(s: *const c_char, e: Encoding) {
    let s = unsafe { CStr::from_ptr(s) }.to_str().unwrap();
    if !e.equivalent_to_str(s) {
        panic!("{} were not equivalent to {}", e, s);
    }
    // To ensure `equivalent_to_str` is implemented correctly:
    assert_eq!(e.to_string(), s.trim_start_matches('r'));
}

#[allow(unused)]
unsafe fn assert_str<T: Display>(s: *const c_char, expected: T) {
    let s = unsafe { CStr::from_ptr(s) }.to_str().unwrap();
    // Exact comparison to ensure we catch regressions.
    assert_eq!(s, expected.to_string());
}

macro_rules! assert_inner {
    (no_atomic $($t:tt)*) => {};
    (enc $(#[$m:meta])* $stat:ident => $expected:expr) => {
        $(#[$m])*
        #[test]
        fn $stat() {
            extern "C" {
                static $stat: *const c_char;
            }
            unsafe { assert_encoding($stat, $expected) };
        }
    };
    (str $(#[$m:meta])* $stat:ident => $expected:expr) => {
        $(#[$m])*
        #[test]
        fn $stat() {
            extern "C" {
                static $stat: *const c_char;
            }
            unsafe { assert_str($stat, $expected) };
        }
    };
}

macro_rules! assert_types {
    ($(
        $(#[$m:meta])*
        $stat:ident $($should_atomic:ident)? => $type:ident $($encoding:expr)?,
    )+) => {$(
        assert_types!(@ $(#[$m])* $stat $($should_atomic)? => $type $($encoding)?);
    )+};
    (@
        $(#[$m:meta])*
        $stat:ident $($should_atomic:ident)? => enc $encoding:expr
    ) => {
        paste! {
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat>] => $encoding);
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER>] => Encoding::Pointer(&$encoding));
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER_POINTER>] => Encoding::Pointer(&Encoding::Pointer(&$encoding)));
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER_POINTER_POINTER>] => Encoding::Pointer(&Encoding::Pointer(&Encoding::Pointer(&$encoding))));
            assert_inner!($($should_atomic)? enc $(#[$m])* [<ENCODING_ $stat _ATOMIC>] => Encoding::Atomic(&$encoding));
            assert_inner!($($should_atomic)? enc $(#[$m])* [<ENCODING_ $stat _ATOMIC_POINTER>] => Encoding::Pointer(&Encoding::Atomic(&$encoding)));
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER_ATOMIC>] => Encoding::Atomic(&Encoding::Pointer(&$encoding)));
        }
    };
    (@
        $(#[$m:meta])*
        $stat:ident $($should_atomic:ident)? => $type:ident
    ) => {
        paste! {
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat>] => <$type>::ENCODING);
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER>] => <*const $type>::ENCODING);
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER_POINTER>] => <*const *const $type>::ENCODING);
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER_POINTER_POINTER>] => <*const *const *const $type>::ENCODING);
            assert_inner!($($should_atomic)? enc $(#[$m])* [<ENCODING_ $stat _ATOMIC>] => Encoding::Atomic(&<$type>::ENCODING));
            assert_inner!($($should_atomic)? enc $(#[$m])* [<ENCODING_ $stat _ATOMIC_POINTER>] => Encoding::Pointer(&Encoding::Atomic(&<$type>::ENCODING)));
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER_ATOMIC>] => Encoding::Atomic(&<*const $type>::ENCODING));
        }
    };
}

const WITH_ATOMIC_INNER: Encoding = Encoding::Struct(
    "with_atomic_inner",
    &[
        AtomicI32::ENCODING,
        <*const AtomicI32>::ENCODING,
        <AtomicPtr<c_int>>::ENCODING,
    ],
);

#[cfg(not(feature = "gnustep-1-7"))]
const BITFIELD: Encoding = Encoding::Struct(
    "bitfield",
    &[
        Encoding::BitField(5, None),
        Encoding::BitField(0, None),
        Encoding::BitField(2, None),
    ],
);
#[cfg(not(feature = "gnustep-1-7"))]
const BITFIELD_ALL_TYPES: Encoding = Encoding::Struct(
    "bitfield_all_types",
    &[
        BITFIELD,
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
        Encoding::BitField(1, None),
    ],
);
#[cfg(feature = "gnustep-1-7")]
const BITFIELD: Encoding = Encoding::Struct(
    "bitfield",
    &[
        Encoding::BitField(5, Some(&(0, i8::ENCODING))),
        Encoding::BitField(0, Some(&(16, i16::ENCODING))),
        Encoding::BitField(2, Some(&(16, i8::ENCODING))),
    ],
);
#[cfg(feature = "gnustep-1-7")]
const BITFIELD_ALL_TYPES: Encoding = Encoding::Struct(
    "bitfield_all_types",
    &[
        BITFIELD,
        Encoding::BitField(1, Some(&(24, Encoding::Char))),
        Encoding::BitField(1, Some(&(25, Encoding::Short))),
        Encoding::BitField(1, Some(&(26, Encoding::Int))),
        Encoding::BitField(1, Some(&(27, Encoding::C_LONG))),
        Encoding::BitField(1, Some(&(28, Encoding::LongLong))),
        Encoding::BitField(1, Some(&(29, Encoding::Char))),
        Encoding::BitField(1, Some(&(30, Encoding::Short))),
        Encoding::BitField(1, Some(&(31, Encoding::Int))),
        Encoding::BitField(1, Some(&(32, Encoding::C_LONG))),
        Encoding::BitField(1, Some(&(33, Encoding::LongLong))),
        Encoding::BitField(1, Some(&(34, Encoding::UChar))),
        Encoding::BitField(1, Some(&(35, Encoding::UShort))),
        Encoding::BitField(1, Some(&(36, Encoding::UInt))),
        Encoding::BitField(1, Some(&(37, Encoding::C_ULONG))),
        Encoding::BitField(1, Some(&(38, Encoding::ULongLong))),
        Encoding::BitField(1, Some(&(39, Encoding::Bool))),
    ],
);

assert_types! {
    // C types

    C99_BOOL => enc Encoding::Bool,
    CHAR => c_char,
    SIGNED_CHAR => c_schar,
    UNSIGNED_CHAR => c_uchar,
    SHORT => c_short,
    UNSIGNED_SHORT => c_ushort,
    INT => c_int,
    UNSIGNED_INT => c_uint,
    // `long` is weird:
    LONG => enc Encoding::C_LONG,
    // `unsigned long` is weird:
    UNSIGNED_LONG => enc Encoding::C_ULONG,
    LONG_LONG => c_longlong,
    UNSIGNED_LONG_LONG => c_ulonglong,
    FLOAT => c_float,
    DOUBLE => c_double,
    // No appropriate Rust types for these:
    LONG_DOUBLE => enc Encoding::LongDouble, // long double
    FLOAT_COMPLEX => enc Encoding::FloatComplex, // float _Complex
    DOUBLE_COMPLEX => enc Encoding::DoubleComplex, // double _Complex
    LONG_DOUBLE_COMPLEX => enc Encoding::LongDoubleComplex, // long double _Complex

    // TODO:
    // FLOAT_IMAGINARY => float _Imaginary,
    // DOUBLE_IMAGINARY => double _Imaginary,
    // LONG_DOUBLE_IMAGINARY => long double _Imaginary,

    VOID no_atomic => enc Encoding::Void,

    // Struct

    STRUCT_EMPTY => enc Encoding::Struct("empty", &[]),
    STRUCT_ONE_ITEM => enc Encoding::Struct("one_item", &[<*const c_void>::ENCODING]),
    STRUCT_NESTED => enc Encoding::Struct(
        "nested",
        &[
            Encoding::Struct("one_item", &[<*const c_void>::ENCODING]),
            Encoding::Pointer(&Encoding::Struct("one_item", &[<*const c_void>::ENCODING]))
        ]
    ),
    STRUCT_TWO_ITEMS => enc Encoding::Struct("two_items", &[f32::ENCODING, c_int::ENCODING]),
    STRUCT_WITH_ARRAYS => enc Encoding::Struct(
        "with_arrays",
        &[
            <[c_int; 1]>::ENCODING,
            <[&c_int; 2]>::ENCODING,
            <&[c_int; 3]>::ENCODING,
        ],
    ),
    STRUCT_WITH_BLOCK no_atomic => enc Encoding::Struct(
        "with_block",
        &[
            Encoding::Block,
            Encoding::Object,
            Encoding::Pointer(&Encoding::Unknown),
        ],
    ),
    STRUCT_WITH_ATOMIC => enc Encoding::Struct(
        "with_atomic",
        &[
            AtomicI32::ENCODING,
            <*const AtomicI32>::ENCODING,
            <AtomicPtr<c_int>>::ENCODING,
            WITH_ATOMIC_INNER,
            Encoding::Pointer(&WITH_ATOMIC_INNER),
            Encoding::Pointer(&Encoding::Atomic(&WITH_ATOMIC_INNER)),
        ],
    ),

    // Bitfields

    BITFIELD => enc BITFIELD,
    BITFIELD_ALL_TYPES => enc BITFIELD_ALL_TYPES,

    // Unions

    UNION => enc Encoding::Union("union_", &[f32::ENCODING, c_int::ENCODING]),

    // Array

    ARRAY_INT no_atomic => enc <[c_int; 10]>::ENCODING,
    ARRAY_POINTER no_atomic => enc <[*const c_int; 10]>::ENCODING,
    ARRAY_NESTED no_atomic => enc <[[c_int; 20]; 10]>::ENCODING,
    ARRAY_STRUCT no_atomic => enc Encoding::Array(
        0,
        &Encoding::Struct("two_items", &[f32::ENCODING, c_int::ENCODING]),
    ),

    // Objective-C

    // https://github.com/llvm/llvm-project/issues/87490
    #[ignore = "pointers to booleans are broken on newer Clang versions"]
    OBJC_BOOL => Bool,
    ID => enc <*mut AnyObject>::ENCODING,
    CLASS => enc <&AnyClass>::ENCODING,
    // Sel is (intentionally) not RefEncode
    SEL => enc <Sel>::ENCODING,
    NS_INTEGER => NSInteger,
    NS_UINTEGER => NSUInteger,

    // stdint.h

    INT8 => i8,
    INT16 => i16,
    INT32 => i32,
    INT64 => i64,
    UINT8 => u8,
    UINT16 => u16,
    UINT32 => u32,
    UINT64 => u64,

    // `intptr`, `uintptr` and `size_t` are cfg-guarded because they are too
    // much of a hassle to get working on old platforms.
    //
    // Pointers (`intptr*`) works, but not plain `intptr`...

    #[cfg(target_pointer_width = "64")]
    INTPTR => isize,
    #[cfg(target_pointer_width = "64")]
    UINTPTR => usize,

    // stddef.h

    #[cfg(target_pointer_width = "64")]
    SIZE_T => usize,
    PTRDIFF_T => isize,

    // uuid.h

    #[cfg(target_vendor = "apple")]
    UUID_T no_atomic => enc Encoding::Array(16, &u8::ENCODING),

    // simd

    #[cfg(target_vendor = "apple")]
    SIMD_INT2 => enc Encoding::None,
    #[cfg(target_vendor = "apple")]
    SIMD_FLOAT1 => c_float,
    #[cfg(target_vendor = "apple")]
    SIMD_FLOAT2 => enc Encoding::None,
    #[cfg(target_vendor = "apple")]
    SIMD_FLOAT2X4 => enc Encoding::Struct("?", &[Encoding::Array(2, &Encoding::None)]),
    #[cfg(target_vendor = "apple")]
    SIMD_FLOAT4X2 => enc Encoding::Struct("?", &[Encoding::Array(4, &Encoding::None)]),

    // Possible extras; need to be #[cfg]-ed somehow

    // Supported by Rust, not by Apple's Clang (?)
    #[cfg(not(target_pointer_width = "32"))]
    SIGNED_INT_128 => i128,
    #[cfg(not(target_pointer_width = "32"))]
    UNSIGNED_INT_128 => u128,
}
