//! Objective-C types and type aliases.

use core::cell::UnsafeCell;
#[cfg(target_os = "macos")]
use core::ffi::c_void;
use core::marker::{PhantomData, PhantomPinned};
use std::os::raw::{c_char, c_int};

// Opaque types

/// We don't know much about the actual structs, so better mark them `!Send`,
/// `!Sync`, `!Unpin` and as mutable behind shared references. Downstream
/// libraries can always manually opt in to these types afterwards. (It's
/// also less of a breaking change on our part if we re-add these later).
///
/// TODO: Replace this with `extern type` to also mark it as unsized.
type OpaqueData = PhantomData<(UnsafeCell<*const ()>, PhantomPinned)>;

/// A type that represents a method selector.
#[repr(C)]
pub struct objc_selector {
    _priv: [u8; 0],
    _p: OpaqueData,
}

/// An opaque type that represents an Objective-C class.
#[repr(C)]
pub struct objc_class {
    // `isa` field is deprecated, so we don't expose it here.
    // Use `class_getSuperclass` instead.
    _priv: [u8; 0],
    _p: OpaqueData,
}

/// An opaque type that represents an instance of a class.
#[repr(C)]
pub struct objc_object {
    // `isa` field is deprecated, so we don't expose it here.
    // Use `object_getClass` instead.
    _priv: [u8; 0],
    _p: OpaqueData,
}

/// A type that represents an instance variable.
#[repr(C)]
pub struct objc_ivar {
    _priv: [u8; 0],
    _p: OpaqueData,
}

/// A type that represents a method in a class definition.
#[repr(C)]
pub struct objc_method {
    _priv: [u8; 0],
    _p: OpaqueData,
}

/// Nonstandard naming, actually... (TODO)
#[repr(C)]
pub struct objc_protocol {
    _priv: [u8; 0],
    _p: OpaqueData,
}

#[repr(C)]
pub struct objc_property {
    _priv: [u8; 0],
    _p: OpaqueData,
}

// Data-carrying structs

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

// Type aliases

/// The Objective-C `BOOL` type.
///
/// To convert an Objective-C `BOOL` into a Rust [`bool`], compare it with
/// [`NO`][`super::NO`].
#[cfg(all(target_vendor = "apple", not(target_arch = "aarch64")))]
pub type BOOL = i8;
#[cfg(all(not(target_vendor = "apple"), not(target_arch = "aarch64")))]
pub type BOOL = u8;
#[cfg(target_arch = "aarch64")]
pub type BOOL = bool;

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
