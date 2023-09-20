use core::mem::{self, ManuallyDrop};
use core::ptr::{self, NonNull};

use crate::__macro_helpers::{ConvertArgument, ConvertReturn};
use crate::encode::{Encode, EncodeArgument, EncodeReturn, Encoding};
use crate::mutability::IsMutable;
use crate::rc::Id;
use crate::runtime::{AnyClass, AnyObject, Imp, Sel};
use crate::{ClassType, Message};

/// Wrap the given closure in `exception::catch` if the `catch-all` feature is
/// enabled.
///
/// This is a macro to help with monomorphization when the feature is
/// disabled, as well as improving the final stack trace (`#[track_caller]`
/// doesn't really work on closures).
#[cfg(not(feature = "catch-all"))]
macro_rules! conditional_try {
    (|| $expr:expr) => {
        $expr
    };
}

#[cfg(feature = "catch-all")]
macro_rules! conditional_try {
    (|| $expr:expr) => {{
        let f = core::panic::AssertUnwindSafe(|| $expr);
        match crate::exception::catch(f) {
            Ok(r) => r,
            Err(exception) => {
                if let Some(exception) = exception {
                    panic!("uncaught {exception:?}")
                } else {
                    panic!("uncaught exception nil")
                }
            }
        }
    }};
}

#[cfg(feature = "apple")]
mod msg_send_primitive {
    #[allow(unused_imports)]
    use core::mem;

    use super::MessageArguments;
    use crate::encode::EncodeReturn;
    #[allow(unused_imports)]
    use crate::encode::Encoding;
    use crate::ffi;
    use crate::runtime::{AnyClass, AnyObject, Imp, Sel};

    /// On the below architectures we can statically find the correct method to
    /// call from the return type, by looking at its `EncodeReturn` impl.
    #[allow(clippy::missing_safety_doc)]
    unsafe trait MsgSendFn: EncodeReturn {
        const MSG_SEND: Imp;
        const MSG_SEND_SUPER: Imp;
    }

    #[cfg(target_arch = "aarch64")]
    /// `objc_msgSend_stret` is not even available in arm64.
    ///
    /// <https://twitter.com/gparker/status/378079715824660480>
    unsafe impl<T: EncodeReturn> MsgSendFn for T {
        const MSG_SEND: Imp = ffi::objc_msgSend;
        const MSG_SEND_SUPER: Imp = ffi::objc_msgSendSuper;
    }

    #[cfg(target_arch = "arm")]
    /// Double-word sized fundamental data types don't use stret, but any
    /// composite type larger than 4 bytes does.
    ///
    /// <https://web.archive.org/web/20191016000656/http://infocenter.arm.com/help/topic/com.arm.doc.ihi0042f/IHI0042F_aapcs.pdf>
    /// <https://developer.arm.com/documentation/ihi0042/latest>
    unsafe impl<T: EncodeReturn> MsgSendFn for T {
        const MSG_SEND: Imp = {
            if let Encoding::LongLong | Encoding::ULongLong | Encoding::Double = T::ENCODING_RETURN
            {
                ffi::objc_msgSend
            } else if mem::size_of::<T>() <= 4 {
                ffi::objc_msgSend
            } else {
                ffi::objc_msgSend_stret
            }
        };
        const MSG_SEND_SUPER: Imp = {
            if let Encoding::LongLong | Encoding::ULongLong | Encoding::Double = T::ENCODING_RETURN
            {
                ffi::objc_msgSendSuper
            } else if mem::size_of::<T>() <= 4 {
                ffi::objc_msgSendSuper
            } else {
                ffi::objc_msgSendSuper_stret
            }
        };
    }

