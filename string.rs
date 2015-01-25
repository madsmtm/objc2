use std::ffi;
use std::mem;
use std::str;

use objc::Id;

use INSObject;

pub trait INSCopying : INSObject {
    type Output: INSObject;

    fn copy(&self) -> Id<Self::Output> {
        unsafe {
            let obj: *mut Self::Output = msg_send![self, copy];
            Id::from_retained_ptr(obj)
        }
    }
}

pub trait INSMutableCopying : INSObject {
    type Output: INSObject;

    fn mutable_copy(&self) -> Id<Self::Output> {
        unsafe {
            let obj: *mut Self::Output = msg_send![self, mutableCopy];
            Id::from_retained_ptr(obj)
        }
    }
}

pub trait INSString : INSObject {
    fn as_str(&self) -> &str {
        unsafe {
            let result: *const i8 = msg_send![self, UTF8String];
            let bytes = ffi::c_str_to_bytes(&result);
            let s = str::from_utf8(bytes).unwrap();
            mem::transmute(s)
        }
    }

    fn from_str(string: &str) -> Id<Self> {
        let cls = <Self as INSObject>::class();
        let utf8_encoding = 4u;
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithBytes:string.as_ptr()
                                                       length:string.len()
                                                     encoding:utf8_encoding];
            Id::from_retained_ptr(obj)
        }
    }
}

object_struct!(NSString);

impl INSString for NSString { }

impl INSCopying for NSString {
    type Output = NSString;
}

impl Str for NSString {
    fn as_slice(&self) -> &str {
        self.as_str()
    }
}
