use std::ffi::{CString, self};
use std::mem;
use std::str;

use objc::{encode, Encode, Id};

use {class, INSCopying, INSObject};

pub trait INSValue : INSObject {
    type Value: Copy + Encode;

    fn value(&self) -> Self::Value {
        assert!(self.encoding() == encode::<Self::Value>());
        unsafe {
            let value = mem::uninitialized::<Self::Value>();
            msg_send![self, getValue:&value];
            value
        }
    }

    fn encoding(&self) -> &str {
        unsafe {
            let result = msg_send![self, objCType] as *const i8;
            let bytes = ffi::c_str_to_bytes(&result);
            let s = str::from_utf8(bytes).unwrap();
            mem::transmute(s)
        }
    }

    fn from_value(value: &Self::Value) -> Id<Self> {
        let cls = class::<Self>();
        let encoding = CString::from_slice(encode::<Self::Value>().as_bytes());
        unsafe {
            let obj = msg_send![cls, alloc];
            let obj = msg_send![obj, initWithBytes:value
                                          objCType:encoding.as_ptr()];
            Id::from_retained_ptr(obj as *mut Self)
        }
    }
}

object_struct!(NSValue<T>);

impl<T: Copy + Encode> INSValue for NSValue<T> {
    type Value = T;
}

impl<T> INSCopying for NSValue<T> {
    type Output = NSValue<T>;
}

#[cfg(test)]
mod tests {
    use objc::{encode, Id};
    use {INSValue, NSValue};

    #[test]
    fn test_value() {
        let val: Id<NSValue<uint>> = INSValue::from_value(&13);
        assert!(val.value() == 13);
        assert!(val.encoding() == encode::<uint>());
    }
}