    #[cfg(target_arch = "x86")]
    /// Structures 1 or 2 bytes in size are placed in EAX.
    /// Structures 4 or 8 bytes in size are placed in: EAX and EDX.
    /// Structures of other sizes are placed at the address supplied by the caller.
    ///
    /// <https://developer.apple.com/library/mac/documentation/DeveloperTools/Conceptual/LowLevelABI/130-IA-32_Function_Calling_Conventions/IA32.html>
    unsafe impl<T: EncodeReturn> MsgSendFn for T {
        const MSG_SEND: Imp = {
            // See https://github.com/apple-oss-distributions/objc4/blob/objc4-818.2/runtime/message.h#L156-L172
            if let Encoding::Float | Encoding::Double | Encoding::LongDouble = T::ENCODING_RETURN {
                ffi::objc_msgSend_fpret
            } else if let 0 | 1 | 2 | 4 | 8 = mem::size_of::<T>() {
                ffi::objc_msgSend
            } else {
                ffi::objc_msgSend_stret
            }
        };
        const MSG_SEND_SUPER: Imp = {
            if let 0 | 1 | 2 | 4 | 8 = mem::size_of::<T>() {
                ffi::objc_msgSendSuper
            } else {
                ffi::objc_msgSendSuper_stret
            }
        };
    }

    #[cfg(target_arch = "x86_64")]
    /// If the size of an object is larger than two eightbytes, it has class
    /// MEMORY. If the type has class MEMORY, then the caller provides space for
    /// the return value and passes the address of this storage.
    ///
    /// <https://www.uclibc.org/docs/psABI-x86_64.pdf>
    unsafe impl<T: EncodeReturn> MsgSendFn for T {
        const MSG_SEND: Imp = {
            // See https://github.com/apple-oss-distributions/objc4/blob/objc4-818.2/runtime/message.h#L156-L172
            if let Encoding::LongDouble = T::ENCODING_RETURN {
                ffi::objc_msgSend_fpret
            } else if let Encoding::LongDoubleComplex = T::ENCODING_RETURN {
                ffi::objc_msgSend_fp2ret
            } else if mem::size_of::<T>() <= 16 {
                ffi::objc_msgSend
            } else {
                ffi::objc_msgSend_stret
            }
        };
        const MSG_SEND_SUPER: Imp = {
            if mem::size_of::<T>() <= 16 {
                ffi::objc_msgSendSuper
            } else {
                ffi::objc_msgSendSuper_stret
            }
        };
    }

    #[inline]
    #[track_caller]
    pub(crate) unsafe fn send_unverified<A, R>(receiver: *mut AnyObject, sel: Sel, args: A) -> R
    where
        A: MessageArguments,
        R: EncodeReturn,
    {
        let msg_send_fn = R::MSG_SEND;
        unsafe { A::__invoke(msg_send_fn, receiver, sel, args) }
    }

    #[inline]
    #[track_caller]
    pub(crate) unsafe fn send_super_unverified<A, R>(
        receiver: *mut AnyObject,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> R
    where
        A: MessageArguments,
        R: EncodeReturn,
    {
        let superclass: *const AnyClass = superclass;
        let mut sup = ffi::objc_super {
            receiver: receiver.cast(),
            super_class: superclass.cast(),
        };
        let receiver: *mut ffi::objc_super = &mut sup;
        let receiver = receiver.cast();

        let msg_send_fn = R::MSG_SEND_SUPER;
        unsafe { A::__invoke(msg_send_fn, receiver, sel, args) }
    }
}

#[cfg(feature = "gnustep-1-7")]
mod msg_send_primitive {
    use core::mem;

    use super::MessageArguments;
    use crate::encode::EncodeReturn;
    use crate::ffi;
    use crate::runtime::{AnyClass, AnyObject, Imp, Sel};

    #[inline]
    fn unwrap_msg_send_fn(msg_send_fn: Option<Imp>) -> Imp {
        match msg_send_fn {
            Some(msg_send_fn) => msg_send_fn,
            None => {
                // SAFETY: This will never be NULL, even if the selector is not
                // found a callable function pointer will still be returned!
                //
                // `clang` doesn't insert a NULL check here either.
                unsafe { core::hint::unreachable_unchecked() }
            }
        }
    }

