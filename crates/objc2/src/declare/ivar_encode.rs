use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::encode::{Encode, Encoding};

use super::InnerIvarType;

/// Ivar types that are [`Encode`].
pub struct IvarEncode<T>(PhantomData<T>);

impl<T: Encode> super::ivar::private::Sealed for IvarEncode<T> {}

unsafe impl<T: Encode> InnerIvarType for IvarEncode<T> {
    const __IVAR_ENCODING: Encoding = T::ENCODING;
    type __Inner = T;
    type Output = T;
    // Note: We explicitly tell `Ivar` that it shouldn't do anything to drop,
    // since if the object was deallocated before an `init` method was called,
    // the ivar would not have been initialized properly!
    //
    // For example in the case of `NonNull<u8>`, it would be zero-initialized
    // which is an invalid state for that.
    const __MAY_DROP: bool = false;

    #[inline]
    unsafe fn __to_ref(inner: &Self::__Inner) -> &Self::Output {
        inner
    }

    #[inline]
    unsafe fn __to_mut(inner: &mut Self::__Inner) -> &mut Self::Output {
        inner
    }

    #[inline]
    fn __to_ptr(inner: NonNull<Self::__Inner>) -> NonNull<Self::Output> {
        inner
    }
}
