use core::mem;
use core::mem::ManuallyDrop;
use core::ptr::{self, NonNull};

use crate::encode::{Encode, EncodeArguments, EncodeConvert, RefEncode};
use crate::rc::{Id, Owned, Ownership, Shared};
use crate::runtime::{Class, Imp, Object, Sel};
use crate::ClassType;

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

/// Help with monomorphizing in `icrate`
#[cfg(debug_assertions)]
#[track_caller]
fn msg_send_check(
    obj: Option<&Object>,
    sel: Sel,
    args: &[crate::encode::Encoding],
    ret: &crate::encode::Encoding,
) {
    use crate::verify::{verify_method_signature, Inner, VerificationError};

    let cls = if let Some(obj) = obj {
        obj.class()
    } else {
        panic_null(sel)
    };

    let err = if let Some(method) = cls.instance_method(sel) {
        if let Err(err) = verify_method_signature(method, args, ret) {
            err
        } else {
            return;
        }
    } else {
        VerificationError::from(Inner::MethodNotFound)
    };

    panic_verify(cls, sel, err);
}

#[cfg(debug_assertions)]
#[track_caller]
fn panic_null(sel: Sel) -> ! {
    panic!("messsaging {sel:?} to nil")
}

#[cfg(debug_assertions)]
#[track_caller]
fn panic_verify(cls: &Class, sel: Sel, err: crate::VerificationError) -> ! {
    panic!(
        "invalid message send to {}[{cls:?} {sel:?}]: {err}",
        if cls.is_metaclass() { "+" } else { "-" },
    )
}

#[cfg(feature = "apple")]
#[path = "apple/mod.rs"]
mod platform;
#[cfg(feature = "gnustep-1-7")]
#[path = "gnustep.rs"]
mod platform;

use self::platform::{send_super_unverified, send_unverified};

/// Types that can be sent Objective-C messages.
///
/// Implementing this provides [`MessageReceiver`] implementations for common
/// pointer types and references to the type, which allows using them as the
/// receiver (first argument) in the [`msg_send!`][`crate::msg_send`] macro.
///
/// This trait also allows the object to be used in [`rc::Id`][`Id`].
///
/// This is a subtrait of [`RefEncode`], meaning the type must also implement
/// that, almost always as [`Encoding::Object`].
///
/// [`Encoding::Object`]: crate::Encoding::Object
///
///
/// # Safety
///
/// The type must represent an Objective-C object, meaning it:
/// - Must be valid to reinterpret as [`runtime::Object`][`Object`].
/// - Must be able to be the receiver of an Objective-C message sent with
///   [`objc_msgSend`] or similar.
/// - Must respond to the standard memory management `retain`, `release` and
///   `autorelease` messages.
/// - Must support weak references. TODO: Make a new trait for this, for
///   example `NSTextView` only supports weak references on macOS 10.12 or
///   above.
///
/// [`objc_msgSend`]: https://developer.apple.com/documentation/objectivec/1456712-objc_msgsend
///
///
/// # Example
///
/// ```
/// use objc2::runtime::Object;
/// use objc2::{Encoding, Message, RefEncode};
///
/// #[repr(C)]
/// struct MyObject {
///     // This has the exact same layout as `Object`
///     inner: Object
/// }
///
/// unsafe impl RefEncode for MyObject {
///     const ENCODING_REF: Encoding = Encoding::Object;
/// }
///
/// unsafe impl Message for MyObject {}
///
/// // `*mut MyObject` and other pointer/reference types to the object can
/// // now be used in `msg_send!`
/// //
/// // And `Id<MyObject, O>` can now be constructed.
/// ```
pub unsafe trait Message: RefEncode {}

unsafe impl Message for Object {}

// TODO: Make this fully private
pub(crate) mod private {
    use super::*;

    pub trait Sealed {}

    impl<T: Message + ?Sized> Sealed for *const T {}
    impl<T: Message + ?Sized> Sealed for *mut T {}
    impl<T: Message + ?Sized> Sealed for NonNull<T> {}

    impl<'a, T: Message + ?Sized> Sealed for &'a T {}
    impl<'a, T: Message + ?Sized> Sealed for &'a mut T {}

