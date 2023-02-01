//! # Unstable encoding traits.
//!
//! The traits in this module are unstable, meaning that they are **outside
//! the usual SemVer guarantee**, and may freely change or be removed in patch
//! versions.
//!
//! They are provided here mostly for documentation purposes (e.g. you can
//! check [`EncodeReturn`] to see which types are possible as the return value
//! of a function pointer).
//!
//! If you would like to use some of these, please [open an issue], then we
//! can discuss making them stable.
//!
//! [open an issue]: https://github.com/madsmtm/objc2/issues/new
#![cfg_attr(
    feature = "unstable-docsrs",
    doc(cfg(feature = "unstable-encode-internals"))
)]

use crate::encode::{Encode, Encoding};
use crate::runtime::Bool;

mod return_private {
    pub trait Sealed {}
}

/// Types that are safe as the return value from Objective-C.
///
/// We currently don't need a similar `EncodeArgument` trait, but we might in
/// the future.
pub unsafe trait EncodeReturn: return_private::Sealed {
    /// The Objective-C type-encoding for this type.
    const ENCODING_RETURN: Encoding;
}

impl return_private::Sealed for () {}
unsafe impl EncodeReturn for () {
    const ENCODING_RETURN: Encoding = Encoding::Void;
}

impl<T: Encode> return_private::Sealed for T {}
unsafe impl<T: Encode> EncodeReturn for T {
    const ENCODING_RETURN: Encoding = T::ENCODING;
}

mod convert_private {
    pub trait Sealed {}
}

impl<T: EncodeReturn> convert_private::Sealed for T {}
impl convert_private::Sealed for bool {}

/// Represents types that can easily be converted to/from an [`Encode`] type.
///
/// This is implemented specially for [`bool`] to allow using that as
/// Objective-C `BOOL`, where it would otherwise not be allowed (since they
/// are not ABI compatible).
pub trait EncodeConvertArgument: convert_private::Sealed {
    /// The inner type that this can be converted to and from.
    #[doc(hidden)]
    type __Inner: Encode;

    #[doc(hidden)]
    fn __from_inner(inner: Self::__Inner) -> Self;

    #[doc(hidden)]
    fn __into_inner(self) -> Self::__Inner;
}

impl<T: Encode> EncodeConvertArgument for T {
    type __Inner = Self;

    #[inline]
    fn __from_inner(inner: Self::__Inner) -> Self {
        inner
    }

    #[inline]
    fn __into_inner(self) -> Self::__Inner {
        self
    }
}

impl EncodeConvertArgument for bool {
    type __Inner = Bool;

    #[inline]
    fn __from_inner(inner: Self::__Inner) -> Self {
        inner.as_bool()
    }

    #[inline]
    fn __into_inner(self) -> Self::__Inner {
        Bool::new(self)
    }
}

/// Same as [`EncodeConvertArgument`], but for return types.
pub trait EncodeConvertReturn: convert_private::Sealed {
    /// The inner type that this can be converted to and from.
    #[doc(hidden)]
    type __Inner: EncodeReturn;

    #[doc(hidden)]
    fn __from_inner(inner: Self::__Inner) -> Self;

    #[doc(hidden)]
    fn __into_inner(self) -> Self::__Inner;
}

impl<T: EncodeReturn> EncodeConvertReturn for T {
    type __Inner = Self;

    #[inline]
    fn __from_inner(inner: Self::__Inner) -> Self {
        inner
    }

    #[inline]
    fn __into_inner(self) -> Self::__Inner {
        self
    }
}

impl EncodeConvertReturn for bool {
    type __Inner = Bool;

    #[inline]
    fn __from_inner(inner: Self::__Inner) -> Self {
        inner.as_bool()
    }

    #[inline]
    fn __into_inner(self) -> Self::__Inner {
        Bool::new(self)
    }
}

mod args_private {
    pub trait Sealed {}
}

