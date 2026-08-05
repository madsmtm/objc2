use core::ffi::{c_char, CStr};
use core::ptr::NonNull;

use crate::encode::{EncodeArgument, EncodeArguments, EncodeReturn};
use crate::rc::{Allocated, Retained};
use crate::runtime::{AnyObject, Bool, Sel};
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
/// This is also done specially for `&mut Retained<_>`-like arguments, to
/// allow using those as "out" / pass-by-writeback parameters.
pub trait ConvertArgument: argument_private::Sealed + Sized {
    /// The inner type that this can be converted to and from.
    #[doc(hidden)]
    type __Inner: EncodeArgument;

    /// A helper type for out parameters in `msg_send!`/`extern_methods!`.
    ///
    /// When dropped, this will process any necessary change to the
    /// parameters before the message send returns.
    #[doc(hidden)]
    type __DropAfterMsgSend: Sized;

    /// A helper type for out parameters in `define_class!`.
    ///
    /// When dropped, this will process any necessary change to the
    /// parameters before the method returns.
    #[doc(hidden)]
    type __DropBeforeReturn: Sized;

    /// # Safety
    ///
    /// The `__OnDropAfterMsgSend` return type must not be leaked, and the
    /// `__Inner` pointer must not be used after the `__OnDropAfterMsgSend`
    /// has dropped.
    ///
    /// NOTE: The standard way to ensure such a thing is with closures, but
    /// using those would interact poorly with backtraces of the message send,
    /// so we're forced to ensure this out of band.
    #[doc(hidden)]
    unsafe fn __into_argument(self) -> (Self::__Inner, Self::__DropAfterMsgSend);

    /// # Safety
    ///
    /// The returned value must not be used after the `__OnDropBeforeReturn`
    /// has dropped.
    #[doc(hidden)]
    fn __from_defined_param(inner: Self::__Inner) -> (Self, Self::__DropBeforeReturn);
}

// Implemented in writeback.rs
impl<T: Message> argument_private::Sealed for &mut Retained<T> {}
impl<T: Message> argument_private::Sealed for Option<&mut Retained<T>> {}
impl<T: Message> argument_private::Sealed for &mut Option<Retained<T>> {}
impl<T: Message> argument_private::Sealed for Option<&mut Option<Retained<T>>> {}

impl<T: EncodeArgument> argument_private::Sealed for T {}
impl<T: EncodeArgument> ConvertArgument for T {
    type __Inner = Self;

    type __DropAfterMsgSend = ();
    type __DropBeforeReturn = ();

    #[inline]
    unsafe fn __into_argument(self) -> (Self::__Inner, Self::__DropAfterMsgSend) {
        (self, ())
    }

    #[inline]
    fn __from_defined_param(inner: Self::__Inner) -> (Self, Self::__DropBeforeReturn) {
        (inner, ())
    }
}

impl argument_private::Sealed for bool {}
impl ConvertArgument for bool {
    type __Inner = Bool;

    type __DropAfterMsgSend = ();
    type __DropBeforeReturn = ();

    #[inline]
    unsafe fn __into_argument(self) -> (Self::__Inner, Self::__DropAfterMsgSend) {
        (Bool::new(self), ())
    }

    #[inline]
    fn __from_defined_param(inner: Self::__Inner) -> (Self, Self::__DropBeforeReturn) {
        (inner.as_bool(), ())
    }
}

impl argument_private::Sealed for &CStr {}
impl ConvertArgument for &CStr {
    type __Inner = NonNull<c_char>;

    type __DropAfterMsgSend = ();
    type __DropBeforeReturn = ();

    #[inline]
    unsafe fn __into_argument(self) -> (Self::__Inner, Self::__DropAfterMsgSend) {
        let ptr = NonNull::new(self.as_ptr().cast_mut()).unwrap();
        (ptr, ())
    }

    #[inline]
    fn __from_defined_param(inner: Self::__Inner) -> (Self, Self::__DropBeforeReturn) {
        // SAFETY: The pointer comes from the caller, and the signature the
        // user wrote for the defined method denotes the lifetime.
        (unsafe { CStr::from_ptr(inner.as_ptr()) }, ())
    }
}

impl argument_private::Sealed for Option<&CStr> {}
impl ConvertArgument for Option<&CStr> {
    type __Inner = Option<NonNull<c_char>>;

    type __DropAfterMsgSend = ();
    type __DropBeforeReturn = ();

    #[inline]
    unsafe fn __into_argument(self) -> (Self::__Inner, Self::__DropAfterMsgSend) {
        let ptr = self.map(|x| NonNull::new(x.as_ptr().cast_mut()).unwrap());
        (ptr, ())
    }

    #[inline]
    fn __from_defined_param(inner: Self::__Inner) -> (Self, Self::__DropBeforeReturn) {
        // SAFETY: The pointer comes from the caller, and the signature the
        // user wrote for the defined method denotes the lifetime.
        (inner.map(|x| unsafe { CStr::from_ptr(x.as_ptr()) }), ())
    }
}

