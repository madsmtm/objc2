#![no_std]
#![warn(elided_lifetimes_in_paths)]
#![deny(non_ascii_idents)]
#![warn(unreachable_pub)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::cargo)]
#![warn(clippy::ptr_as_ptr)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/icrate/0.0.1")]
#![recursion_limit = "512"]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "objective-c")]
pub use objc2;

// Frameworks
#[cfg(feature = "AppKit")]
pub mod AppKit;
#[cfg(feature = "Foundation")]
pub mod Foundation;

#[allow(unused_imports)]
mod common {
    pub(crate) use std::ffi::{
        c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
        c_ulong, c_ulonglong, c_ushort, c_void,
    };
    pub(crate) use std::marker::PhantomData;
    pub(crate) use std::ptr::NonNull;

    pub(crate) use objc2::ffi::{NSInteger, NSUInteger};
    pub(crate) use objc2::rc::{Id, Shared};
    pub(crate) use objc2::runtime::{Class, Object, Sel};
    pub(crate) use objc2::{
        __inner_extern_class, extern_class, extern_methods, ClassType, Message,
    };

    // TODO
    pub(crate) type Protocol = Object;
    pub(crate) type TodoBlock = *const c_void;
    pub(crate) type TodoFunction = *const c_void;
    pub(crate) type TodoArray = *const c_void;
    pub(crate) type TodoClass = Object;

    pub(crate) type Boolean = u8; // unsigned char
}
