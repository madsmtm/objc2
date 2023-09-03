use crate::encode::{EncodeArgument, EncodeReturn};
use crate::rc::Id;
use crate::runtime::Bool;
use crate::Message;

mod argument_private {
    pub trait Sealed {}
}

/// Represents types that can be converted to/from an [`EncodeArgument`] type.
///
/// This is implemented specially for [`bool`] to allow using that as
/// Objective-C `BOOL`, where it would otherwise not be allowed (since they
/// are not ABI compatible).
///
/// This is also done specially for `&mut Id<_>`-like arguments, to allow
/// using those as "out" parameters.
pub trait ConvertArgument: argument_private::Sealed {
    /// The inner type that this can be converted to and from.
    #[doc(hidden)]
    type __Inner: EncodeArgument;

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

// Implemented in writeback.rs
impl<T: Message> argument_private::Sealed for &mut Id<T> {}
impl<T: Message> argument_private::Sealed for Option<&mut Id<T>> {}
impl<T: Message> argument_private::Sealed for &mut Option<Id<T>> {}
impl<T: Message> argument_private::Sealed for Option<&mut Option<Id<T>>> {}

impl<T: EncodeArgument> argument_private::Sealed for T {}
impl<T: EncodeArgument> ConvertArgument for T {
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

impl argument_private::Sealed for bool {}
impl ConvertArgument for bool {
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

mod return_private {
    pub trait Sealed {}
}

/// Same as [`ConvertArgument`], but for return types.
pub trait ConvertReturn: return_private::Sealed {
    /// The inner type that this can be converted to and from.
    #[doc(hidden)]
    type __Inner: EncodeReturn;

    #[doc(hidden)]
    fn __into_declared_return(self) -> Self::__Inner;

    #[doc(hidden)]
    fn __from_return(inner: Self::__Inner) -> Self;
}

impl<T: EncodeReturn> return_private::Sealed for T {}
impl<T: EncodeReturn> ConvertReturn for T {
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

impl return_private::Sealed for bool {}
impl ConvertReturn for bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    use core::any::TypeId;

    #[test]
    fn convert_normally_noop() {
        assert_eq!(
            TypeId::of::<<i32 as ConvertArgument>::__Inner>(),
            TypeId::of::<i32>()
        );
        assert_eq!(<i32 as ConvertArgument>::__from_declared_param(42), 42);
        assert_eq!(ConvertArgument::__into_argument(42i32).0, 42);
    }

    #[test]
    fn convert_i8() {
        assert_eq!(
            TypeId::of::<<i8 as ConvertArgument>::__Inner>(),
            TypeId::of::<i8>()
        );
        assert_eq!(<i8 as ConvertArgument>::__from_declared_param(-3), -3);
        assert_eq!(ConvertArgument::__into_argument(-3i32).0, -3);
    }

    #[test]
    fn convert_bool() {
        assert!(!<bool as ConvertArgument>::__from_declared_param(Bool::NO));
        assert!(<bool as ConvertArgument>::__from_declared_param(Bool::YES));
        assert!(!<bool as ConvertReturn>::__from_return(Bool::NO));
        assert!(<bool as ConvertReturn>::__from_return(Bool::YES));

        assert!(!ConvertArgument::__into_argument(false).0.as_bool());
        assert!(ConvertArgument::__into_argument(true).0.as_bool());
        assert!(!ConvertReturn::__into_declared_return(false).as_bool());
        assert!(ConvertReturn::__into_declared_return(true).as_bool());

        #[cfg(all(feature = "apple", target_os = "macos", target_arch = "x86_64"))]
        assert_eq!(
            <bool as ConvertArgument>::__Inner::ENCODING_ARGUMENT,
            crate::encode::Encoding::Char,
        );
    }
}
