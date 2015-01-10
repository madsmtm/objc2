use std::ffi;
use std::mem;
use std::str;

use objc::Id;

use {class, INSObject};

pub trait INSCopying<T: INSObject> : INSObject {
    fn copy(&self) -> Id<T> {
        unsafe {
            let obj = msg_send![self, copy];
            Id::from_retained_ptr(obj as *mut T)
        }
    }
}

pub trait INSMutableCopying<T: INSObject> : INSObject {
    fn mutable_copy(&self) -> Id<T> {
        unsafe {
            let obj = msg_send![self, mutableCopy];
            Id::from_retained_ptr(obj as *mut T)
        }
    }
}

pub trait INSString : INSObject {
    fn as_str(&self) -> &str {
        unsafe {
            let result = msg_send![self, UTF8String] as *const i8;
            let bytes = ffi::c_str_to_bytes(&result);
            let s = str::from_utf8(bytes).unwrap();
            mem::transmute(s)
        }
    }

    fn from_str(string: &str) -> Id<Self> {
        let cls = class::<Self>();
        let utf8_encoding = 4u;
        unsafe {
            let obj = msg_send![cls, alloc];
            let obj = msg_send![obj, initWithBytes:string.as_ptr()
                                            length:string.len()
                                          encoding:utf8_encoding];
            Id::from_retained_ptr(obj as *mut Self)
        }
    }
}

object_struct!(NSString);

impl INSString for NSString { }

impl INSCopying<NSString> for NSString { }

impl Str for NSString {
    fn as_slice(&self) -> &str {
        self.as_str()
    }
}
