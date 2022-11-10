use core::marker::PhantomData;
use core::mem;
use core::ptr::NonNull;

use crate::encode::{Encode, Encoding};

use super::InnerIvarType;

/// TODO
pub struct PrivateIvar<T> {
    /// For proper variance and auto traits.
    p: PhantomData<T>,
}

impl<T: Sized> super::ivar::private::Sealed for PrivateIvar<T> {}

// SAFETY: TODO
unsafe impl<T: Sized> InnerIvarType for PrivateIvar<T> {
    const __ENCODING: Encoding = Encoding::Struct(
        "__objc2_private",
        &[Encoding::Array(
            {
                let size = mem::size_of::<T>();
                let align = mem::align_of::<T>();
                if size % align != 0 {
                    panic!("size was not a multiple of alignment in `PrivateIvar`")
                }
                // When calculating size from an encoding, the size will be
                // multiplied with the alignment of the type below.
                size / align
            },
            // Output an encoding with the given alignment so that code that
            // uses the encoding to determine alignment can still do so.
            {
                const INNER: Encoding = match mem::align_of::<T>() {
                    1 => i8::ENCODING,
                    2 => i16::ENCODING,
                    4 => i32::ENCODING,
                    8 => i64::ENCODING,
                    _ => panic!("expected alignment to be one of 1, 2, 4 and 8 in `PrivateIvar`"),
                };
                &INNER
            },
        )],
    );

    type __Inner = T;
    type Output = T;

    const __MAY_DROP: bool = true;

    #[inline]
    unsafe fn __to_ref(this: &Self::__Inner) -> &Self::Output {
        this
    }

    #[inline]
    unsafe fn __to_mut(this: &mut Self::__Inner) -> &mut Self::Output {
        this
    }

    #[inline]
    fn __to_ptr(inner: NonNull<Self::__Inner>) -> NonNull<Self::Output> {
        inner.cast()
    }
}
