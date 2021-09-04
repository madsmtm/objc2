use std::os::raw::{c_char, c_uint};

use crate::{objc_property_attribute_t, objc_property_t};

extern "C" {
    pub fn property_copyAttributeList(
        property: objc_property_t,
        outCount: *mut c_uint,
    ) -> *mut objc_property_attribute_t;
    pub fn property_copyAttributeValue(
        property: objc_property_t,
        attributeName: *const c_char,
    ) -> *mut c_char;
    pub fn property_getAttributes(property: objc_property_t) -> *const c_char;
    pub fn property_getName(property: objc_property_t) -> *const c_char;
}
