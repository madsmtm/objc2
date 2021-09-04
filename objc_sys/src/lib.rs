//! # Bindings to the objc_objective-C core runtime
//!
//! # Notable differences
//!
//! Protocol / objc_protocol is no longer a type alias of objc_object, for
//! better type safety. Their internal representation is the same, so the
//! functionality is just a cast away.

#![no_std]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc-encode/1.1.0")]

extern crate std;

use core::ffi::c_void;
use std::os::raw::{c_char, c_int, c_uint};

mod constants;
mod types;

pub use constants::*;
pub use types::*;

#[link(name = "objc", kind = "dylib")]
extern "C" {
    pub fn sel_registerName(name: *const c_char) -> *const objc_selector;
    pub fn sel_getName(sel: *const objc_selector) -> *const c_char;

    pub fn class_getName(cls: *const objc_class) -> *const c_char;
    pub fn class_getSuperclass(cls: *const objc_class) -> *const objc_class;
    pub fn class_getInstanceSize(cls: *const objc_class) -> usize;
    pub fn class_getInstanceMethod(
        cls: *const objc_class,
        sel: *const objc_selector,
    ) -> *const objc_method;
    pub fn class_getInstanceVariable(
        cls: *const objc_class,
        name: *const c_char,
    ) -> *const objc_ivar;
    pub fn class_copyMethodList(
        cls: *const objc_class,
        outCount: *mut c_uint,
    ) -> *mut *const objc_method;
    pub fn class_copyIvarList(
        cls: *const objc_class,
        outCount: *mut c_uint,
    ) -> *mut *const objc_ivar;
    pub fn class_addMethod(
        cls: *mut objc_class,
        name: *const objc_selector,
        imp: IMP,
        types: *const c_char,
    ) -> BOOL;
    pub fn class_addIvar(
        cls: *mut objc_class,
        name: *const c_char,
        size: usize,
        alignment: u8,
        types: *const c_char,
    ) -> BOOL;
    pub fn class_addProtocol(cls: *mut objc_class, proto: *const objc_protocol) -> BOOL;
    pub fn class_conformsToProtocol(cls: *const objc_class, proto: *const objc_protocol) -> BOOL;
    pub fn class_copyProtocolList(
        cls: *const objc_class,
        outCount: *mut c_uint,
    ) -> *mut *const objc_protocol;

    pub fn objc_allocateClassPair(
        superclass: *const objc_class,
        name: *const c_char,
        extraBytes: usize,
    ) -> *mut objc_class;
    pub fn objc_disposeClassPair(cls: *mut objc_class);
    pub fn objc_registerClassPair(cls: *mut objc_class);

    pub fn class_createInstance(cls: *const objc_class, extraBytes: usize) -> *mut objc_object;
    pub fn object_dispose(obj: *mut objc_object) -> *mut objc_object;
    pub fn object_getClass(obj: *const objc_object) -> *const objc_class;

    pub fn objc_getClassList(buffer: *mut *const objc_class, bufferLen: c_int) -> c_int;
    pub fn objc_copyClassList(outCount: *mut c_uint) -> *mut *const objc_class;
    pub fn objc_getClass(name: *const c_char) -> *const objc_class;
    pub fn objc_getProtocol(name: *const c_char) -> *const objc_protocol;
    pub fn objc_copyProtocolList(outCount: *mut c_uint) -> *mut *const objc_protocol;
    pub fn objc_allocateProtocol(name: *const c_char) -> *mut objc_protocol;
    pub fn objc_registerProtocol(proto: *mut objc_protocol);

    pub fn objc_autoreleasePoolPush() -> *mut c_void;
    pub fn objc_autoreleasePoolPop(context: *mut c_void);

    pub fn protocol_addMethodDescription(
        proto: *mut objc_protocol,
        name: *const objc_selector,
        types: *const c_char,
        isRequiredMethod: BOOL,
        isInstanceMethod: BOOL,
    );
    pub fn protocol_addProtocol(proto: *mut objc_protocol, addition: *const objc_protocol);
    pub fn protocol_getName(proto: *const objc_protocol) -> *const c_char;
    pub fn protocol_isEqual(proto: *const objc_protocol, other: *const objc_protocol) -> BOOL;
    pub fn protocol_copyProtocolList(
        proto: *const objc_protocol,
        outCount: *mut c_uint,
    ) -> *mut *const objc_protocol;
    pub fn protocol_conformsToProtocol(
        proto: *const objc_protocol,
        other: *const objc_protocol,
    ) -> BOOL;

    pub fn ivar_getName(ivar: *const objc_ivar) -> *const c_char;
    pub fn ivar_getOffset(ivar: *const objc_ivar) -> isize;
    pub fn ivar_getTypeEncoding(ivar: *const objc_ivar) -> *const c_char;

    pub fn method_getName(method: *const objc_method) -> *const objc_selector;
    pub fn method_getImplementation(method: *const objc_method) -> IMP;
    pub fn method_copyReturnType(method: *const objc_method) -> *mut c_char;
    pub fn method_copyArgumentType(method: *const objc_method, index: c_uint) -> *mut c_char;
    pub fn method_getNumberOfArguments(method: *const objc_method) -> c_uint;
    pub fn method_setImplementation(method: *mut objc_method, imp: IMP) -> IMP;
    pub fn method_exchangeImplementations(m1: *mut objc_method, m2: *mut objc_method);

    pub fn objc_retain(obj: *mut objc_object) -> *mut objc_object;
    pub fn objc_release(obj: *mut objc_object);
    pub fn objc_autorelease(obj: *mut objc_object);

    pub fn objc_loadWeakRetained(location: *mut *mut objc_object) -> *mut objc_object;
    pub fn objc_initWeak(
        location: *mut *mut objc_object,
        obj: *mut objc_object,
    ) -> *mut objc_object;
    pub fn objc_destroyWeak(location: *mut *mut objc_object);
    pub fn objc_copyWeak(to: *mut *mut objc_object, from: *mut *mut objc_object);
}