    impl<'a, T: Message + ?Sized, O: Ownership> Sealed for &'a Id<T, O> {}
    impl<'a, T: Message + ?Sized> Sealed for &'a mut Id<T, Owned> {}

    impl<T: Message + ?Sized, O: Ownership> Sealed for ManuallyDrop<Id<T, O>> {}

    impl Sealed for *const Class {}
    impl<'a> Sealed for &'a Class {}
}

/// Types that can directly be used as the receiver of Objective-C messages.
///
/// Examples include objects, classes, and blocks.
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
    fn __as_raw_receiver(self) -> *mut Object;

    /// Sends a message to self with the given selector and arguments.
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
        R: EncodeConvert,
    {
        let this = self.__as_raw_receiver();
        #[cfg(debug_assertions)]
        {
            // SAFETY: Caller ensures only valid or NULL pointers.
            let obj = unsafe { this.as_ref() };
            msg_send_check(obj, sel, A::ENCODINGS, &R::__Inner::ENCODING);
        }
        unsafe { EncodeConvert::__from_inner(send_unverified(this, sel, args)) }
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
    unsafe fn send_super_message<A, R>(self, superclass: &Class, sel: Sel, args: A) -> R
    where
        A: MessageArguments,
        R: EncodeConvert,
    {
        let this = self.__as_raw_receiver();
        #[cfg(debug_assertions)]
        {
            if this.is_null() {
                panic_null(sel);
            }
            if let Err(err) = superclass.verify_sel::<A, R>(sel) {
                panic_verify(superclass, sel, err);
            }
        }
        unsafe { EncodeConvert::__from_inner(send_super_unverified(this, superclass, sel, args)) }
    }

    #[inline]
    #[track_caller]
    #[doc(hidden)]
    unsafe fn __send_super_message_static<A, R>(self, sel: Sel, args: A) -> R
    where
        Self::__Inner: ClassType,
        <Self::__Inner as ClassType>::Super: ClassType,
        A: MessageArguments,
        R: EncodeConvert,
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
    unsafe fn __send_message_error<A, E>(self, sel: Sel, args: A) -> Result<(), Id<E, Shared>>
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
        superclass: &Class,
        sel: Sel,
        args: A,
    ) -> Result<(), Id<E, Shared>>
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
    unsafe fn __send_super_message_static_error<A, E>(
        self,
        sel: Sel,
        args: A,
    ) -> Result<(), Id<E, Shared>>
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
unsafe fn encountered_error<E: Message>(err: *mut E) -> Id<E, Shared> {
    // SAFETY: Ensured by caller
    unsafe { Id::retain(err) }.expect("error parameter should be set if the method returns NO")
}

// Note that we implement MessageReceiver for unsized types as well, this is
// to support `extern type`s in the future, not because we want to allow DSTs.

unsafe impl<T: Message + ?Sized> MessageReceiver for *const T {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut Object {
        (self as *mut T).cast()
    }
}

unsafe impl<T: Message + ?Sized> MessageReceiver for *mut T {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut Object {
        self.cast()
    }
}

unsafe impl<T: Message + ?Sized> MessageReceiver for NonNull<T> {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut Object {
        self.as_ptr().cast()
    }
}

unsafe impl<'a, T: Message + ?Sized> MessageReceiver for &'a T {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut Object {
        let ptr: *const T = self;
        (ptr as *mut T).cast()
    }
}

unsafe impl<'a, T: Message + ?Sized> MessageReceiver for &'a mut T {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut Object {
        let ptr: *mut T = self;
        ptr.cast()
    }
}

unsafe impl<'a, T: Message + ?Sized, O: Ownership> MessageReceiver for &'a Id<T, O> {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut Object {
        (Id::as_ptr(self) as *mut T).cast()
    }
}

unsafe impl<'a, T: Message + ?Sized> MessageReceiver for &'a mut Id<T, Owned> {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut Object {
        Id::as_mut_ptr(self).cast()
    }
}

unsafe impl<T: Message + ?Sized, O: Ownership> MessageReceiver for ManuallyDrop<Id<T, O>> {
    type __Inner = T;

