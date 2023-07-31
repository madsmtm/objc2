use core::mem;
use core::mem::ManuallyDrop;
use core::ptr::{self, NonNull};

use crate::encode::__unstable::{
    EncodeArguments, EncodeConvertArgument, EncodeConvertReturn, EncodeReturn,
};
use crate::encode::{Encode, RefEncode};
use crate::mutability::IsMutable;
use crate::rc::Id;
use crate::runtime::{AnyClass, AnyObject, Imp, Sel};
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
    obj: Option<&AnyObject>,
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
    panic!("messsaging {sel} to nil")
}

#[cfg(debug_assertions)]
#[track_caller]
fn panic_verify(cls: &AnyClass, sel: Sel, err: crate::runtime::VerificationError) -> ! {
    panic!(
        "invalid message send to {}[{cls} {sel}]: {err}",
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
/// that, almost always with [`RefEncode::ENCODING_REF`] being
/// [`Encoding::Object`].
///
/// This can be implemented for unsized (`!Sized`) types, but the intention is
/// not to support dynamically sized types like slices, only `extern type`s
/// (which is currently unstable).
///
/// [`Encoding::Object`]: crate::Encoding::Object
///
///
/// # `Drop` interaction
///
/// If the inner type implements [`Drop`], that implementation will very
/// likely not be called, since there is no way to ensure that the Objective-C
/// runtime will do so. If you need to run some code when the object is
/// destroyed, implement the `dealloc` method instead.
///
/// The [`declare_class!`] macro does this for you, but the [`extern_class!`]
/// macro fundamentally cannot.
///
/// [`declare_class!`]: crate::declare_class
/// [`extern_class!`]: crate::extern_class
///
///
/// # Safety
///
/// The type must represent an Objective-C object, meaning it:
/// - Must be valid to reinterpret as [`runtime::AnyObject`][`AnyObject`].
/// - Must be able to be the receiver of an Objective-C message sent with
///   [`objc_msgSend`] or similar.
/// - Must respond to the standard memory management `retain`, `release` and
///   `autorelease` messages.
/// - Must support weak references. (In the future we should probably make a
///   new trait for this, for example `NSTextView` only supports weak
///   references on macOS 10.12 or above).
///
/// [`objc_msgSend`]: https://developer.apple.com/documentation/objectivec/1456712-objc_msgsend
///
///
/// # Example
///
/// ```
/// use objc2::runtime::NSObject;
/// use objc2::{Encoding, Message, RefEncode};
///
/// #[repr(C)]
/// struct MyObject {
///     // This has the exact same layout as `NSObject`
///     inner: NSObject
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
/// // And `Id<MyObject>` can now be constructed.
/// ```
pub unsafe trait Message: RefEncode {}

// TODO: Make this fully private
pub(crate) mod private {
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
        R: EncodeConvertReturn,
    {
        let this = self.__as_raw_receiver();
        #[cfg(debug_assertions)]
        {
            // SAFETY: Caller ensures only valid or NULL pointers.
            let obj = unsafe { this.as_ref() };
            msg_send_check(obj, sel, A::ENCODINGS, &R::__Inner::ENCODING_RETURN);
        }
        unsafe { EncodeConvertReturn::__from_return(send_unverified(this, sel, args)) }
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
        R: EncodeConvertReturn,
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
        unsafe {
            EncodeConvertReturn::__from_return(send_super_unverified(this, superclass, sel, args))
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
        R: EncodeConvertReturn,
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

impl<'a, T: ?Sized + IsMutable> private::Sealed for &'a mut Id<T> {}
unsafe impl<'a, T: ?Sized + IsMutable> MessageReceiver for &'a mut Id<T> {
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
        unsafe impl<$($t: EncodeConvertArgument),*> MessageArguments for ($($t,)*) {
            #[inline]
            unsafe fn __invoke<R: EncodeReturn>(imp: Imp, obj: *mut AnyObject, sel: Sel, ($($a,)*): Self) -> R {
                $(let $a = EncodeConvertArgument::__into_argument($a);)*

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
                    // once. See `src/rc/writeback.rs` for details.
                    unsafe { <$t as EncodeConvertArgument>::__process_after_message_send($a.1) };
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