    #[track_caller]
    pub(crate) unsafe fn send_unverified<A, R>(receiver: *mut AnyObject, sel: Sel, args: A) -> R
    where
        A: MessageArguments,
        R: EncodeReturn,
    {
        // If `receiver` is NULL, objc_msg_lookup will return a standard C-method
        // taking two arguments, the receiver and the selector. Transmuting and
        // calling such a function with multiple parameters is UB, so instead we
        // return NULL directly.
        if receiver.is_null() {
            // SAFETY: Caller guarantees that messages to NULL-receivers only
            // return pointers, and a mem::zeroed pointer is just a NULL-pointer.
            return unsafe { mem::zeroed() };
        }

        let msg_send_fn = unsafe { ffi::objc_msg_lookup(receiver.cast(), sel.as_ptr()) };
        let msg_send_fn = unwrap_msg_send_fn(msg_send_fn);
        unsafe { A::__invoke(msg_send_fn, receiver, sel, args) }
    }

    #[track_caller]
    pub(crate) unsafe fn send_super_unverified<A, R>(
        receiver: *mut AnyObject,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> R
    where
        A: MessageArguments,
        R: EncodeReturn,
    {
        if receiver.is_null() {
            // SAFETY: Same as in `send_unverified`.
            return unsafe { mem::zeroed() };
        }

        let superclass: *const AnyClass = superclass;
        let sup = ffi::objc_super {
            receiver: receiver.cast(),
            super_class: superclass.cast(),
        };
        let msg_send_fn = unsafe { ffi::objc_msg_lookup_super(&sup, sel.as_ptr()) };
        let msg_send_fn = unwrap_msg_send_fn(msg_send_fn);
        unsafe { A::__invoke(msg_send_fn, receiver, sel, args) }
    }
}

/// Help with monomorphizing in `icrate`
#[cfg(debug_assertions)]
#[track_caller]
fn msg_send_check(
    obj: Option<&AnyObject>,
    sel: Sel,
    args: &[crate::encode::Encoding],
    ret: &crate::encode::Encoding,
) {
    let cls = if let Some(obj) = obj {
        obj.class()
    } else {
        panic_null(sel)
    };

    msg_send_check_class(cls, sel, args, ret);
}

#[cfg(debug_assertions)]
#[track_caller]
fn msg_send_check_class(
    cls: &AnyClass,
    sel: Sel,
    args: &[crate::encode::Encoding],
    ret: &crate::encode::Encoding,
) {
    use crate::verify::{verify_method_signature, Inner, VerificationError};

    let err = if let Some(method) = cls.instance_method(sel) {
        if let Err(err) = verify_method_signature(method, args, ret) {
            err
        } else {
            return;
        }
    } else {
        VerificationError::from(Inner::MethodNotFound)
    };

    panic_verify(cls, sel, &err);
}

#[cfg(debug_assertions)]
#[track_caller]
fn panic_null(sel: Sel) -> ! {
    panic!("messsaging {sel} to nil")
}

#[cfg(debug_assertions)]
#[track_caller]
fn panic_verify(cls: &AnyClass, sel: Sel, err: &crate::runtime::VerificationError) -> ! {
    panic!(
        "invalid message send to {}[{cls} {sel}]: {err}",
        if cls.is_metaclass() { "+" } else { "-" },
    )
}

mod private {
    pub trait Sealed {}
}

/// Types that can directly be used as the receiver of Objective-C messages.
///
/// Examples include objects pointers, class pointers, and block pointers.
///
/// This is a sealed trait (for now) that is automatically implemented for
/// pointers to types implementing [`Message`], so that code can be generic
/// over the message receiver.
///
/// This is mostly an implementation detail; you'll want to implement
/// [`Message`] for your type instead.
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait MessageReceiver: private::Sealed + Sized {
    #[doc(hidden)]
    type __Inner: ?Sized;

    #[doc(hidden)]
    fn __as_raw_receiver(self) -> *mut AnyObject;

    /// Sends a message to the receiver with the given selector and arguments.
    ///
    /// The correct version of `objc_msgSend` will be chosen based on the
    /// return type. For more information, see the section on "Sending
    /// Messages" in Apple's [documentation][runtime].
    ///
    /// If the selector is known at compile-time, it is recommended to use the
    /// [`msg_send!`] macro rather than this method.
    ///
    /// [runtime]: https://developer.apple.com/documentation/objectivec/objective-c_runtime?language=objc
    ///
    ///
    /// # Safety
    ///
    /// This shares the same safety requirements as [`msg_send!`].
    ///
    /// The added invariant is that the selector must take the same number of
    /// arguments as is given.
    ///
    /// [`msg_send!`]: crate::msg_send
    #[inline]
    #[track_caller]
    unsafe fn send_message<A, R>(self, sel: Sel, args: A) -> R
    where
        A: MessageArguments,
        R: ConvertReturn,
    {
        let this = self.__as_raw_receiver();
        #[cfg(debug_assertions)]
        {
            // SAFETY: Caller ensures only valid or NULL pointers.
            let obj = unsafe { this.as_ref() };
            msg_send_check(obj, sel, A::__ENCODINGS, &R::__Inner::ENCODING_RETURN);
        }
        unsafe {
            ConvertReturn::__from_return(conditional_try!(|| msg_send_primitive::send_unverified(
                this, sel, args
            )))
        }
    }

    /// Sends a message to a specific superclass with the given selector and
    /// arguments.
    ///
    /// The correct version of `objc_msgSend_super` will be chosen based on the
    /// return type. For more information, see the section on "Sending
    /// Messages" in Apple's [documentation][runtime].
    ///
    /// If the selector is known at compile-time, it is recommended to use the
    /// [`msg_send!(super(...), ...)`] macro rather than this method.
    ///
    /// [runtime]: https://developer.apple.com/documentation/objectivec/objective-c_runtime?language=objc
    ///
    ///
    /// # Safety
    ///
    /// This shares the same safety requirements as
    /// [`msg_send!(super(...), ...)`].
    ///
    /// The added invariant is that the selector must take the same number of
    /// arguments as is given.
    ///
    /// [`msg_send!(super(...), ...)`]: crate::msg_send
    #[inline]
    #[track_caller]
    unsafe fn send_super_message<A, R>(self, superclass: &AnyClass, sel: Sel, args: A) -> R
    where
        A: MessageArguments,
        R: ConvertReturn,
    {
        let this = self.__as_raw_receiver();
        #[cfg(debug_assertions)]
        {
            if this.is_null() {
                panic_null(sel);
            }
            msg_send_check_class(
                superclass,
                sel,
                A::__ENCODINGS,
                &R::__Inner::ENCODING_RETURN,
            );
        }
        unsafe {
            ConvertReturn::__from_return(conditional_try!(|| {
                msg_send_primitive::send_super_unverified(this, superclass, sel, args)
            }))
        }
    }

    #[inline]
    #[track_caller]
    #[doc(hidden)]
    unsafe fn __send_super_message_static<A, R>(self, sel: Sel, args: A) -> R
    where
        Self::__Inner: ClassType,
        <Self::__Inner as ClassType>::Super: ClassType,
        A: MessageArguments,
        R: ConvertReturn,
    {
        unsafe { self.send_super_message(<Self::__Inner as ClassType>::Super::class(), sel, args) }
    }

    // Error functions below. See MsgSendId::send_message_id_error for further
    // details.
    //
    // Some of this could be abstracted away using closures, but that would
    // interfere with `#[track_caller]`, so we avoid doing that.

    #[inline]
    #[track_caller]
    #[doc(hidden)]
    unsafe fn __send_message_error<A, E>(self, sel: Sel, args: A) -> Result<(), Id<E>>
    where
        *mut *mut E: Encode,
        A: __TupleExtender<*mut *mut E>,
        <A as __TupleExtender<*mut *mut E>>::PlusOneArgument: MessageArguments,
        E: Message,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        let res: bool = unsafe { self.send_message(sel, args) };
        if res {
            Ok(())
        } else {
            Err(unsafe { encountered_error(err) })
        }
    }

