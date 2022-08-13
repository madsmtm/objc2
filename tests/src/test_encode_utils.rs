#![allow(non_snake_case)]
use core::fmt::Display;
use core::sync::atomic::{AtomicI32, AtomicPtr};
use objc2::ffi::{NSInteger, NSUInteger};
use objc2::runtime::{Bool, Class, Object, Sel};
use objc2::{Encode, Encoding};
use paste::paste;
use std::ffi::CStr;
use std::os::raw::*;
use std::string::ToString;

use super::*;

unsafe fn assert_encoding(s: *const c_char, e: Encoding) {
    let s = CStr::from_ptr(s).to_str().unwrap();
    if !e.equivalent_to_str(s) {
        panic!("{} were not equivalent to {}", e, s);
    }
    // To ensure `equivalent_to_str` is implemented correctly:
    assert_eq!(e.to_string(), s.trim_start_matches('r'));
}

#[allow(unused)]
unsafe fn assert_str<T: Display>(s: *const c_char, expected: T) {
    let s = CStr::from_ptr(s).to_str().unwrap();
    // Exact comparison to ensure we catch regressions (and that they are not
    // just masked by ).
    assert_eq!(s, expected.to_string());
}

macro_rules! assert_inner {
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
            $(assert_types!(#$should_atomic);)?
            assert_inner!(enc $(#[$m])* $(#[cfg($should_atomic)])? [<ENCODING_ $stat _ATOMIC>] => Encoding::Atomic(&$encoding));
            assert_inner!(enc $(#[$m])* $(#[cfg($should_atomic)])? [<ENCODING_ $stat _ATOMIC_POINTER>] => Encoding::Pointer(&Encoding::Atomic(&$encoding)));
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
            $(assert_types!(#$should_atomic);)?
            assert_inner!(enc $(#[$m])* $(#[cfg($should_atomic)])? [<ENCODING_ $stat _ATOMIC>] => Encoding::Atomic(&<$type>::ENCODING));
            assert_inner!(enc $(#[$m])* $(#[cfg($should_atomic)])? [<ENCODING_ $stat _ATOMIC_POINTER>] => Encoding::Pointer(&Encoding::Atomic(&<$type>::ENCODING)));
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER_ATOMIC>] => Encoding::Atomic(&<*const $type>::ENCODING));
        }
    };
    (#no_atomic) => {};
}

const WITH_ATOMIC_INNER: Encoding = Encoding::Struct(
    "with_atomic_inner",
    &[
        AtomicI32::ENCODING,
        <*const AtomicI32>::ENCODING,
        <AtomicPtr<c_int>>::ENCODING,
    ],
);

assert_types! {
    // C types

    C99_BOOL => bool,
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

    #[cfg(feature = "apple")]
    BITFIELD => enc Encoding::Struct(
        "bitfield",
        &[
            Encoding::BitField(1, &Encoding::UInt),
            Encoding::BitField(30, &Encoding::UInt),
        ],
    ),

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

    OBJC_BOOL => Bool,
    ID => enc <*mut Object>::ENCODING,
    CLASS => enc <&Class>::ENCODING,
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

    // `intptr`, `uintptr` and `size_t` are cfg-guarded because they are
    // simply just too much of a hassle to get working on this old platform.
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

    UUID_T no_atomic => enc Encoding::Array(16, &u8::ENCODING),

    // Possible extras; need to be #[cfg]-ed somehow

    // SIGNED_INT_128 => i128,
    // UNSIGNED_INT_128 => u128,
}

// Bitfields

#[cfg(feature = "gnustep-1-7")]
mod bitfields {
    use super::*;

    assert_inner!(str ENCODING_BITFIELD => "{bitfield=b0I1b1I30}");
    assert_inner!(str ENCODING_BITFIELD_POINTER => "^{bitfield=b0I1b1I30}");
    assert_inner!(str ENCODING_BITFIELD_ATOMIC => "A{bitfield}");
}
