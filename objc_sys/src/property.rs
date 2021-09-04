use std::os::raw::{c_char, c_uint};

use crate::OpaqueData;

#[repr(C)]
pub struct objc_property {
    _priv: [u8; 0],
    _p: OpaqueData,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_property_attribute_t {
    pub name: *const c_char,
    pub value: *const c_char,
}

extern "C" {
    pub fn property_copyAttributeList(
        property: *const objc_property,
        out_len: *mut c_uint,
    ) -> *mut objc_property_attribute_t;
    pub fn property_copyAttributeValue(
        property: *const objc_property,
        attribute_name: *const c_char,
    ) -> *mut c_char;
    pub fn property_getAttributes(property: *const objc_property) -> *const c_char;
    pub fn property_getName(property: *const objc_property) -> *const c_char;
}