    #[inline]
    #[track_caller]
    #[doc(hidden)]
    unsafe fn __send_super_message_error<A, E>(
        self,
        superclass: &AnyClass,
        sel: Sel,
        args: A,
    ) -> Result<(), Id<E>>
    where
        *mut *mut E: Encode,
        A: __TupleExtender<*mut *mut E>,
        <A as __TupleExtender<*mut *mut E>>::PlusOneArgument: MessageArguments,
        E: Message,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        let res: bool = unsafe { self.send_super_message(superclass, sel, args) };
        if res {
            Ok(())
        } else {
            Err(unsafe { encountered_error(err) })
        }
    }

    #[inline]
    #[track_caller]
    #[doc(hidden)]
    unsafe fn __send_super_message_static_error<A, E>(self, sel: Sel, args: A) -> Result<(), Id<E>>
    where
        Self::__Inner: ClassType,
        <Self::__Inner as ClassType>::Super: ClassType,
        *mut *mut E: Encode,
        A: __TupleExtender<*mut *mut E>,
        <A as __TupleExtender<*mut *mut E>>::PlusOneArgument: MessageArguments,
        E: Message,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        let res: bool = unsafe { self.__send_super_message_static(sel, args) };
        if res {
            Ok(())
        } else {
            Err(unsafe { encountered_error(err) })
        }
    }
}

