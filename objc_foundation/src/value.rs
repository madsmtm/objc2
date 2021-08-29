use std::any::Any;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_void};
use std::str;

use objc::runtime::Class;
use objc::Encode;
use objc_id::Id;

use super::{INSCopying, INSObject};

pub trait INSValue: INSObject {
    type Value: 'static + Copy + Encode;

    fn value(&self) -> Self::Value {
        assert!(&Self::Value::ENCODING == self.encoding());
        let mut value = MaybeUninit::<Self::Value>::uninit();
        let ptr = value.as_mut_ptr() as *mut c_void;
        unsafe {
            let _: () = msg_send![self, getValue: ptr];
            value.assume_init()
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
        let value_ptr: *const Self::Value = &value;
        let bytes = value_ptr as *const c_void;
        let encoding = CString::new(Self::Value::ENCODING.to_string()).unwrap();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithBytes:bytes
                                                     objCType:encoding.as_ptr()];
            Id::from_retained_ptr(obj)
        }
    }
}

pub struct NSValue<T> {
    value: PhantomData<T>,
}

object_impl!(NSValue<T>);

impl<T> INSObject for NSValue<T>
where
    T: Any,
{
    fn class() -> &'static Class {
        class!(NSValue)
    }
}

impl<T> INSValue for NSValue<T>
where
    T: Any + Copy + Encode,
{
    type Value = T;
}

impl<T> INSCopying for NSValue<T>
where
    T: Any,
{
    type Output = NSValue<T>;
}

#[cfg(test)]
mod tests {
    use crate::{INSValue, NSValue};
    use objc::Encode;

    #[test]
    fn test_value() {
        let val = NSValue::from_value(13u32);
        assert!(val.value() == 13);
        assert!(&u32::ENCODING == val.encoding());
    }
}
