use alloc::string::ToString;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::str;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use objc2::msg_send;
use objc2::rc::{Id, Shared};
use objc2::Encode;

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
            assert!(
                Self::Value::ENCODING.equivalent_to_str(encoding),
                "Wrong encoding"
            );
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

object!(
    unsafe pub struct NSValue<T> {
        value: PhantomData<T>,
    }
);

// TODO: SAFETY
unsafe impl<T: Sync> Sync for NSValue<T> {}
unsafe impl<T: Send> Send for NSValue<T> {}

unsafe impl<T: 'static + Copy + Encode> INSValue for NSValue<T> {
    type Value = T;
}

unsafe impl<T: 'static> INSCopying for NSValue<T> {
    type Ownership = Shared;
    type Output = NSValue<T>;
}

/// ```compile_fail
/// use objc2_foundation::NSValue;
/// fn needs_eq<T: Eq + ?Sized>() {}
/// needs_eq::<NSValue<f32>>();
/// ```
#[cfg(doctest)]
pub struct NSValueFloatNotEq;

#[cfg(test)]
mod tests {
    use alloc::format;

    use crate::{INSValue, NSRange, NSValue};
    use objc2::rc::{Id, Shared};
    use objc2::Encode;

    #[test]
    fn test_value() {
        let val = NSValue::new(13u32);
        assert_eq!(val.get(), 13);
        assert!(u32::ENCODING.equivalent_to_str(val.encoding().unwrap()));
    }

    #[test]
    fn test_equality() {
        let val1 = NSValue::new(123u32);
        let val2 = NSValue::new(123u32);
        assert_eq!(val1, val1);
        assert_eq!(val1, val2);

        let val3 = NSValue::new(456u32);
        assert_ne!(val1, val3);
    }

    #[test]
    fn test_equality_across_types() {
        let val1 = NSValue::new(123);
        let val2: Id<NSValue<u32>, Shared> = NSValue::new(123);
        let val2: Id<NSValue<u8>, Shared> = unsafe { core::mem::transmute(val2) };

        // Test that `objCType` is checked when comparing equality
        assert_ne!(val1, val2);
    }

    #[test]
    #[ignore = "Output is different depending on OS version and runtime"]
    fn test_debug() {
        fn assert_debug(val: impl core::fmt::Debug, expected: &str) {
            assert_eq!(format!("{:?}", val), format!("{:?}", expected));
        }
        let val = NSValue::new(0xabu8);
        let expected = "<ab>";
        assert_debug(val, expected);

        let val = NSValue::new(0x12i8);
        let expected = "<12>";
        assert_debug(val, expected);

        let val = NSValue::new(0xdeadbeefu32);
        let expected = "<efbeadde>";
        assert_debug(val, expected);

        // Very brittle
        let val = NSValue::new(1.0f32);
        let expected = &format!(
            "<{:08x}>",
            u32::from_le_bytes(1.0f32.to_le_bytes()).reverse_bits()
        );
        assert_debug(val, expected);
    }

    #[test]
    fn test_value_nsrange() {
        let val = NSValue::new(NSRange::from(1..2));
        assert!(NSRange::ENCODING.equivalent_to_str(val.encoding().unwrap()));
        let range: NSRange = unsafe { objc2::msg_send![val, rangeValue] };
        assert_eq!(range, NSRange::from(1..2));
        // NSValue -getValue is broken on GNUStep for some types
        #[cfg(not(gnustep))]
        assert_eq!(val.get(), NSRange::from(1..2));
    }
}