    #[inline]
    fn __as_raw_receiver(self) -> *mut Object {
        Id::consume_as_ptr(self).cast()
    }
}

unsafe impl MessageReceiver for *const Class {
    type __Inner = Class;

    #[inline]
    fn __as_raw_receiver(self) -> *mut Object {
        (self as *mut Class).cast()
    }
}

unsafe impl<'a> MessageReceiver for &'a Class {
    type __Inner = Class;

    #[inline]
    fn __as_raw_receiver(self) -> *mut Object {
        let ptr: *const Class = self;
        (ptr as *mut Class).cast()
    }
}

/// Types that may be used as the arguments of an Objective-C message.
///
/// This is implemented for tuples of up to 16 arguments, where each argument
/// implements [`Encode`][crate::Encode] (or can be converted from one).
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait MessageArguments: EncodeArguments {
    /// Invoke an [`Imp`] with the given object, selector, and arguments.
    ///
    /// This method is the primitive used when sending messages and should not
    /// be called directly; instead, use the `msg_send!` macro or, in cases
    /// with a dynamic selector, the [`MessageReceiver::send_message`] method.
    #[doc(hidden)]
    unsafe fn __invoke<R: Encode>(imp: Imp, obj: *mut Object, sel: Sel, args: Self) -> R;
}

pub trait __TupleExtender<T> {
    #[doc(hidden)]
    type PlusOneArgument;
    #[doc(hidden)]
    fn add_argument(self, arg: T) -> Self::PlusOneArgument;
}

macro_rules! message_args_impl {
    ($($a:ident: $t:ident),*) => (
        unsafe impl<$($t: EncodeConvert),*> MessageArguments for ($($t,)*) {
            #[inline]
            unsafe fn __invoke<R: Encode>(imp: Imp, obj: *mut Object, sel: Sel, ($($a,)*): Self) -> R {
                // The imp must be cast to the appropriate function pointer
                // type before being called; the msgSend functions are not
                // parametric, but instead "trampolines" to the actual
                // method implementations.
                #[cfg(not(feature = "unstable-c-unwind"))]
                let imp: unsafe extern "C" fn(*mut Object, Sel $(, $t::__Inner)*) -> R = unsafe {
                    mem::transmute(imp)
                };
                #[cfg(feature = "unstable-c-unwind")]
                let imp: unsafe extern "C-unwind" fn(*mut Object, Sel $(, $t::__Inner)*) -> R = unsafe {
                    mem::transmute(imp)
                };
                // TODO: On x86_64 it would be more efficient to use a GOT
                // entry here (e.g. adding `nonlazybind` in LLVM).
                // Same can be said of e.g. `objc_retain` and `objc_release`.
                unsafe { imp(obj, sel $(, EncodeConvert::__into_inner($a))*) }
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
    use crate::rc::{Id, Owned};
    use crate::test_utils;
    use crate::{msg_send, msg_send_id};

    #[allow(unused)]
    fn test_different_receivers(mut obj: Id<Object, Owned>) {
        unsafe {
            let x = &mut obj;
            let _: () = msg_send![x, mutable1];
            // let _: () = msg_send![x, mutable2];
            let _: () = msg_send![&mut *obj, mutable1];
            let _: () = msg_send![&mut *obj, mutable2];
            #[allow(clippy::needless_borrow)]
            let obj: NonNull<Object> = (&mut *obj).into();
            let _: () = msg_send![obj, mutable1];
            let _: () = msg_send![obj, mutable2];
            let obj: *mut Object = obj.as_ptr();
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
        use crate::rc::Shared;

        let nil: *mut Object = ::core::ptr::null_mut();

        // This result should not be relied on
        let result: Option<Id<Object, Shared>> = unsafe { msg_send_id![nil, description] };
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
        let result: Option<Id<Object, Shared>> =
            unsafe { msg_send_id![nil, multiple: 1u32, arguments: 2i8] };
        assert!(result.is_none());

        // This result should not be relied on
        let result: Option<Id<Object, Shared>> = unsafe { msg_send_id![None, init] };
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
