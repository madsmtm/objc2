#![allow(
    unused_imports,
    dead_code,
    unused_variables,
    deprecated,
    non_snake_case
)]

use objc2::encode::{EncodeArguments, EncodeReturn};

// Automatically generated by header-translator, but not in Git, so only
// enable when requested, to better allow e.g. rust-analyzer to work when we
// haven't yet generated it.
#[cfg(feature = "test-frameworks")]
include!("imports.rs");

pub use block2;
pub use core::ffi::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
    c_ulong, c_ulonglong, c_ushort, c_void,
};
pub use core::ptr::NonNull;
pub use dispatch;
pub use libc;
pub use objc2::ffi::{NSInteger, NSUInteger};
pub use objc2::rc::{Allocated, Retained};
pub use objc2::runtime::{
    AnyClass, AnyObject, AnyProtocol, Bool, Imp, NSObject, NSObjectProtocol, ProtocolObject, Sel,
};
pub use objc2::{available, sel, ClassType, MainThreadMarker};

// MacTypes.h
pub type OSStatus = i32;
pub type Byte = u8;
pub type Boolean = u8; // unsigned char
pub type ConstStr255Param = *const core::ffi::c_char;
pub type ConstStringPtr = *const core::ffi::c_char;
pub type FourCharCode = u32;
pub type LangCode = i16;
pub type OSType = FourCharCode;
pub type RegionCode = i16;
pub type ResType = FourCharCode;
pub type StringPtr = *mut core::ffi::c_char;
pub type UniChar = u16;
pub type UTF32Char = u32; // Or maybe Rust's char?

#[track_caller]
pub fn check_method<Arguments: EncodeArguments, Return: EncodeReturn>(
    cls: &AnyClass,
    sel: Sel,
    _expected_encoding: &str,
) {
    let Some(method) = cls.instance_method(sel) else {
        // Some classes don't have the method available in the runtime;
        // we can't really do anything to test things then.
        return;
    };

    if let Err(err) = cls.verify_sel::<Arguments, Return>(sel) {
        panic!("could not verify selector {sel}\n    {err}");
    }

    // TODO: Parse the expected encoding, and check it.
    //
    // let cstr = unsafe { objc2::ffi::method_getTypeEncoding(method) };
    // assert!(!cstr.is_null());
    // let actual_encoding = unsafe { core::ffi::CStr::from_ptr(cstr) }
    //     .to_str()
    //     .expect("method type encoding must be UTF-8");
    //
    // assert_eq!(
    //     actual_encoding, expected_encoding,
    //     "method encoding in header did not match implementation for {sel}",
    // );
}

#[test]
fn smoke_test_encoding() {
    let encoding = if cfg!(target_pointer_width = "64") {
        "@16@0:8"
    } else {
        "@8@0:4"
    };
    check_method::<(), *mut NSObject>(NSObject::class(), sel!(self), encoding);
}