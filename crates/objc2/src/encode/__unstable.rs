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
use crate::rc::Id;
use crate::runtime::Bool;
use crate::Message;

mod return_private {
    pub trait Sealed {}
}

/// Types that are safe as the return value from Objective-C.
///
/// We currently don't need a similar `EncodeArgument` trait, but we might in
/// the future.
///
///
/// # Safety
///
/// Similar to [`Encode`].
//
// Note: While this is not public, it is still a breaking change to change,
// since `block2` relies on it.
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

// Implemented in rc/writeback.rs
impl<T: Message> convert_private::Sealed for &mut Id<T> {}
impl<T: Message> convert_private::Sealed for Option<&mut Id<T>> {}
impl<T: Message> convert_private::Sealed for &mut Option<Id<T>> {}
impl<T: Message> convert_private::Sealed for Option<&mut Option<Id<T>>> {}

/// Represents types that can be converted to/from an [`Encode`] type.
///
/// This is implemented specially for [`bool`] to allow using that as
/// Objective-C `BOOL`, where it would otherwise not be allowed (since they
/// are not ABI compatible).
///
/// This is also done specially for `&mut Id<_>`-like arguments, to allow
/// using those as "out" parameters.
pub trait EncodeConvertArgument: convert_private::Sealed {
    /// The inner type that this can be converted to and from.
    #[doc(hidden)]
    type __Inner: Encode;

    /// A helper type for out parameters.
    #[doc(hidden)]
    type __StoredBeforeMessage: Sized;

    #[doc(hidden)]
    fn __from_declared_param(inner: Self::__Inner) -> Self;

    #[doc(hidden)]
    fn __into_argument(self) -> (Self::__Inner, Self::__StoredBeforeMessage);

    #[doc(hidden)]
    unsafe fn __process_after_message_send(_stored: Self::__StoredBeforeMessage) {}
}

/// Same as [`EncodeConvertArgument`], but for return types.
pub trait EncodeConvertReturn: convert_private::Sealed {
    /// The inner type that this can be converted to and from.
    #[doc(hidden)]
    type __Inner: EncodeReturn;

    #[doc(hidden)]
    fn __into_declared_return(self) -> Self::__Inner;

    #[doc(hidden)]
    fn __from_return(inner: Self::__Inner) -> Self;
}

impl<T: Encode> EncodeConvertArgument for T {
    type __Inner = Self;

    type __StoredBeforeMessage = ();

    #[inline]
    fn __from_declared_param(inner: Self::__Inner) -> Self {
        inner
    }

    #[inline]
    fn __into_argument(self) -> (Self::__Inner, Self::__StoredBeforeMessage) {
        (self, ())
    }
}

impl<T: EncodeReturn> EncodeConvertReturn for T {
    type __Inner = Self;

    #[inline]
    fn __into_declared_return(self) -> Self::__Inner {
        self
    }

    #[inline]
    fn __from_return(inner: Self::__Inner) -> Self {
        inner
    }
}

impl EncodeConvertArgument for bool {
    type __Inner = Bool;

    type __StoredBeforeMessage = ();

    #[inline]
    fn __from_declared_param(inner: Self::__Inner) -> Self {
        inner.as_bool()
    }

    #[inline]
    fn __into_argument(self) -> (Self::__Inner, Self::__StoredBeforeMessage) {
        (Bool::new(self), ())
    }
}

impl EncodeConvertReturn for bool {
    type __Inner = Bool;

    #[inline]
    fn __into_declared_return(self) -> Self::__Inner {
        Bool::new(self)
    }

    #[inline]
    fn __from_return(inner: Self::__Inner) -> Self {
        inner.as_bool()
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
pub trait EncodeArguments: args_private::Sealed {
    /// The encodings for the arguments.
    const ENCODINGS: &'static [Encoding];
}

macro_rules! encode_args_impl {
    ($($Arg: ident),*) => {
        impl<$($Arg: EncodeConvertArgument),*> args_private::Sealed for ($($Arg,)*) {}

        impl<$($Arg: EncodeConvertArgument),*> EncodeArguments for ($($Arg,)*) {
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
        assert_eq!(
            <i32 as EncodeConvertArgument>::__from_declared_param(42),
            42
        );
        assert_eq!(EncodeConvertArgument::__into_argument(42i32).0, 42);
    }

    #[test]
    fn convert_i8() {
        assert_eq!(
            TypeId::of::<<i8 as EncodeConvertArgument>::__Inner>(),
            TypeId::of::<i8>()
        );
        assert_eq!(<i8 as EncodeConvertArgument>::__from_declared_param(-3), -3);
        assert_eq!(EncodeConvertArgument::__into_argument(-3i32).0, -3);
    }

    #[test]
    fn convert_bool() {
        assert!(!<bool as EncodeConvertArgument>::__from_declared_param(
            Bool::NO
        ));
        assert!(<bool as EncodeConvertArgument>::__from_declared_param(
            Bool::YES
        ));
        assert!(!<bool as EncodeConvertReturn>::__from_return(Bool::NO));
        assert!(<bool as EncodeConvertReturn>::__from_return(Bool::YES));

        assert!(!EncodeConvertArgument::__into_argument(false).0.as_bool());
        assert!(EncodeConvertArgument::__into_argument(true).0.as_bool());
        assert!(!EncodeConvertReturn::__into_declared_return(false).as_bool());
        assert!(EncodeConvertReturn::__into_declared_return(true).as_bool());

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
