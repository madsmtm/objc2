#![allow(unused_imports)]
#![allow(dead_code)]

pub(crate) use core::ffi::c_void;
pub(crate) use core::marker::PhantomData;
pub(crate) use core::ptr::NonNull;
#[cfg(feature = "std")]
pub(crate) use std::ffi::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
    c_ulong, c_ulonglong, c_ushort,
};

#[cfg(feature = "objective-c")]
pub(crate) use objc2::ffi::{NSInteger, NSUInteger};
#[cfg(feature = "objective-c")]
pub(crate) use objc2::rc::{Allocated, Id, Shared};
#[cfg(feature = "objective-c")]
pub(crate) use objc2::runtime::{Bool, Class, Object, Sel};
#[cfg(feature = "objective-c")]
pub(crate) use objc2::{__inner_extern_class, extern_class, extern_methods, ClassType, Message};

#[cfg(feature = "blocks")]
pub(crate) use block2::Block;

// TODO
#[cfg(feature = "objective-c")]
pub(crate) type Protocol = Object;
pub(crate) type TodoFunction = *const c_void;
#[cfg(feature = "objective-c")]
pub(crate) type TodoClass = Object;
#[cfg(feature = "objective-c")]
pub(crate) type TodoProtocols = Object;

// MacTypes.h
pub(crate) type Boolean = u8; // unsigned char
pub(crate) type FourCharCode = u32;
pub(crate) type OSType = FourCharCode;
pub(crate) type ResType = FourCharCode;
pub(crate) type UTF32Char = u32; // Or maybe Rust's char?
