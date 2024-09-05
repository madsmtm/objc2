use std::os::raw::c_char;
#[cfg(any(doc, not(feature = "unstable-objfw")))]
use std::os::raw::c_uint;

#[cfg(any(doc, not(feature = "unstable-objfw")))]
use crate::ffi::{objc_method_description, objc_property, objc_property_attribute_t};
use crate::{
    ffi::BOOL,
    runtime::{AnyProtocol, Sel},
};

extern_c! {
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn objc_getProtocol(name: *const c_char) -> *const AnyProtocol;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    /// The returned array is deallocated with [`free`][crate::ffi::free].
    pub fn objc_copyProtocolList(out_len: *mut c_uint) -> *mut *const AnyProtocol;

    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn objc_allocateProtocol(name: *const c_char) -> *mut AnyProtocol;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn objc_registerProtocol(proto: *mut AnyProtocol);

    pub fn protocol_conformsToProtocol(
        proto: *const AnyProtocol,
        other: *const AnyProtocol,
    ) -> BOOL;
    pub fn protocol_isEqual(proto: *const AnyProtocol, other: *const AnyProtocol) -> BOOL;
    pub fn protocol_getName(proto: *const AnyProtocol) -> *const c_char;

    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn protocol_addMethodDescription(
        proto: *mut AnyProtocol,
        name: Sel,
        types: *const c_char,
        is_required_method: BOOL,
        is_instance_method: BOOL,
    );
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn protocol_addProperty(
        proto: *mut AnyProtocol,
        name: *const c_char,
        attributes: *const objc_property_attribute_t,
        attributes_len: c_uint,
        is_required_property: BOOL,
        is_instance_property: BOOL,
    );
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn protocol_addProtocol(proto: *mut AnyProtocol, addition: *const AnyProtocol);
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    /// The returned array is deallocated with [`free`][crate::ffi::free].
    pub fn protocol_copyMethodDescriptionList(
        proto: *const AnyProtocol,
        is_required_method: BOOL,
        is_instance_method: BOOL,
        out_len: *mut c_uint,
    ) -> *mut objc_method_description;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    /// The returned array is deallocated with [`free`][crate::ffi::free].
    pub fn protocol_copyPropertyList(
        proto: *const AnyProtocol,
        out_len: *mut c_uint,
    ) -> *mut *const objc_property;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    /// The returned array is deallocated with [`free`][crate::ffi::free].
    pub fn protocol_copyProtocolList(
        proto: *const AnyProtocol,
        out_len: *mut c_uint,
    ) -> *mut *const AnyProtocol;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn protocol_getMethodDescription(
        proto: *const AnyProtocol,
        sel: Sel,
        is_required_method: BOOL,
        is_instance_method: BOOL,
    ) -> objc_method_description;
    #[cfg(any(doc, not(feature = "unstable-objfw")))]
    pub fn protocol_getProperty(
        proto: *const AnyProtocol,
        name: *const c_char,
        is_required_property: BOOL,
        is_instance_property: BOOL,
    ) -> *const objc_property;

    // #[cfg(any(doc, macos >= 10.12))]
    // protocol_copyPropertyList2

    // #[cfg(any(doc, feature = "gnustep-1-7"))]
    // _protocol_getMethodTypeEncoding
}
