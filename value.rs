use std::ffi::CString;
use std::mem;
use std::str::from_c_str;

use objc::{encode, Encode, Id};

use {class, INSCopying, INSObject};

pub trait INSValue<T: Copy + Encode> : INSObject {
    fn value(&self) -> T {
        assert!(self.encoding() == encode::<T>());
        unsafe {
            let value = mem::uninitialized::<T>();
            msg_send![self getValue:&value];
            value
        }
    }

    fn encoding(&self) -> &str {
        unsafe {
            let result = msg_send![self objCType] as *const i8;
            from_c_str(result)
        }
    }

    fn from_value(value: &T) -> Id<Self> {
        let cls = class::<Self>();
        let encoding = CString::from_slice(encode::<T>().as_bytes());
        unsafe {
            let obj = msg_send![cls alloc];
            let obj = msg_send![obj initWithBytes:value
                                         objCType:encoding.as_ptr()];
            Id::from_retained_ptr(obj as *mut Self)
        }
    }
}

object_struct!(NSValue<T>);

impl<T: Copy + Encode> INSValue<T> for NSValue<T> { }

impl<T> INSCopying<NSValue<T>> for NSValue<T> { }

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
