#![allow(non_snake_case)]
use alloc::format;
use core::fmt::Display;
use objc2::ffi::{NSInteger, NSUInteger};
use objc2::runtime::{Bool, Class, Object, Sel};
use objc2::{Encode, Encoding, RefEncode};
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
        $stat:ident => $type:ident $($encoding:expr)?,
    )+) => {$(
        assert_types!(@ $(#[$m])* $stat => $type $($encoding)?);
    )+};
    (@
        $(#[$m:meta])*
        $stat:ident => enc $encoding:expr
    ) => {
        paste! {
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat>] => $encoding);
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER>] => Encoding::Pointer(&$encoding));
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER_POINTER>] => Encoding::Pointer(&Encoding::Pointer(&$encoding)));
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER_POINTER_POINTER>] => Encoding::Pointer(&Encoding::Pointer(&Encoding::Pointer(&$encoding))));
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _ATOMIC>] => Encoding::Atomic(&$encoding));
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _ATOMIC_POINTER>] => Encoding::Pointer(&Encoding::Atomic(&$encoding)));
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER_ATOMIC>] => Encoding::Atomic(&Encoding::Pointer(&$encoding)));
        }
    };
    (@
        $(#[$m:meta])*
        $stat:ident => $type:ident
    ) => {
        paste! {
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat>] => <$type>::ENCODING);
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER>] => <*const $type>::ENCODING);
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER_POINTER>] => <*const *const $type>::ENCODING);
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER_POINTER_POINTER>] => <*const *const *const $type>::ENCODING);
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _ATOMIC>] => Encoding::Atomic(&<$type>::ENCODING));
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _ATOMIC_POINTER>] => Encoding::Pointer(&Encoding::Atomic(&<$type>::ENCODING)));
            assert_inner!(enc $(#[$m])* [<ENCODING_ $stat _POINTER_ATOMIC>] => Encoding::Atomic(&<*const $type>::ENCODING));
        }
    };
}

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

    // VOID => void,

    // Struct

    // STRUCT_EMPTY
    // STRUCT_ONE_ITEM
    // STRUCT_NESTED
    // STRUCT_TWO_ITEMS
    // STRUCT_WITH_ARRAYS

    // Bitfields

    // BITFIELD

    // Array

    // ARRAY_INT
    // ARRAY_POINTER
    // ARRAY_NESTED
    // ARRAY_STRUCT

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

    // Possible extras; need to be #[cfg]-ed somehow

    // SIGNED_INT_128 => i128,
    // UNSIGNED_INT_128 => u128,
}

// _Atomic void* is not available
assert_inner!(enc ENCODING_VOID => <()>::ENCODING);
assert_inner!(enc ENCODING_VOID_POINTER => <*const c_void>::ENCODING);
assert_inner!(str ENCODING_VOID_POINTER_CONST => format!("r{}", <*const c_void>::ENCODING));
assert_inner!(enc ENCODING_VOID_POINTER_POINTER => <*const *const c_void>::ENCODING);

// Structs (atomics or double indirection erase type information)

const ENC0: Encoding<'static> = Encoding::Struct("empty", &[]);
assert_inner!(enc ENCODING_STRUCT_EMPTY => ENC0);
assert_inner!(enc ENCODING_STRUCT_EMPTY_POINTER => Encoding::Pointer(&ENC0));
assert_inner!(str ENCODING_STRUCT_EMPTY_POINTER_POINTER => "^^{empty}");
assert_inner!(str ENCODING_STRUCT_EMPTY_POINTER_POINTER_POINTER => "^^^{empty}");
assert_inner!(str ENCODING_STRUCT_EMPTY_ATOMIC => "A{empty}");

const ENC1: Encoding<'static> = Encoding::Struct("one_item", &[<*const c_void>::ENCODING]);
assert_inner!(enc ENCODING_STRUCT_ONE_ITEM => ENC1);
assert_inner!(enc ENCODING_STRUCT_ONE_ITEM_POINTER => Encoding::Pointer(&ENC1));
assert_inner!(str ENCODING_STRUCT_ONE_ITEM_POINTER_POINTER => "^^{one_item}");
assert_inner!(str ENCODING_STRUCT_ONE_ITEM_POINTER_POINTER_POINTER => "^^^{one_item}");
assert_inner!(str ENCODING_STRUCT_ONE_ITEM_ATOMIC => "A{one_item}");

