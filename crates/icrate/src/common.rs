#![allow(unused_imports)]
#![allow(dead_code)]

pub(crate) use core::ffi::c_void;
pub(crate) use core::marker::PhantomData;
pub(crate) use core::ptr::NonNull;
#[cfg(feature = "std")]
pub(crate) use std::os::raw::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
    c_ulong, c_ulonglong, c_ushort,
};

#[cfg(feature = "objc2")]
pub(crate) use objc2::ffi::{NSInteger, NSIntegerMax, NSUInteger, NSUIntegerMax, IMP};
#[cfg(feature = "objc2")]
pub(crate) use objc2::mutability::{
    Immutable, ImmutableWithMutableSubclass, InteriorMutable, IsIdCloneable, IsMainThreadOnly,
    MainThreadOnly, Mutable, MutableWithImmutableSuperclass,
};
#[cfg(feature = "objc2")]
pub(crate) use objc2::rc::{Allocated, DefaultId, Id};
#[cfg(feature = "objc2")]
pub(crate) use objc2::runtime::{AnyClass, AnyObject, Bool, Sel};
#[cfg(feature = "objc2")]
pub(crate) use objc2::runtime::{NSObject, NSObjectProtocol, ProtocolObject};
#[cfg(feature = "objc2")]
pub(crate) use objc2::{
    __inner_extern_class, extern_category, extern_class, extern_methods, extern_protocol,
    ClassType, Message, ProtocolType,
};

#[cfg(feature = "block2")]
pub(crate) use block2::Block;

// TODO
#[cfg(feature = "objc2")]
pub(crate) type AnyProtocol = AnyObject;
pub(crate) type TodoFunction = *const c_void;
#[cfg(feature = "objc2")]
pub(crate) type TodoClass = AnyObject;
#[cfg(feature = "objc2")]
pub(crate) type TodoProtocols = AnyObject;
