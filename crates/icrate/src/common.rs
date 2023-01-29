#![allow(unused_imports)]
#![allow(dead_code)]

pub(crate) use core::marker::PhantomData;
pub(crate) use core::ptr::NonNull;
pub(crate) use libc::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
    c_ulong, c_ulonglong, c_ushort, c_void, gid_t, mode_t, uid_t,
};

#[cfg(feature = "objective-c")]
pub(crate) use objc2::ffi::{NSInteger, NSIntegerMax, NSUInteger, NSUIntegerMax, IMP};
#[cfg(feature = "objective-c")]
pub(crate) use objc2::rc::{Allocated, Id, Owned, Ownership, Shared};
#[cfg(feature = "objective-c")]
pub(crate) use objc2::runtime::{Bool, Class, Object, Sel};
#[cfg(feature = "objective-c")]
pub(crate) use objc2::runtime::{NSObject, NSObjectProtocol};
#[cfg(feature = "objective-c")]
pub(crate) use objc2::{
    __inner_extern_class, extern_class, extern_methods, extern_protocol, ClassType, Message,
    ProtocolObject, ProtocolType,
};

#[cfg(feature = "block")]
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
pub(crate) type ConstStr255Param = *const c_char;
pub(crate) type ConstStringPtr = *const c_char;
pub(crate) type FourCharCode = u32;
pub(crate) type LangCode = c_short;
pub(crate) type OSType = FourCharCode;
pub(crate) type RegionCode = c_short;
pub(crate) type ResType = FourCharCode;
pub(crate) type StringPtr = *mut c_char;
pub(crate) type UInt16 = c_ushort;
pub(crate) type UTF32Char = u32; // Or maybe Rust's char?

pub(crate) const INT64_MAX: i64 = i64::MAX;
