use alloc::string::ToString;
use core::cmp::Ordering;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::NonNull;
use core::{fmt, str};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use objc2::rc::{DefaultId, Id, Shared};
use objc2::Encode;
use objc2::{msg_send, msg_send_id};

use super::{NSCopying, NSObject};
use crate::__inner_extern_class;

__inner_extern_class! {
    // `T: Eq` bound to prevent `NSValue<f32>` from being `Eq`
    // (even though `[NAN isEqual: NAN]` is true in Objective-C).
    #[derive(PartialEq, Eq, Hash)]
    unsafe pub struct NSValue<T>: NSObject {
        value: PhantomData<T>,
    }
}

// SAFETY: `NSValue<T>` is basically just a wrapper around an inner type, and
// is immutable.
unsafe impl<T: Sync> Sync for NSValue<T> {}
unsafe impl<T: Send> Send for NSValue<T> {}

impl<T: UnwindSafe> UnwindSafe for NSValue<T> {}
impl<T: RefUnwindSafe> RefUnwindSafe for NSValue<T> {}

impl<T: 'static + Copy + Encode> NSValue<T> {
    // Default / empty new is not provided because `-init` returns `nil` on
    // Apple and GNUStep throws an exception on all other messages to this
    // invalid instance.

    /// TODO.
    ///
    /// Note that this is broken on GNUStep for some types, see
    /// [gnustep/libs-base#216].
    ///
    /// [gnustep/libs-base#216]: https://github.com/gnustep/libs-base/pull/216
    pub fn get(&self) -> T {
        if let Some(encoding) = self.encoding() {
            // TODO: This can be a debug assertion (?)
            assert!(T::ENCODING.equivalent_to_str(encoding), "Wrong encoding");
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
    pub unsafe fn get_unchecked(&self) -> T {
        let mut value = MaybeUninit::<T>::uninit();
        let ptr: *mut c_void = value.as_mut_ptr().cast();
        let _: () = unsafe { msg_send![self, getValue: ptr] };
        unsafe { value.assume_init() }
    }

    pub fn encoding(&self) -> Option<&str> {
        let result: Option<NonNull<c_char>> = unsafe { msg_send![self, objCType] };
        result.map(|s| unsafe { CStr::from_ptr(s.as_ptr()) }.to_str().unwrap())
    }

    pub fn new(value: T) -> Id<Self, Shared> {
        let cls = Self::class();
        let bytes: *const T = &value;
        let bytes: *const c_void = bytes.cast();
        let encoding = CString::new(T::ENCODING.to_string()).unwrap();
        unsafe {
            let obj = msg_send_id![cls, alloc];
            msg_send_id![
                obj,
                initWithBytes: bytes,
                objCType: encoding.as_ptr(),
            ]
            .unwrap()
        }
    }
}

unsafe impl<T: 'static> NSCopying for NSValue<T> {
    type Ownership = Shared;
    type Output = NSValue<T>;
}

impl<T: 'static> alloc::borrow::ToOwned for NSValue<T> {
    type Owned = Id<NSValue<T>, Shared>;
    fn to_owned(&self) -> Self::Owned {
        self.copy()
    }
}

impl<T: 'static + Copy + Encode + Ord> Ord for NSValue<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(&other.get())
    }
}

impl<T: 'static + Copy + Encode + PartialOrd> PartialOrd for NSValue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }
}

impl<T: 'static + Copy + Encode + Default> DefaultId for NSValue<T> {
    type Ownership = Shared;

    #[inline]
    fn default_id() -> Id<Self, Self::Ownership> {
        Self::new(Default::default())
    }
}

impl<T: 'static + Copy + Encode + fmt::Display> fmt::Display for NSValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.get(), f)
    }
}

impl<T: 'static + Copy + Encode + fmt::Debug> fmt::Debug for NSValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.get(), f)
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use super::*;
    use crate::NSRange;

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
    fn test_display_debug() {
        fn assert_display_debug<T: fmt::Debug + fmt::Display>(val: T, expected: &str) {
            // The two impls for these happen to be the same
            assert_eq!(format!("{}", val), expected);
            assert_eq!(format!("{:?}", val), expected);
        }
        assert_display_debug(NSValue::new(171u8), "171");
        assert_display_debug(NSValue::new(-12i8), "-12");
        assert_display_debug(NSValue::new(0xdeadbeefu32), "3735928559");
        assert_display_debug(NSValue::new(1.1f32), "1.1");
        assert_display_debug(NSValue::new(true), "true");
        assert_display_debug(NSValue::new(false), "false");

        let val = NSValue::new(1.0f32);
        assert_eq!(format!("{}", val), "1");
        assert_eq!(format!("{:?}", val), "1.0");
    }

    #[test]
    fn test_value_nsrange() {
        let val = NSValue::new(NSRange::from(1..2));
        assert!(NSRange::ENCODING.equivalent_to_str(val.encoding().unwrap()));
        let range: NSRange = unsafe { objc2::msg_send![&val, rangeValue] };
        assert_eq!(range, NSRange::from(1..2));
        // NSValue -getValue is broken on GNUStep for some types
        #[cfg(not(feature = "gnustep-1-7"))]
        assert_eq!(val.get(), NSRange::from(1..2));
    }
}