/// Types that represent an ordered group of function arguments, where each
/// argument has an Objective-C type-encoding, or can be converted from one.
///
/// This is implemented for tuples of up to 16 arguments, where each argument
/// implements [`EncodeConvertArgument`]. It is primarily used to make generic
/// code a bit easier.
///
/// Note that tuples themselves don't implement [`Encode`] directly, because
/// they're not FFI-safe!
pub unsafe trait EncodeArguments: args_private::Sealed {
    /// The encodings for the arguments.
    const ENCODINGS: &'static [Encoding];
}

macro_rules! encode_args_impl {
    ($($Arg: ident),*) => {
        impl<$($Arg: EncodeConvertArgument),*> args_private::Sealed for ($($Arg,)*) {}

        unsafe impl<$($Arg: EncodeConvertArgument),*> EncodeArguments for ($($Arg,)*) {
            const ENCODINGS: &'static [Encoding] = &[
                // T::__Inner::ENCODING => T::ENCODING
                // bool::__Inner::ENCODING => Bool::ENCODING
                $($Arg::__Inner::ENCODING),*
            ];
        }
    };
}

encode_args_impl!();
encode_args_impl!(A);
encode_args_impl!(A, B);
encode_args_impl!(A, B, C);
encode_args_impl!(A, B, C, D);
encode_args_impl!(A, B, C, D, E);
encode_args_impl!(A, B, C, D, E, F);
encode_args_impl!(A, B, C, D, E, F, G);
encode_args_impl!(A, B, C, D, E, F, G, H);
encode_args_impl!(A, B, C, D, E, F, G, H, I);
encode_args_impl!(A, B, C, D, E, F, G, H, I, J);
encode_args_impl!(A, B, C, D, E, F, G, H, I, J, K);
encode_args_impl!(A, B, C, D, E, F, G, H, I, J, K, L);
encode_args_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M);
encode_args_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
encode_args_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
encode_args_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);

#[cfg(test)]
mod tests {
    use super::*;

    use core::any::TypeId;

    #[test]
    fn test_return() {
        assert_eq!(<i32>::ENCODING_RETURN, <i32>::ENCODING);
        assert_eq!(<()>::ENCODING_RETURN, Encoding::Void);
    }

    #[test]
    fn convert_normally_noop() {
        assert_eq!(
            TypeId::of::<<i32 as EncodeConvertArgument>::__Inner>(),
            TypeId::of::<i32>()
        );
        assert_eq!(<i32 as EncodeConvertArgument>::__from_inner(42), 42);
        assert_eq!(EncodeConvertArgument::__into_inner(42i32), 42);
    }

    #[test]
    fn convert_i8() {
        assert_eq!(
            TypeId::of::<<i8 as EncodeConvertArgument>::__Inner>(),
            TypeId::of::<i8>()
        );
        assert_eq!(<i8 as EncodeConvertArgument>::__from_inner(-3), -3);
        assert_eq!(EncodeConvertArgument::__into_inner(-3i32), -3);
    }

    #[test]
    fn convert_bool() {
        assert!(!<bool as EncodeConvertArgument>::__from_inner(Bool::NO));
        assert!(<bool as EncodeConvertArgument>::__from_inner(Bool::YES));
        assert!(!<bool as EncodeConvertReturn>::__from_inner(Bool::NO));
        assert!(<bool as EncodeConvertReturn>::__from_inner(Bool::YES));

        assert!(!EncodeConvertArgument::__into_inner(false).as_bool());
        assert!(EncodeConvertArgument::__into_inner(true).as_bool());
        assert!(!EncodeConvertReturn::__into_inner(false).as_bool());
        assert!(EncodeConvertReturn::__into_inner(true).as_bool());

        #[cfg(all(feature = "apple", target_os = "macos", target_arch = "x86_64"))]
        assert_eq!(
            <bool as EncodeConvertArgument>::__Inner::ENCODING,
            Encoding::Char
        );
    }

    #[test]
    fn test_encode_arguments() {
        assert!(<()>::ENCODINGS.is_empty());
        assert_eq!(<(i8,)>::ENCODINGS, &[i8::ENCODING]);
        assert_eq!(<(i8, u32)>::ENCODINGS, &[i8::ENCODING, u32::ENCODING]);
    }
}
