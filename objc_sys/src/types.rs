//! Objective-C types and type aliases.
#![allow(non_camel_case_types)]

#[cfg(target_os = "macos")]
use core::ffi::c_void;
use std::os::raw::{c_char, c_int};

/// The Objective-C `BOOL` type.
///
/// To convert an Objective-C `BOOL` into a Rust [`bool`], compare it with [`NO`].
#[cfg(all(target_vendor = "apple", not(target_arch = "aarch64")))]
pub type BOOL = i8;
#[cfg(all(not(target_vendor = "apple"), not(target_arch = "aarch64")))]
pub type BOOL = u8;
#[cfg(target_arch = "aarch64")]
pub type BOOL = bool;

/// The equivalent of true for Objective-C's [`BOOL`] type.
#[cfg(not(target_arch = "aarch64"))]
pub const YES: BOOL = 1;
#[cfg(target_arch = "aarch64")]
pub const YES: BOOL = true;

/// The equivalent of false for Objective-C's [`BOOL`] type.
#[cfg(not(target_arch = "aarch64"))]
pub const NO: BOOL = 0;
#[cfg(target_arch = "aarch64")]
pub const NO: BOOL = false;

/// A type that represents a method selector.
#[repr(C)]
pub struct objc_selector {
    _priv: [u8; 0],
}

/// An opaque type that represents an Objective-C class.
#[repr(C)]
pub struct objc_class {
    _priv: [u8; 0],
}

/// An opaque type that represents an instance of a class.
#[repr(C)]
pub struct objc_object {
    _priv: [u8; 0],
}

/// A type that represents an instance variable.
#[repr(C)]
pub struct objc_ivar {
    _priv: [u8; 0],
}

/// A type that represents a method in a class definition.
#[repr(C)]
pub struct objc_method {
    _priv: [u8; 0],
}

/// Nonstandard naming, actually... (TODO)
#[repr(C)]
pub struct objc_protocol {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct objc_property {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_method_description {
    pub name: *const objc_selector,
    pub types: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_property_attribute_t {
    pub name: *const c_char,
    pub value: *const c_char,
}

/// Remember that this is non-null!
pub type objc_exception_preprocessor = unsafe extern "C" fn(exception: id) -> id;

/// Remember that this is non-null!
pub type objc_exception_matcher = unsafe extern "C" fn(catch_type: Class, exception: id) -> c_int;

/// Remember that this is non-null!
pub type objc_uncaught_exception_handler = unsafe extern "C" fn(exception: id);

/// Remember that this is non-null!
#[cfg(target_os = "macos")]
pub type objc_exception_handler = unsafe extern "C" fn(unused: id, context: *mut c_void);

/// An immutable pointer to a selector.
///
/// Type alias provided for convenience.
pub type SEL = *const objc_selector;

/// A mutable pointer to a class.
///
/// Type alias provided for convenience.
pub type Class = *mut objc_class;

/// A mutable pointer to an object / instance.
///
/// Type alias provided for convenience.
pub type id = *mut objc_object;

/// An immutable pointer to an instance variable.
///
/// Type alias provided for convenience.
pub type Ivar = *const objc_ivar;

/// A mutable pointer to a method.
///
/// Type alias provided for convenience.
pub type Method = *mut objc_method;

/// An opaque type that represents a protocol.
///
/// Type alias provided for convenience.
pub type Protocol = objc_protocol;

/// A mutable pointer to a property.
///
/// Type alias provided for convenience.
pub type objc_property_t = *mut objc_property;

/// A pointer to the start of a method implementation.
///
/// Remember that this is non-null!
/// Use `Option<IMP>` where nullability is expected. TODO
pub type IMP = unsafe extern "C" fn();
