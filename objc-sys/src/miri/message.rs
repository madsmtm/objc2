use core::mem;

use crate::{objc_class, objc_object, objc_selector};

pub fn custom_msg_send<T, R>(obj: *mut objc_object, sel: *const objc_selector, args: T) -> R {
    if obj.is_null() {
        unsafe { mem::zeroed() }
    }
    if sel.is_null() {
        panic!("Null selector")
    }
    todo!()
}

pub fn custom_msg_send_super<T, R>(
    obj: *mut objc_object,
    superclass: *const objc_class,
    sel: *const objc_selector,
    args: T,
) -> R {
    if obj.is_null() {
        unsafe { mem::zeroed() }
    }
    if superclass.is_null() {
        panic!("Nil superclass")
    }
    if sel.is_null() {
        panic!("Null selector")
    }
    todo!()
}