mod return_private {
    pub trait Sealed {}
}

/// Same as [`ConvertArgument`], but for return types.
///
/// See `RetainSemantics` for more details.
pub trait ConvertReturn<MethodFamily>: return_private::Sealed {
    type Inner: EncodeReturn;

    #[track_caller]
    unsafe fn convert_message_return(
        inner: Self::Inner,
        receiver_ptr: *mut AnyObject,
        sel: Sel,
    ) -> Self;

    fn convert_defined_return(self) -> Self::Inner;
}

impl<T: EncodeReturn> return_private::Sealed for T {}
impl<T: EncodeReturn, MethodFamily> ConvertReturn<MethodFamily> for T {
    type Inner = Self;

    #[inline]
    unsafe fn convert_message_return(
        inner: Self::Inner,
        _receiver_ptr: *mut AnyObject,
        _sel: Sel,
    ) -> Self {
        inner
    }

    #[inline]
    fn convert_defined_return(self) -> Self::Inner {
        self
    }
}

impl return_private::Sealed for bool {}
impl<MethodFamily> ConvertReturn<MethodFamily> for bool {
    type Inner = Bool;

    #[inline]
    unsafe fn convert_message_return(
        inner: Self::Inner,
        _receiver_ptr: *mut AnyObject,
        _sel: Sel,
    ) -> Self {
        inner.as_bool()
    }

    #[inline]
    fn convert_defined_return(self) -> Self::Inner {
        Bool::new(self)
    }
}

impl return_private::Sealed for &CStr {}
impl<MethodFamily> ConvertReturn<MethodFamily> for &CStr {
    type Inner = NonNull<c_char>;

    #[inline]
    unsafe fn convert_message_return(
        inner: Self::Inner,
        _receiver_ptr: *mut AnyObject,
        _sel: Sel,
    ) -> Self {
        // SAFETY: The pointer comes from the caller, and the signature the
        // user wrote for the called method denotes the lifetime.
        unsafe { CStr::from_ptr(inner.as_ptr()) }
    }

    #[inline]
    fn convert_defined_return(self) -> Self::Inner {
        NonNull::new(self.as_ptr().cast_mut()).unwrap()
    }
}

impl return_private::Sealed for Option<&CStr> {}
impl<MethodFamily> ConvertReturn<MethodFamily> for Option<&CStr> {
    type Inner = Option<NonNull<c_char>>;

    #[inline]
    unsafe fn convert_message_return(
        inner: Self::Inner,
        _receiver_ptr: *mut AnyObject,
        _sel: Sel,
    ) -> Self {
        // SAFETY: The pointer comes from the caller, and the signature the
        // user wrote for the called method denotes the lifetime.
        inner.map(|x| unsafe { CStr::from_ptr(x.as_ptr()) })
    }

    #[inline]
    fn convert_defined_return(self) -> Self::Inner {
        self.map(|x| NonNull::new(x.as_ptr().cast_mut()).unwrap())
    }
}

// Implemented in retain_semantics.rs
impl<T: ?Sized + Message> return_private::Sealed for Retained<T> {}
impl<T: ?Sized + Message> return_private::Sealed for Option<Retained<T>> {}
impl<T: ?Sized + Message> return_private::Sealed for Allocated<T> {}

/// Helper for converting types when handling `Result<T, Retained<NSError>>`.
pub trait ConvertError: Sized {
    /// The type that is returned internally.
    type Inner;
    fn into_option(inner: Self::Inner) -> Option<Self>;
    fn from_option(option: Option<Self>) -> Self::Inner;
}

// `bool` -> `Result<(), _>`.
impl ConvertError for () {
    type Inner = bool;

    #[inline]
    fn into_option(inner: bool) -> Option<()> {
        inner.then_some(())
    }

    #[inline]
    fn from_option(option: Option<()>) -> bool {
        option.is_some()
    }
}

// `Option<Retained<T>>` -> `Result<Retained<T>, _>`.
impl<T: ?Sized> ConvertError for Retained<T> {
    type Inner = Option<Retained<T>>;

    #[inline]
    fn into_option(inner: Option<Self>) -> Option<Self> {
        inner
    }

    #[inline]
    fn from_option(option: Option<Self>) -> Option<Self> {
        option
    }
}

pub trait ConvertArguments {
    #[doc(hidden)]
    type __Inner: EncodeArguments;

    #[doc(hidden)]
    type __DropAfterMsgSend: Sized;

    #[doc(hidden)]
    unsafe fn __into_arguments(self) -> (Self::__Inner, Self::__DropAfterMsgSend);
}

pub trait TupleExtender<T> {
    #[doc(hidden)]
    type PlusOneArgument;
    #[doc(hidden)]
    fn add_argument(self, arg: T) -> Self::PlusOneArgument;
}