// const ENC_NESTED: Encoding<'static> = Encoding::Struct("nested", &[ENC1, Encoding::Pointer(&ENC1)]);
// assert_inner!(enc ENCODING_STRUCT_NESTED => ENC_NESTED);
// assert_inner!(enc ENCODING_STRUCT_NESTED_POINTER => Encoding::Pointer(&ENC_NESTED));
assert_inner!(str ENCODING_STRUCT_NESTED => "{nested={one_item=^v}^{one_item}}");
assert_inner!(str ENCODING_STRUCT_NESTED_POINTER => "^{nested={one_item=^v}^{one_item}}");
assert_inner!(str ENCODING_STRUCT_NESTED_POINTER_POINTER => "^^{nested}");
assert_inner!(str ENCODING_STRUCT_NESTED_POINTER_POINTER_POINTER => "^^^{nested}");
assert_inner!(str ENCODING_STRUCT_NESTED_ATOMIC => "A{nested}");

const ENC2: Encoding<'static> = Encoding::Struct("two_items", &[f32::ENCODING, c_int::ENCODING]);
assert_inner!(enc ENCODING_STRUCT_TWO_ITEMS => ENC2);
assert_inner!(enc ENCODING_STRUCT_TWO_ITEMS_POINTER => Encoding::Pointer(&ENC2));
assert_inner!(str ENCODING_STRUCT_TWO_ITEMS_ATOMIC => "A{two_items}");

const WITH_ARRAYS: Encoding<'static> = Encoding::Struct(
    "with_arrays",
    &[
        <[c_int; 1]>::ENCODING,
        <[&c_int; 2]>::ENCODING,
        <&[c_int; 3]>::ENCODING,
    ],
);
assert_inner!(str ENCODING_STRUCT_WITH_ARRAYS => WITH_ARRAYS);
assert_inner!(str ENCODING_STRUCT_WITH_ARRAYS_POINTER => Encoding::Pointer(&WITH_ARRAYS));
assert_inner!(str ENCODING_STRUCT_WITH_ARRAYS_ATOMIC => "A{with_arrays}");

// Bitfields

#[cfg(feature = "apple")]
mod bitfields {
    use super::*;

    const BITFIELD: Encoding<'static> = Encoding::Struct(
        "bitfield",
        &[
            Encoding::BitField(1, &Encoding::UInt),
            Encoding::BitField(30, &Encoding::UInt),
        ],
    );
    assert_inner!(enc ENCODING_BITFIELD => BITFIELD);
    assert_inner!(enc ENCODING_BITFIELD_POINTER => Encoding::Pointer(&BITFIELD));
    assert_inner!(str ENCODING_BITFIELD_ATOMIC => "A{bitfield}");
}

#[cfg(feature = "gnustep-1-7")]
mod bitfields {
    use super::*;

    assert_inner!(str ENCODING_BITFIELD => "{bitfield=b0I1b1I30}");
    assert_inner!(str ENCODING_BITFIELD_POINTER => "^{bitfield=b0I1b1I30}");
    assert_inner!(str ENCODING_BITFIELD_ATOMIC => "A{bitfield}");
}

// Unions

const UNION: Encoding<'static> = Encoding::Union("union_", &[f32::ENCODING, c_int::ENCODING]);
assert_inner!(enc ENCODING_UNION => UNION);
assert_inner!(enc ENCODING_UNION_POINTER => Encoding::Pointer(&UNION));
assert_inner!(str ENCODING_UNION_ATOMIC => "A(union_)");

// Arrays (atomics are not supported)

type Array = [c_int; 10];
assert_inner!(enc ENCODING_ARRAY_INT => Array::ENCODING);
assert_inner!(enc ENCODING_ARRAY_INT_POINTER => Array::ENCODING_REF);

type Pointer = [*const c_int; 10];
assert_inner!(enc ENCODING_ARRAY_POINTER => Pointer::ENCODING);
assert_inner!(str ENCODING_ARRAY_POINTER_POINTER => Pointer::ENCODING_REF);

type Nested = [[c_int; 20]; 10];
assert_inner!(enc ENCODING_ARRAY_NESTED => Nested::ENCODING);
assert_inner!(str ENCODING_ARRAY_NESTED_POINTER => Nested::ENCODING_REF);

assert_inner!(enc ENCODING_ARRAY_STRUCT => Encoding::Array(0, &ENC2));
assert_inner!(str ENCODING_ARRAY_STRUCT_POINTER => Encoding::Pointer(&Encoding::Array(0, &ENC2)));

// UUIDs

assert_inner!(enc ENCODING_UUID_T => Encoding::Array(16, &u8::ENCODING));
assert_inner!(enc ENCODING_UUID_T_POINTER => Encoding::Pointer(&Encoding::Array(16, &u8::ENCODING)));