#[cold]
#[track_caller]
unsafe fn encountered_error<E: Message>(err: *mut E) -> Id<E> {
    // SAFETY: Ensured by caller
    unsafe { Id::retain(err) }.expect("error parameter should be set if the method returns NO")
}

// Note that we implement MessageReceiver for unsized types as well, this is
// to support `extern type`s in the future, not because we want to allow DSTs.

impl<T: ?Sized + Message> private::Sealed for *const T {}
unsafe impl<T: ?Sized + Message> MessageReceiver for *const T {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        (self as *mut T).cast()
    }
}

impl<T: ?Sized + Message> private::Sealed for *mut T {}
unsafe impl<T: ?Sized + Message> MessageReceiver for *mut T {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        self.cast()
    }
}

impl<T: ?Sized + Message> private::Sealed for NonNull<T> {}
unsafe impl<T: ?Sized + Message> MessageReceiver for NonNull<T> {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        self.as_ptr().cast()
    }
}

impl<'a, T: ?Sized + Message> private::Sealed for &'a T {}
unsafe impl<'a, T: ?Sized + Message> MessageReceiver for &'a T {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        let ptr: *const T = self;
        (ptr as *mut T).cast()
    }
}

// TODO: Use `T: IsMutable + Root` here once we can handle `init` methods
// better in `declare_class!`.
impl<'a, T: ?Sized + Message> private::Sealed for &'a mut T {}
unsafe impl<'a, T: ?Sized + Message> MessageReceiver for &'a mut T {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        let ptr: *mut T = self;
        ptr.cast()
    }
}

impl<'a, T: ?Sized + Message> private::Sealed for &'a Id<T> {}
unsafe impl<'a, T: ?Sized + Message> MessageReceiver for &'a Id<T> {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        (Id::as_ptr(self) as *mut T).cast()
    }
}

impl<'a, T: ?Sized + Message + IsMutable> private::Sealed for &'a mut Id<T> {}
unsafe impl<'a, T: ?Sized + Message + IsMutable> MessageReceiver for &'a mut Id<T> {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        Id::as_mut_ptr(self).cast()
    }
}

impl<T: ?Sized + Message> private::Sealed for ManuallyDrop<Id<T>> {}
unsafe impl<T: ?Sized + Message> MessageReceiver for ManuallyDrop<Id<T>> {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        Id::consume_as_ptr(ManuallyDrop::into_inner(self)).cast()
    }
}

impl private::Sealed for *const AnyClass {}
unsafe impl MessageReceiver for *const AnyClass {
    type __Inner = AnyClass;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        (self as *mut AnyClass).cast()
    }
}

impl<'a> private::Sealed for &'a AnyClass {}
unsafe impl<'a> MessageReceiver for &'a AnyClass {
    type __Inner = AnyClass;

    #[inline]
    fn __as_raw_receiver(self) -> *mut AnyObject {
        let ptr: *const AnyClass = self;
        (ptr as *mut AnyClass).cast()
    }
}

#[cfg(test)]
mod test_utils_hack {
    use super::*;
    use crate::test_utils::CustomObject;

    // TODO: Remove the need for this hack
    impl private::Sealed for &CustomObject {}
    impl private::Sealed for &mut CustomObject {}
    impl private::Sealed for ManuallyDrop<CustomObject> {}
}

