use alloc::string::ToString;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::str;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use objc2::rc::{Id, Shared};
use objc2::runtime::Class;
use objc2::Encode;
use objc2::{class, msg_send};

use super::{INSCopying, INSObject};

pub unsafe trait INSValue: INSObject {
    type Value: 'static + Copy + Encode;

    // Default / empty new is not provided because `-init` returns `nil` on
    // Apple and GNUStep throws an exception on all other messages to this
    // invalid instance.

    /// TODO.
    ///
    /// Note that this is broken on GNUStep for some types, see
    /// [gnustep/libs-base#216].
    ///
    /// [gnustep/libs-base#216]: https://github.com/gnustep/libs-base/pull/216
    fn get(&self) -> Self::Value {
        if let Some(encoding) = self.encoding() {
            // TODO: This can be a debug assertion (?)
            assert!(&Self::Value::ENCODING == encoding, "Wrong encoding");
            unsafe { self.get_unchecked() }
        } else {
            panic!("Missing NSValue encoding");
        }
    }

    /// TODO
    ///
    /// # Safety
    ///
    /// The user must ensure that the inner value is properly initialized.
    unsafe fn get_unchecked(&self) -> Self::Value {
        let mut value = MaybeUninit::<Self::Value>::uninit();
        let ptr = value.as_mut_ptr() as *mut c_void;
        let _: () = unsafe { msg_send![self, getValue: ptr] };
        unsafe { value.assume_init() }
    }

    fn encoding(&self) -> Option<&str> {
        let result: Option<NonNull<c_char>> = unsafe { msg_send![self, objCType] };
        result.map(|s| unsafe { CStr::from_ptr(s.as_ptr()) }.to_str().unwrap())
    }

    fn new(value: Self::Value) -> Id<Self, Shared> {
        let cls = Self::class();
        let bytes = &value as *const Self::Value as *const c_void;
        let encoding = CString::new(Self::Value::ENCODING.to_string()).unwrap();
        unsafe {
            let obj: *mut Self = msg_send![cls, alloc];
            let obj: *mut Self = msg_send![
                obj,
                initWithBytes: bytes,
                objCType: encoding.as_ptr(),
            ];
            Id::new(NonNull::new_unchecked(obj))
        }
    }
}

pub struct NSValue<T> {
    value: PhantomData<T>,
}

object_impl!(unsafe NSValue<T>);

unsafe impl<T: 'static> INSObject for NSValue<T> {
    fn class() -> &'static Class {
        class!(NSValue)
    }
}

unsafe impl<T: 'static + Copy + Encode> INSValue for NSValue<T> {
    type Value = T;
}

unsafe impl<T: 'static> INSCopying for NSValue<T> {
    type Ownership = Shared;
    type Output = NSValue<T>;
}

#[cfg(test)]
mod tests {
    use crate::{INSValue, NSRange, NSValue};
    use objc2::Encode;

    #[test]
    fn test_value() {
        let val = NSValue::new(13u32);
        assert_eq!(val.get(), 13);
        assert!(&u32::ENCODING == val.encoding().unwrap());
    }

    #[test]
    fn test_value_nsrange() {
        let val = NSValue::new(NSRange::from(1..2));
        assert!(&NSRange::ENCODING == val.encoding().unwrap());
        let range: NSRange = unsafe { objc2::msg_send![val, rangeValue] };
        assert_eq!(range, NSRange::from(1..2));
        // NSValue -getValue is broken on GNUStep for some types
        #[cfg(not(gnustep))]
        assert_eq!(val.get(), NSRange::from(1..2));
    }
}