macro_rules! args_impl {
    ($($a:ident: $t:ident),*) => (
        impl<$($t: ConvertArgument),*> ConvertArguments for ($($t,)*) {
            type __Inner = ($($t::__Inner,)*);

            type __DropAfterMsgSend = ($($t::__DropAfterMsgSend,)*);

            #[inline]
            unsafe fn __into_arguments(self) -> (Self::__Inner, Self::__DropAfterMsgSend) {
                let ($($a,)*) = self;
                // SAFETY: Upheld by caller
                $(let $a = unsafe { ConvertArgument::__into_argument($a) };)*

                (($($a.0,)*), ($($a.1,)*))
            }
        }

        impl<$($t,)* T> TupleExtender<T> for ($($t,)*) {
            type PlusOneArgument = ($($t,)* T,);

            #[inline]
            fn add_argument(self, arg: T) -> Self::PlusOneArgument {
                let ($($a,)*) = self;
                ($($a,)* arg,)
            }
        }
    );
}

args_impl!();
args_impl!(t1: T1);
args_impl!(t1: T1, t2: T2);
args_impl!(t1: T1, t2: T2, t3: T3);
args_impl!(t1: T1, t2: T2, t3: T3, t4: T4);
args_impl!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5);
args_impl!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6);
args_impl!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7);
args_impl!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8);
args_impl!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9);
args_impl!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10);
args_impl!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10, t11: T11);
args_impl!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10, t11: T11, t12: T12);
args_impl!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10, t11: T11, t12: T12, t13: T13);
args_impl!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10, t11: T11, t12: T12, t13: T13, t14: T14);
args_impl!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10, t11: T11, t12: T12, t13: T13, t14: T14, t15: T15);
args_impl!(t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10, t11: T11, t12: T12, t13: T13, t14: T14, t15: T15, t16: T16);

#[cfg(test)]
mod tests {
    use super::*;

    use core::any::TypeId;
    use core::ptr;

    use crate::{define_class, msg_send, runtime::NSObject, sel, ClassType};

    #[test]
    fn convert_normally_noop() {
        assert_eq!(
            TypeId::of::<<i32 as ConvertArgument>::__Inner>(),
            TypeId::of::<i32>()
        );
        assert_eq!(<i32 as ConvertArgument>::__from_defined_param(42).0, 42);
        assert_eq!(unsafe { ConvertArgument::__into_argument(42i32).0 }, 42);
    }

    #[test]
    fn convert_i8() {
        assert_eq!(
            TypeId::of::<<i8 as ConvertArgument>::__Inner>(),
            TypeId::of::<i8>()
        );
        assert_eq!(<i8 as ConvertArgument>::__from_defined_param(-3).0, -3);
        assert_eq!(unsafe { ConvertArgument::__into_argument(-3i32).0 }, -3);
    }

    #[test]
    fn convert_bool() {
        let receiver_ptr = ptr::null_mut::<AnyObject>();
        let sel = sel!(foo);

        assert!(!<bool as ConvertArgument>::__from_defined_param(Bool::NO).0);
        assert!(<bool as ConvertArgument>::__from_defined_param(Bool::YES).0);
        assert!(!unsafe {
            <bool as ConvertReturn<()>>::convert_message_return(Bool::NO, receiver_ptr, sel)
        });
        assert!(unsafe {
            <bool as ConvertReturn<()>>::convert_message_return(Bool::YES, receiver_ptr, sel)
        });

        assert!(!unsafe { ConvertArgument::__into_argument(false).0 }.as_bool());
        assert!(unsafe { ConvertArgument::__into_argument(true).0 }.as_bool());
        assert!(!ConvertReturn::<()>::convert_defined_return(false).as_bool());
        assert!(ConvertReturn::<()>::convert_defined_return(true).as_bool());

        #[cfg(all(target_vendor = "apple", target_os = "macos", target_arch = "x86_64"))]
        assert_eq!(
            <bool as ConvertArgument>::__Inner::ENCODING_ARGUMENT,
            crate::encode::Encoding::Char,
        );
    }

    #[test]
    fn convert_cstr() {
        define_class!(
            #[unsafe(super(NSObject))]
            struct Foo;

            impl Foo {
                #[unsafe(method(foo:))]
                fn foo(arg: &CStr) -> &'static CStr {
                    // TODO: Add syntax for allowing returning internal pointers.
                    unsafe { std::mem::transmute(arg) }
                }

                #[unsafe(method(fooOptional:))]
                fn foo_optional(arg: Option<&CStr>) -> Option<&'static CStr> {
                    // TODO: Add syntax for allowing returning internal pointers.
                    unsafe { std::mem::transmute(arg) }
                }
            }
        );

        let cls = Foo::class();
        let cstr = CStr::from_bytes_with_nul(b"foobar\0").unwrap();

        let result: &CStr = unsafe { msg_send![cls, foo: cstr] };
        assert_eq!(result, cstr);

        let result: Option<&CStr> = unsafe { msg_send![cls, fooOptional: Some(cstr)] };
        assert_eq!(result.unwrap(), cstr);
        let result: Option<&CStr> = unsafe { msg_send![cls, fooOptional: None::<&CStr>] };
        assert_eq!(result, None);
    }
}
