use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem;
use std::str;
use libc::c_char;

use objc::{Encode, Id};
use objc::runtime::Class;

use {INSCopying, INSObject};

pub trait INSValue : INSObject {
    type Value: 'static + Copy + Encode;

    fn value(&self) -> Self::Value {
        assert!(Self::Value::encode() == self.encoding());
        unsafe {
            let mut value = mem::uninitialized::<Self::Value>();
            let _: () = msg_send![self, getValue:&mut value];
            value
        }
    }

    fn encoding(&self) -> &str {
        unsafe {
            let result: *const c_char = msg_send![self, objCType];
            let s = CStr::from_ptr(result);
            str::from_utf8(s.to_bytes()).unwrap()
        }
    }

    fn from_value(value: Self::Value) -> Id<Self> {
        let cls = Self::class();
        let encoding = Self::Value::encode();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithBytes:&value
                                                     objCType:encoding.as_ptr()];
            Id::from_retained_ptr(obj)
        }
    }
}

pub struct NSValue<T> {
    value: PhantomData<T>,
}

object_impl!(NSValue<T>);

impl<T> INSObject for NSValue<T> where T: 'static {
    fn class() -> &'static Class {
        Class::get("NSValue").unwrap()
    }
}

impl<T> INSValue for NSValue<T> where T: 'static + Copy + Encode {
    type Value = T;
}

impl<T> INSCopying for NSValue<T> where T: 'static {
    type Output = NSValue<T>;
}

#[cfg(test)]
mod tests {
    use objc::Encode;
    use {INSValue, NSValue};

    #[test]
    fn test_value() {
        let val = NSValue::from_value(13u32);
        assert!(val.value() == 13);
        assert!(u32::encode() == val.encoding());
    }
}