mod message_args_private {
    pub trait Sealed {}
}

/// Types that may be used as the arguments of an Objective-C message.
///
/// This is implemented for tuples of up to 16 arguments, where each argument
/// implements [`Encode`] (or can be converted from one).
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait MessageArguments: message_args_private::Sealed {
    #[doc(hidden)]
    const __ENCODINGS: &'static [Encoding];

    /// Invoke an [`Imp`] with the given object, selector, and arguments.
    ///
    /// This method is the primitive used when sending messages and should not
    /// be called directly; instead, use the `msg_send!` macro or, in cases
    /// with a dynamic selector, the [`MessageReceiver::send_message`] method.
    #[doc(hidden)]
    unsafe fn __invoke<R: EncodeReturn>(imp: Imp, obj: *mut AnyObject, sel: Sel, args: Self) -> R;
}

pub trait __TupleExtender<T> {
    #[doc(hidden)]
    type PlusOneArgument;
    #[doc(hidden)]
    fn add_argument(self, arg: T) -> Self::PlusOneArgument;
}

macro_rules! message_args_impl {
    ($($a:ident: $t:ident),*) => (
        impl<$($t: ConvertArgument),*> message_args_private::Sealed for ($($t,)*) {}

        unsafe impl<$($t: ConvertArgument),*> MessageArguments for ($($t,)*) {
            const __ENCODINGS: &'static [Encoding] = &[
                $($t::__Inner::ENCODING_ARGUMENT),*
            ];

            #[inline]
            unsafe fn __invoke<R: EncodeReturn>(imp: Imp, obj: *mut AnyObject, sel: Sel, ($($a,)*): Self) -> R {
                $(let $a = ConvertArgument::__into_argument($a);)*

                // The imp must be cast to the appropriate function pointer
                // type before being called; the msgSend functions are not
                // parametric, but instead "trampolines" to the actual
                // method implementations.
                #[cfg(not(feature = "unstable-c-unwind"))]
                let imp: unsafe extern "C" fn(*mut AnyObject, Sel $(, $t::__Inner)*) -> R = unsafe {
                    mem::transmute(imp)
                };
                #[cfg(feature = "unstable-c-unwind")]
                let imp: unsafe extern "C-unwind" fn(*mut AnyObject, Sel $(, $t::__Inner)*) -> R = unsafe {
                    mem::transmute(imp)
                };
                // TODO: On x86_64 it would be more efficient to use a GOT
                // entry here (e.g. adding `nonlazybind` in LLVM).
                // Same can be said of e.g. `objc_retain` and `objc_release`.
                let result = unsafe { imp(obj, sel $(, $a.0)*) };

                // TODO: If we want `objc_retainAutoreleasedReturnValue` to
                // work, we must not do any work before it has been run; so
                // somehow, we should've done that before this call!
                $(
                    // SAFETY: The argument was passed to the message sending
                    // function, and the stored values are only processed this
                    // once. See `src/__macro_helpers/writeback.rs` for
                    // details.
                    unsafe { <$t as ConvertArgument>::__process_after_message_send($a.1) };
                )*
                result
            }
        }

        impl<$($t,)* T> __TupleExtender<T> for ($($t,)*) {
            type PlusOneArgument = ($($t,)* T,);

            fn add_argument(self, arg: T) -> Self::PlusOneArgument {
                let ($($a,)*) = self;
                ($($a,)* arg,)
            }
        }
    );
}

