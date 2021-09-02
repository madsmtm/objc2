//! # Bindings to the Objective-C core runtime
#![no_std]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc-encode/1.1.0")]

extern crate std;

use core::ffi::c_void;
use std::os::raw::{c_char, c_int, c_uint};

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
pub struct Sel {
    _priv: [u8; 0],
}

/// A type that represents an Objective-C class.
#[repr(C)]
pub struct Class {
    _priv: [u8; 0],
}

/// A type that represents an instance of a class.
#[repr(C)]
pub struct Object {
    _priv: [u8; 0],
}

/// A type that represents an instance variable.
#[repr(C)]
pub struct Ivar {
    _priv: [u8; 0],
}

/// A type that represents a method in a class definition.
#[repr(C)]
pub struct Method {
    _priv: [u8; 0],
}

/// A type that represents an Objective-C protocol.
#[repr(C)]
pub struct Protocol {
    _priv: [u8; 0],
}

/// A pointer to the start of a method implementation.
pub type Imp = unsafe extern "C" fn();

#[link(name = "objc", kind = "dylib")]
extern "C" {
    pub fn sel_registerName(name: *const c_char) -> *const Sel;
    pub fn sel_getName(sel: *const Sel) -> *const c_char;

    pub fn class_getName(cls: *const Class) -> *const c_char;
    pub fn class_getSuperclass(cls: *const Class) -> *const Class;
    pub fn class_getInstanceSize(cls: *const Class) -> usize;
    pub fn class_getInstanceMethod(cls: *const Class, sel: *const Sel) -> *const Method;
    pub fn class_getInstanceVariable(cls: *const Class, name: *const c_char) -> *const Ivar;
    pub fn class_copyMethodList(cls: *const Class, outCount: *mut c_uint) -> *mut *const Method;
    pub fn class_copyIvarList(cls: *const Class, outCount: *mut c_uint) -> *mut *const Ivar;
    pub fn class_addMethod(
        cls: *mut Class,
        name: *const Sel,
        imp: Imp,
        types: *const c_char,
    ) -> BOOL;
    pub fn class_addIvar(
        cls: *mut Class,
        name: *const c_char,
        size: usize,
        alignment: u8,
        types: *const c_char,
    ) -> BOOL;
    pub fn class_addProtocol(cls: *mut Class, proto: *const Protocol) -> BOOL;
    pub fn class_conformsToProtocol(cls: *const Class, proto: *const Protocol) -> BOOL;
    pub fn class_copyProtocolList(cls: *const Class, outCount: *mut c_uint)
        -> *mut *const Protocol;

    pub fn objc_allocateClassPair(
        superclass: *const Class,
        name: *const c_char,
        extraBytes: usize,
    ) -> *mut Class;
    pub fn objc_disposeClassPair(cls: *mut Class);
    pub fn objc_registerClassPair(cls: *mut Class);

    pub fn class_createInstance(cls: *const Class, extraBytes: usize) -> *mut Object;
    pub fn object_dispose(obj: *mut Object) -> *mut Object;
    pub fn object_getClass(obj: *const Object) -> *const Class;

    pub fn objc_getClassList(buffer: *mut *const Class, bufferLen: c_int) -> c_int;
    pub fn objc_copyClassList(outCount: *mut c_uint) -> *mut *const Class;
    pub fn objc_getClass(name: *const c_char) -> *const Class;
    pub fn objc_getProtocol(name: *const c_char) -> *const Protocol;
    pub fn objc_copyProtocolList(outCount: *mut c_uint) -> *mut *const Protocol;
    pub fn objc_allocateProtocol(name: *const c_char) -> *mut Protocol;
    pub fn objc_registerProtocol(proto: *mut Protocol);

    pub fn objc_autoreleasePoolPush() -> *mut c_void;
    pub fn objc_autoreleasePoolPop(context: *mut c_void);

    pub fn protocol_addMethodDescription(
        proto: *mut Protocol,
        name: *const Sel,
        types: *const c_char,
        isRequiredMethod: BOOL,
        isInstanceMethod: BOOL,
    );
    pub fn protocol_addProtocol(proto: *mut Protocol, addition: *const Protocol);
    pub fn protocol_getName(proto: *const Protocol) -> *const c_char;
    pub fn protocol_isEqual(proto: *const Protocol, other: *const Protocol) -> BOOL;
    pub fn protocol_copyProtocolList(
        proto: *const Protocol,
        outCount: *mut c_uint,
    ) -> *mut *const Protocol;
    pub fn protocol_conformsToProtocol(proto: *const Protocol, other: *const Protocol) -> BOOL;

    pub fn ivar_getName(ivar: *const Ivar) -> *const c_char;
    pub fn ivar_getOffset(ivar: *const Ivar) -> isize;
    pub fn ivar_getTypeEncoding(ivar: *const Ivar) -> *const c_char;

    pub fn method_getName(method: *const Method) -> *const Sel;
    pub fn method_getImplementation(method: *const Method) -> Imp;
    pub fn method_copyReturnType(method: *const Method) -> *mut c_char;
    pub fn method_copyArgumentType(method: *const Method, index: c_uint) -> *mut c_char;
    pub fn method_getNumberOfArguments(method: *const Method) -> c_uint;
    pub fn method_setImplementation(method: *mut Method, imp: Imp) -> Imp;
    pub fn method_exchangeImplementations(m1: *mut Method, m2: *mut Method);

    pub fn objc_retain(obj: *mut Object) -> *mut Object;
    pub fn objc_release(obj: *mut Object);
    pub fn objc_autorelease(obj: *mut Object);

    pub fn objc_loadWeakRetained(location: *mut *mut Object) -> *mut Object;
    pub fn objc_initWeak(location: *mut *mut Object, obj: *mut Object) -> *mut Object;
    pub fn objc_destroyWeak(location: *mut *mut Object);
    pub fn objc_copyWeak(to: *mut *mut Object, from: *mut *mut Object);
}
