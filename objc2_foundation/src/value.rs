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

    /// TODO.
    ///
    /// If the value was created using [`INSObject::new`], the value may not
    /// be initialized. In that case, this will return [`None`].
    fn get(&self) -> Option<Self::Value> {
        // If encoding is `None`, this was created using INSObject::new
        if let Some(encoding) = self.encoding() {
            // TODO: This can be a debug assertion (?)
            assert!(&Self::Value::ENCODING == encoding);
            Some(unsafe { self.get_unchecked() })
        } else {
            None
        }
    }

    unsafe fn get_unchecked(&self) -> Self::Value {
        let mut value = MaybeUninit::<Self::Value>::uninit();
        let ptr = value.as_mut_ptr() as *mut c_void;
        let _: () = msg_send![self, getValue: ptr];
        value.assume_init()
    }

    fn encoding(&self) -> Option<&str> {
        let result: Option<NonNull<c_char>> = unsafe { msg_send![self, objCType] };
        result.map(|s| unsafe { CStr::from_ptr(s.as_ptr()) }.to_str().unwrap())
    }

    fn from_value(value: Self::Value) -> Id<Self, Self::Ownership> {
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
    type Ownership = Shared;

    fn class() -> &'static Class {
        #[cfg(not(gnustep))]
        return class!(NSValue);

        #[cfg(gnustep)]
        // We can't use NSValue directly, because its `new` method throws an
        // exception (instead of just becoming an invalid NSValue). Luckily,
        // the `GSValue` subclass has the desired behaviour, so we can just
        // use that. Unfortunately, this is less efficient for the following:
        // ```
        // match T::ENCODING {
        //     Encoding::Object => ...,
        //     Encoding::Struct("_NSPoint", _) => ...,
        //     Encoding::Pointer(&Encoding::Void) => ...,
        //     Encoding::Struct("_NSRange", _) => ...,
        //     Encoding::Struct("_NSRect", _) => ...,
        //     Encoding::Struct("_NSSize", _) => ...,
        // }
        // ```
        //
        // See GNUStep's `NSValue +valueClassWithObjCType` and
        // `GSConcreteValueTemplate.m` for more, though we can't use the
        // classes in there either, because their `new` methods return valid
        // objects (and so `<NSValue<NSRange>>::new()` would work differently
        // on GNUStep).
        return class!(GSValue);
    }
}

unsafe impl<T: 'static + Copy + Encode> INSValue for NSValue<T> {
    type Value = T;
}

unsafe impl<T: 'static> INSCopying for NSValue<T> {
    type Output = NSValue<T>;
}

#[cfg(test)]
mod tests {
    use crate::{INSObject, INSValue, NSRange, NSValue};
    use objc2::Encode;

    #[test]
    fn test_value() {
        let val = NSValue::from_value(13u32);
        assert_eq!(val.get().unwrap(), 13);
        assert!(&u32::ENCODING == val.encoding().unwrap());
    }

    #[test]
    fn test_value_new() {
        let val = <NSValue<u8>>::new();
        assert!(val.encoding().is_none());
        assert!(val.get().is_none());
    }

    #[test]
    fn test_value_nsrange() {
        let val = NSValue::from_value(NSRange::from(1..2));
        assert!(&NSRange::ENCODING == val.encoding().unwrap());
        assert_eq!(val.get().unwrap(), NSRange::from(1..2));

        let val = <NSValue<NSRange>>::new();
        assert!(val.encoding().is_none());
        assert!(val.get().is_none());
    }
}
