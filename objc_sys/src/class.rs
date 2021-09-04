use std::os::raw::{c_char, c_int, c_uint};

use crate::{
    objc_ivar, objc_method, objc_object, objc_property_attribute_t, objc_property_t, objc_protocol,
    objc_selector, OpaqueData, BOOL, IMP,
};

/// An opaque type that represents an Objective-C class.
#[repr(C)]
pub struct objc_class {
    // `isa` field is deprecated, so we don't expose it here.
    // Use `class_getSuperclass` instead.
    _priv: [u8; 0],
    _p: OpaqueData,
}

extern "C" {
    pub fn objc_getClass(name: *const c_char) -> *const objc_class;
    pub fn objc_getRequiredClass(name: *const c_char) -> *mut objc_class;
    pub fn objc_lookUpClass(name: *const c_char) -> *mut objc_class;
    pub fn objc_getMetaClass(name: *const c_char) -> *mut objc_class;
    pub fn objc_copyClassList(outCount: *mut c_uint) -> *mut *const objc_class;
    pub fn objc_getClassList(buffer: *mut *const objc_class, bufferCount: c_int) -> c_int;

    pub fn objc_allocateClassPair(
        superclass: *mut objc_class,
        name: *const c_char,
        extraBytes: usize,
    ) -> *mut objc_class;
    pub fn objc_duplicateClass(
        original: *mut objc_class,
        name: *const c_char,
        extraBytes: usize,
    ) -> *mut objc_class;
    pub fn objc_disposeClassPair(cls: *mut objc_class);
    pub fn objc_registerClassPair(cls: *mut objc_class);

    pub fn class_addIvar(
        cls: *mut objc_class,
        name: *const c_char,
        size: usize,
        alignment: u8,
        types: *const c_char,
    ) -> BOOL;
    pub fn class_addMethod(
        cls: *mut objc_class,
        name: *const objc_selector,
        imp: IMP,
        types: *const c_char,
    ) -> BOOL;
    pub fn class_addProperty(
        cls: *mut objc_class,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributeCount: c_uint,
    ) -> BOOL;
    pub fn class_addProtocol(cls: *mut objc_class, protocol: *mut objc_protocol) -> BOOL;
    pub fn class_conformsToProtocol(cls: *mut objc_class, protocol: *mut objc_protocol) -> BOOL;
    pub fn class_copyIvarList(cls: *mut objc_class, outCount: *mut c_uint)
        -> *mut *const objc_ivar;
    pub fn class_copyMethodList(
        cls: *mut objc_class,
        outCount: *mut c_uint,
    ) -> *mut *mut objc_method;
    pub fn class_copyPropertyList(
        cls: *mut objc_class,
        outCount: *mut c_uint,
    ) -> *mut objc_property_t;
    pub fn class_copyProtocolList(
        cls: *mut objc_class,
        outCount: *mut c_uint,
    ) -> *mut *mut objc_protocol;
    pub fn class_createInstance(cls: *mut objc_class, extraBytes: usize) -> *mut objc_object;
    pub fn class_getClassMethod(
        cls: *mut objc_class,
        name: *const objc_selector,
    ) -> *mut objc_method;
    pub fn class_getClassVariable(cls: *mut objc_class, name: *const c_char) -> *const objc_ivar;
    pub fn class_getImageName(cls: *mut objc_class) -> *const c_char;
    pub fn class_getInstanceMethod(
        cls: *mut objc_class,
        name: *const objc_selector,
    ) -> *mut objc_method;
    pub fn class_getInstanceSize(cls: *mut objc_class) -> usize;
    pub fn class_getInstanceVariable(cls: *mut objc_class, name: *const c_char)
        -> *const objc_ivar;
    pub fn class_getIvarLayout(cls: *mut objc_class) -> *const u8;
    pub fn class_getName(cls: *mut objc_class) -> *const c_char;
    pub fn class_getProperty(cls: *mut objc_class, name: *const c_char) -> objc_property_t;
    pub fn class_getSuperclass(cls: *mut objc_class) -> *mut objc_class;
    pub fn class_getVersion(cls: *mut objc_class) -> c_int;
    pub fn class_getWeakIvarLayout(cls: *mut objc_class) -> *const u8;
    pub fn class_isMetaClass(cls: *mut objc_class) -> BOOL;
    pub fn class_replaceMethod(
        cls: *mut objc_class,
        name: *const objc_selector,
        imp: IMP,
        types: *const c_char,
    ) -> IMP;
    pub fn class_replaceProperty(
        cls: *mut objc_class,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributeCount: c_uint,
    );
    pub fn class_respondsToSelector(cls: *mut objc_class, sel: *const objc_selector) -> BOOL;
    pub fn class_setIvarLayout(cls: *mut objc_class, layout: *const u8);
    pub fn class_setVersion(cls: *mut objc_class, version: c_int);
    pub fn class_setWeakIvarLayout(cls: *mut objc_class, layout: *const u8);
}
