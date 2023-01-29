use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::encode::Encoding;

use super::InnerIvarType;

/// Ivar of [`bool`].
///
/// This is used to work around the fact that `bool` is not [`Encode`], only
/// [`EncodeConvert`] (and that is only usable when we can do a conversion
/// step, which we can't here).
///
/// If you want to access this instance variable to Objective-C, you must do
/// so using C99 `_Bool`; if you want to use `BOOL` in Objective-C, you should
/// use `IvarEncode<Bool>`.
///
/// [`Encode`]: crate::encode::Encode
/// [`EncodeConvert`]: crate::encode::EncodeConvert
pub struct IvarBool(PhantomData<bool>);

impl super::ivar::private::Sealed for IvarBool {}

unsafe impl InnerIvarType for IvarBool {
    const __IVAR_ENCODING: Encoding = Encoding::Bool;
    type __Inner = bool;
    type Output = bool;
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