message_args_impl!();
message_args_impl!(a: A);
message_args_impl!(a: A, b: B);
message_args_impl!(a: A, b: B, c: C);
message_args_impl!(a: A, b: B, c: C, d: D);
message_args_impl!(a: A, b: B, c: C, d: D, e: E);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
message_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);
message_args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K
);
message_args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L
);
message_args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L,
    m: M
);
message_args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L,
    m: M,
    n: N
);
message_args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L,
    m: M,
    n: N,
    o: O
);
message_args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L,
    m: M,
    n: N,
    o: O,
    p: P
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutability;
    use crate::rc::Id;
    use crate::runtime::NSObject;
    use crate::test_utils;
    use crate::{declare_class, msg_send, msg_send_id};

    declare_class!(
        struct MutableObject;

        unsafe impl ClassType for MutableObject {
            type Super = NSObject;
            type Mutability = mutability::Mutable;
            const NAME: &'static str = "TestMutableObject";
        }
    );

    #[allow(unused)]
    fn test_different_receivers(mut obj: Id<MutableObject>) {
        unsafe {
            let x = &mut obj;
            let _: () = msg_send![x, mutable1];
            // let _: () = msg_send![x, mutable2];
            let _: () = msg_send![&mut *obj, mutable1];
            let _: () = msg_send![&mut *obj, mutable2];
            #[allow(clippy::needless_borrow)]
            let obj: NonNull<MutableObject> = (&mut *obj).into();
            let _: () = msg_send![obj, mutable1];
            let _: () = msg_send![obj, mutable2];
            let obj: *mut MutableObject = obj.as_ptr();
            let _: () = msg_send![obj, mutable1];
            let _: () = msg_send![obj, mutable2];
        }
    }

    #[test]
    fn test_send_message() {
        let mut obj = test_utils::custom_object();
        let result: u32 = unsafe {
            let _: () = msg_send![&mut obj, setFoo: 4u32];
            msg_send![&obj, foo]
        };
        assert_eq!(result, 4);
    }

    #[test]
    fn test_send_message_stret() {
        let obj = test_utils::custom_object();
        let result: test_utils::CustomStruct = unsafe { msg_send![&obj, customStruct] };
        let expected = test_utils::CustomStruct {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
        };
        assert_eq!(result, expected);
    }

    #[test]
    #[cfg_attr(debug_assertions, should_panic = "messsaging description to nil")]
    fn test_send_message_nil() {
        let nil: *mut NSObject = ::core::ptr::null_mut();

        // This result should not be relied on
        let result: Option<Id<NSObject>> = unsafe { msg_send_id![nil, description] };
        assert!(result.is_none());

        // This result should not be relied on
        let result: usize = unsafe { msg_send![nil, hash] };
        assert_eq!(result, 0);

        // This result should not be relied on
        #[cfg(target_pointer_width = "16")]
        let result: f32 = 0.0;
        #[cfg(target_pointer_width = "32")]
        let result: f32 = unsafe { msg_send![nil, floatValue] };
        #[cfg(target_pointer_width = "64")]
        let result: f64 = unsafe { msg_send![nil, doubleValue] };
        assert_eq!(result, 0.0);

        // This result should not be relied on
        let result: Option<Id<NSObject>> =
            unsafe { msg_send_id![nil, multiple: 1u32, arguments: 2i8] };
        assert!(result.is_none());

        // This result should not be relied on
        let result: Option<Id<NSObject>> = unsafe { msg_send_id![None, init] };
        assert!(result.is_none());
    }

    #[test]
    fn test_send_message_super() {
        let mut obj = test_utils::custom_subclass_object();
        let superclass = test_utils::custom_class();
        unsafe {
            let _: () = msg_send![&mut obj, setFoo: 4u32];
            let foo: u32 = msg_send![super(&obj, superclass), foo];
            assert_eq!(foo, 4);

            // The subclass is overriden to return foo + 2
            let foo: u32 = msg_send![&obj, foo];
            assert_eq!(foo, 6);
        }
    }

    #[test]
    #[cfg_attr(
        feature = "gnustep-1-7",
        ignore = "GNUStep deadlocks here for some reason"
    )]
    fn test_send_message_class_super() {
        let cls = test_utils::custom_subclass();
        let superclass = test_utils::custom_class();
        unsafe {
            let foo: u32 = msg_send![super(cls, superclass.metaclass()), classFoo];
            assert_eq!(foo, 7);

            // The subclass is overriden to return + 2
            let foo: u32 = msg_send![cls, classFoo];
            assert_eq!(foo, 9);
        }
    }

    #[test]
    fn test_send_message_manuallydrop() {
        let obj = ManuallyDrop::new(test_utils::custom_object());
        unsafe {
            let _: () = msg_send![obj, release];
        };
        // `obj` is consumed, can't use here
    }
}
