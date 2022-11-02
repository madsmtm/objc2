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

macro_rules! struct_impl {
    (
        $v:vis struct $name:ident {
            $($field_v:vis $field:ident: $ty:ty),* $(,)?
        }
    ) => {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        $v struct $name {
            $($field_v $field: $ty,)*
        }

        #[cfg(feature = "objective-c")]
        unsafe impl objc2::Encode for $name {
            const ENCODING: objc2::Encoding = objc2::Encoding::Struct(
                stringify!($name),
                &[$(<$ty as objc2::Encode>::ENCODING),*],
            );
        }

        #[cfg(feature = "objective-c")]
        unsafe impl objc2::RefEncode for $name {
            const ENCODING_REF: objc2::Encoding = objc2::Encoding::Pointer(&<Self as objc2::Encode>::ENCODING);
        }
    };
}

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
    pub(crate) use objc2::rc::{Allocated, Id, Shared};
    pub(crate) use objc2::runtime::{Bool, Class, Object, Sel};
    pub(crate) use objc2::{
        __inner_extern_class, extern_class, extern_methods, ClassType, Message,
    };

    // TODO
    pub struct OptionSel(*const objc2::ffi::objc_selector);
    unsafe impl objc2::Encode for OptionSel {
        const ENCODING: objc2::Encoding = objc2::Encoding::Sel;
    }

    // TODO
    pub(crate) type Protocol = Object;
    pub(crate) type TodoBlock = *const c_void;
    pub(crate) type TodoFunction = *const c_void;
    pub(crate) type TodoArray = *const c_void;
    pub(crate) type TodoClass = Object;
    pub(crate) type TodoProtocols = Object;

    // MacTypes.h
    pub(crate) type Boolean = u8; // unsigned char
    pub(crate) type FourCharCode = u32;
    pub(crate) type OSType = FourCharCode;
    pub(crate) type ResType = FourCharCode;
    pub(crate) type UTF32Char = u32; // Or maybe Rust's char?
}
