#![allow(non_snake_case)]
use alloc::format;
use core::fmt::Display;
use objc2::ffi::{NSInteger, NSUInteger};
use objc2::runtime::{Bool, Class, Object, Sel};
use objc2_encode::Encoding;
use paste::paste;
use std::ffi::CStr;
use std::os::raw::*;
use std::string::ToString;

unsafe fn assert_encoding(s: *const c_char, e: Encoding) {
    let s = CStr::from_ptr(s).to_str().unwrap();
    if !e.equivalent_to_str(s) {
        panic!("{} were not equivalent to {}", e, s);
    }
    // To ensure `equivalent_to_str` is implemented correctly:
    assert_eq!(e.to_string(), s.trim_start_matches('r'));
}

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
        $stat:ident => $type:ty,
    )+) => {$(
        paste! {
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat>] => <$type>::ENCODING);
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER>] => <*const $type>::ENCODING);
            assert_inner!(str $(#[$m])* [<ENCODING_ $stat _ATOMIC>] => format!("A{}", <$type>::ENCODING));
        }
    )+};
}

use super::*;
use objc2_encode::Encode;

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
    // LONG => c_long,
    // UNSIGNED_LONG => c_ulong,
    LONG_LONG => c_longlong,
    UNSIGNED_LONG_LONG => c_ulonglong,
    FLOAT => c_float,
    DOUBLE => c_double,
    // LONG_DOUBLE => c_longdouble,

    // FLOAT_COMPLEX => float _Complex,
    // DOUBLE_COMPLEX => double _Complex,
    // LONG_DOUBLE_COMPLEX => long double _Complex,
    // TODO:
    // FLOAT_IMAGINARY => float _Imaginary,
    // DOUBLE_IMAGINARY => double _Imaginary,
    // LONG_DOUBLE_IMAGINARY => long double _Imaginary,

    // VOID => void,

    // Array

    // TODO: INT_ARRAY => [c_int; 10],

    // Struct

    // TODO: Structs and such

    // // Objective-C

    OBJC_BOOL => Bool,
    ID => *mut Object,
    CLASS => &Class,
    // SEL => Sel,
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

    // Possible extras; need to be #[cfg]-ed somehow

    // SIGNED_INT_128 => i128,
    // UNSIGNED_INT_128 => u128,
}

// Sel is (intentionally) not RefEncode
assert_inner!(enc ENCODING_SEL => Sel::ENCODING);
assert_inner!(enc ENCODING_SEL_POINTER => Encoding::Pointer(&Sel::ENCODING));
assert_inner!(str ENCODING_SEL_ATOMIC => format!("A{}", Sel::ENCODING));

// _Atomic void* is not available
assert_inner!(enc ENCODING_VOID => <()>::ENCODING);
assert_inner!(enc ENCODING_VOID_POINTER => <*const c_void>::ENCODING);
assert_inner!(str ENCODING_VOID_POINTER_CONST => format!("r{}", <*const c_void>::ENCODING));
assert_inner!(enc ENCODING_VOID_POINTER_POINTER => <*const *const c_void>::ENCODING);

// `[unsigned] long`s are weird:

assert_inner!(enc ENCODING_LONG => Encoding::LONG);
assert_inner!(enc ENCODING_LONG_POINTER => Encoding::Pointer(&Encoding::LONG));
assert_inner!(str ENCODING_LONG_ATOMIC => format!("A{}", Encoding::LONG));

assert_inner!(enc ENCODING_UNSIGNED_LONG => Encoding::U_LONG);
assert_inner!(enc ENCODING_UNSIGNED_LONG_POINTER => Encoding::Pointer(&Encoding::U_LONG));
assert_inner!(str ENCODING_UNSIGNED_LONG_ATOMIC => format!("A{}", Encoding::U_LONG));

// No appropriate Rust types for these:

assert_inner!(enc ENCODING_LONG_DOUBLE => Encoding::LongDouble);
assert_inner!(enc ENCODING_LONG_DOUBLE_POINTER => Encoding::Pointer(&Encoding::LongDouble));
assert_inner!(str ENCODING_LONG_DOUBLE_ATOMIC => format!("A{}", Encoding::LongDouble));

assert_inner!(enc ENCODING_FLOAT_COMPLEX => Encoding::FloatComplex);
assert_inner!(enc ENCODING_FLOAT_COMPLEX_POINTER => Encoding::Pointer(&Encoding::FloatComplex));
assert_inner!(str ENCODING_FLOAT_COMPLEX_ATOMIC => format!("A{}", Encoding::FloatComplex));

assert_inner!(enc ENCODING_DOUBLE_COMPLEX => Encoding::DoubleComplex);
assert_inner!(enc ENCODING_DOUBLE_COMPLEX_POINTER => Encoding::Pointer(&Encoding::DoubleComplex));
assert_inner!(str ENCODING_DOUBLE_COMPLEX_ATOMIC => format!("A{}", Encoding::DoubleComplex));

assert_inner!(enc ENCODING_LONG_DOUBLE_COMPLEX => Encoding::LongDoubleComplex);
assert_inner!(enc ENCODING_LONG_DOUBLE_COMPLEX_POINTER => Encoding::Pointer(&Encoding::LongDoubleComplex));
assert_inner!(str ENCODING_LONG_DOUBLE_COMPLEX_ATOMIC => format!("A{}", Encoding::LongDoubleComplex));
