#![allow(non_camel_case_types)]
use std::os::raw::{c_char, c_int, c_uint};

#[cfg(any(doc, not(feature = "unstable-objfw")))]
use crate::ffi::{objc_ivar, objc_method, objc_property, objc_property_attribute_t};
use crate::ffi::{objc_protocol, objc_selector, BOOL};
use crate::runtime::{AnyClass, Imp};

#[cfg(any(doc, not(feature = "unstable-objfw")))]
/// This is `c_char` in GNUStep's libobjc2 and `uint8_t` in Apple's objc4.
///
/// The pointer represents opaque data, and is definitely not just an integer,
/// so its signedness (i8 vs. u8) is not applicable.
///
/// So we assign it here as a private alias to u8, to not document the
/// difference.
type ivar_layout_type = u8;

// May call `resolveClassMethod:` or `resolveInstanceMethod:`.
extern_c_unwind! {
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn class_getClassMethod(
        cls: *const AnyClass,
        name: *const objc_selector,
    ) -> *const objc_method;
    #[cfg(any(doc, not(feature = "unstable-objfw")))] // Available in newer versions
    pub fn class_getInstanceMethod(
        cls: *const AnyClass,
        name: *const objc_selector,
    ) -> *const objc_method;

    pub fn class_respondsToSelector(cls: *const AnyClass, sel: *const objc_selector) -> BOOL;

    // #[deprecated = "use class_getMethodImplementation instead"]
    // #[cfg(any(doc, target_vendor = "apple"))]
    // pub fn class_lookupMethod
    // #[deprecated = "use class_respondsToSelector instead"]
    // #[cfg(any(doc, target_vendor = "apple"))]
    // pub fn class_respondsToMethod
}

// TODO: Hooks registered with objc_setHook_getClass may be allowed to unwind?
extern_c! {
    pub fn objc_getClass(name: *const c_char) -> *const AnyClass;
    pub fn objc_getRequiredClass(name: *const c_char) -> *const AnyClass;
    pub fn objc_lookUpClass(name: *const c_char) -> *const AnyClass;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn objc_getMetaClass(name: *const c_char) -> *const AnyClass;
    /// The returned array is deallocated with [`free`][crate::ffi::free].
    pub fn objc_copyClassList(out_len: *mut c_uint) -> *mut *const AnyClass;
    pub fn objc_getClassList(buffer: *mut *const AnyClass, buffer_len: c_int) -> c_int;

    pub fn objc_allocateClassPair(
        superclass: *const AnyClass,
        name: *const c_char,
        extra_bytes: usize,
    ) -> *mut AnyClass;
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn objc_duplicateClass(
        original: *const AnyClass,
        name: *const c_char,
        extra_bytes: usize,
    ) -> *mut AnyClass;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn objc_disposeClassPair(cls: *mut AnyClass);
    pub fn objc_registerClassPair(cls: *mut AnyClass);

    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn class_addIvar(
        cls: *mut AnyClass,
        name: *const c_char,
        size: usize,
        alignment: u8,
        types: *const c_char,
    ) -> BOOL;
    pub fn class_addMethod(
        cls: *mut AnyClass,
        name: *const objc_selector,
        imp: Imp,
        types: *const c_char,
    ) -> BOOL;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn class_addProperty(
        cls: *mut AnyClass,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributes_count: c_uint,
    ) -> BOOL;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn class_addProtocol(cls: *mut AnyClass, protocol: *const objc_protocol) -> BOOL;
    pub fn class_conformsToProtocol(cls: *const AnyClass, protocol: *const objc_protocol)
        -> BOOL;

    #[cfg(any(doc, not(feature = "unstable-objfw")))] // Available in newer versions
    /// The return value is deallocated with [`free`][crate::ffi::free].
    pub fn class_copyIvarList(
        cls: *const AnyClass,
        out_len: *mut c_uint,
    ) -> *mut *const objc_ivar;
    #[cfg(any(doc, not(feature = "unstable-objfw")))] // Available in newer versions
    /// The returned array is deallocated with [`free`][crate::ffi::free].
    pub fn class_copyMethodList(
        cls: *const AnyClass,
        out_len: *mut c_uint,
    ) -> *mut *const objc_method;
    #[cfg(any(doc, not(feature = "unstable-objfw")))] // Available in newer versions
    /// The returned array is deallocated with [`free`][crate::ffi::free].
    pub fn class_copyPropertyList(
        cls: *const AnyClass,
        out_len: *mut c_uint,
    ) -> *mut *const objc_property;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    /// The returned array is deallocated with [`free`][crate::ffi::free].
    pub fn class_copyProtocolList(
        cls: *const AnyClass,
        out_len: *mut c_uint,
    ) -> *mut *const objc_protocol;

    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn class_createInstance(cls: *const AnyClass, extra_bytes: usize) -> *mut crate::runtime::AnyObject;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn class_getClassVariable(cls: *const AnyClass, name: *const c_char) -> *const objc_ivar;
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn class_getImageName(cls: *const AnyClass) -> *const c_char;
    pub fn class_getInstanceSize(cls: *const AnyClass) -> usize;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn class_getInstanceVariable(
        cls: *const AnyClass,
        name: *const c_char,
    ) -> *const objc_ivar;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn class_getIvarLayout(cls: *const AnyClass) -> *const ivar_layout_type;
    pub fn class_getName(cls: *const AnyClass) -> *const c_char;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn class_getProperty(cls: *const AnyClass, name: *const c_char) -> *const objc_property;
    pub fn class_getSuperclass(cls: *const AnyClass) -> *const AnyClass;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn class_getVersion(cls: *const AnyClass) -> c_int;
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn class_getWeakIvarLayout(cls: *const AnyClass) -> *const ivar_layout_type;
    pub fn class_isMetaClass(cls: *const AnyClass) -> BOOL;
    pub fn class_replaceMethod(
        cls: *mut AnyClass,
        name: *const objc_selector,
        imp: Imp,
        types: *const c_char,
    ) -> Option<Imp>;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn class_replaceProperty(
        cls: *mut AnyClass,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributes_len: c_uint,
    );
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn class_setIvarLayout(cls: *mut AnyClass, layout: *const ivar_layout_type);
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn class_setVersion(cls: *mut AnyClass, version: c_int);
    #[cfg(any(doc, target_vendor = "apple"))]
    pub fn class_setWeakIvarLayout(cls: *mut AnyClass, layout: *const ivar_layout_type);

    // #[deprecated = "not recommended"]
    // pub fn class_setSuperclass
}
