use core::mem::MaybeUninit;

use crate::encode::{Encode, Encoding};

use super::InnerIvarType;

/// Ivar types that are [`Encode`].
//
// Note: We put the inner type in a `MaybeUninit`, since we may need to access
// this type before the inner type has been properly initialized.
#[repr(transparent)]
pub struct IvarEncode<T>(MaybeUninit<T>);

// We intentionally don't implement `Drop`, since that may happen before the
// ivar has been initialized.
//
// For example in the case of `NonNull<u8>`, it would be zero-initialized,
// which is an invalid state for that to have.

// SAFETY: `IvarEncode<T>` is `#[repr(transparent)]`, and the layout of
// `MaybeUninit<T>` is the same as `T`.
unsafe impl<T: Encode> Encode for IvarEncode<T> {
    const ENCODING: Encoding = T::ENCODING;
}

impl<T: Encode> super::ivar::private::Sealed for IvarEncode<T> {}

// SAFETY: `IvarEncode<T>` has the same memory layout as T, and
// `MaybeUninit<T>` is safe to zero-initialize.
unsafe impl<T: Encode> InnerIvarType for IvarEncode<T> {
    type Output = T;

    #[inline]
    unsafe fn __deref(&self) -> &Self::Output {
        // SAFETY: Checked by caller
        unsafe { self.0.assume_init_ref() }
    }

    #[inline]
    unsafe fn __deref_mut(&mut self) -> &mut Self::Output {
        // SAFETY: Checked by caller
        unsafe { self.0.assume_init_mut() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::mem;

    #[test]
    fn needs_drop() {
        assert!(!mem::needs_drop::<IvarEncode<i32>>());
        assert!(!mem::needs_drop::<IvarEncode<&i32>>());

        // You wouldn't do this, but let's make sure it works as expected
        #[repr(transparent)]
        struct DropAndEncode(i32);

        unsafe impl Encode for DropAndEncode {
            const ENCODING: Encoding = i32::ENCODING;
        }

        impl Drop for DropAndEncode {
            fn drop(&mut self) {}
        }

        assert!(mem::needs_drop::<DropAndEncode>());
        assert!(!mem::needs_drop::<IvarEncode<DropAndEncode>>());

        assert_eq!(mem::size_of::<IvarEncode<i32>>(), mem::size_of::<i32>());
    }
}
