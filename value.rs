use std::ffi::{CString, self};
use std::mem;
use std::str;

use objc::{encode, Encode, Id};

use {INSCopying, INSObject};

pub trait INSValue : INSObject {
    type Value: Copy + Encode;

    fn value(&self) -> Self::Value {
        assert!(self.encoding() == encode::<Self::Value>());
        unsafe {
            let mut value = mem::uninitialized::<Self::Value>();
            let _: () = msg_send![self, getValue:&mut value];
            value
        }
    }

    fn encoding(&self) -> &str {
        unsafe {
            let result: *const i8 = msg_send![self, objCType];
            let bytes = ffi::c_str_to_bytes(&result);
            let s = str::from_utf8(bytes).unwrap();
            mem::transmute(s)
        }
    }

    fn from_value(value: Self::Value) -> Id<Self> {
        let cls = <Self as INSObject>::class();
        let encoding = CString::from_slice(encode::<Self::Value>().as_bytes());
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![obj, initWithBytes:&value
                                                     objCType:encoding.as_ptr()];
            Id::from_retained_ptr(obj)
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
        let val: Id<NSValue<u32>> = INSValue::from_value(13);
        assert!(val.value() == 13);
        assert!(val.encoding() == encode::<u32>());
    }
}
