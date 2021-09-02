use alloc::string::{String, ToString};
use core::fmt;
use core::mem;
use std::error::Error;

use crate::runtime::{Class, Imp, Object, Sel};
use crate::{Encode, EncodeArguments, RefEncode};

#[cfg(feature = "exception")]
macro_rules! objc_try {
    ($b:block) => {
        $crate::exception::catch_exception(|| $b).map_err(|exception| {
            use alloc::borrow::ToOwned;
            if exception.is_null() {
                MessageError("Uncaught exception nil".to_owned())
            } else {
                MessageError(alloc::format!("Uncaught exception {:?}", &**exception))
            }
        })
    };
}

#[cfg(not(feature = "exception"))]
macro_rules! objc_try {
    ($b:block) => {
        Ok($b)
    };
}

mod verify;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "apple/mod.rs"]
mod platform;
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
#[path = "gnustep.rs"]
mod platform;

use self::platform::{send_super_unverified, send_unverified};
use self::verify::{verify_message_signature, VerificationError};

/// Specifies the superclass of an instance.
#[repr(C)]
struct Super {
    /// Specifies an instance of a class.
    pub receiver: *mut Object,
    /// Specifies the particular superclass of the instance to message.
    pub superclass: *const Class,
}

/// This trait marks types that can be sent Objective-C messages.
///
/// Examples include objects, classes, and blocks.
///
/// Implementing this allows using pointers and references to the type as the
/// receiver (first argument) in the [`msg_send!`][`crate::msg_send`] macro.
///
/// # Safety
///
/// The type must implement [`RefEncode`] and adhere to the safety guidelines
/// therein.
///
/// A pointer to the type must be able to be the receiver of an Objective-C
/// message sent with [`objc_msgSend`] or similar.
///
/// [`objc_msgSend`]: https://developer.apple.com/documentation/objectivec/1456712-objc_msgsend
pub unsafe trait Message: RefEncode {
    /**
    Sends a message to self with the given selector and arguments.

    The correct version of `objc_msgSend` will be chosen based on the
    return type. For more information, see Apple's documentation:
    <https://developer.apple.com/library/mac/documentation/Cocoa/Reference/ObjCRuntimeRef/index.html#//apple_ref/doc/uid/TP40001418-CH1g-88778>

    If the selector is known at compile-time, it is recommended to use the
    `msg_send!` macro rather than this method.
    */
    unsafe fn send_message<A, R>(&self, sel: Sel, args: A) -> Result<R, MessageError>
    where
        Self: Sized,
        A: MessageArguments + EncodeArguments,
        R: Encode,
    {
        send_message(self, sel, args)
    }

    /**
    Verifies that the argument and return types match the encoding of the
    method for the given selector.

    This will look up the encoding of the method for the given selector, `sel`,
    and return a [`MessageError`] if any encodings differ for the arguments `A`
    and return type `R`.

    # Example
    ``` no_run
    # use objc::{class, msg_send, sel};
    # use objc::runtime::{BOOL, Class, Object};
    # use objc::Message;
    let obj: &Object;
    # obj = unsafe { msg_send![class!(NSObject), new] };
    let sel = sel!(isKindOfClass:);
    // Verify isKindOfClass: takes one Class and returns a BOOL
    let result = obj.verify_message::<(&Class,), BOOL>(sel);
    assert!(result.is_ok());
    ```
    */
    fn verify_message<A, R>(&self, sel: Sel) -> Result<(), MessageError>
    where
        Self: Sized,
        A: EncodeArguments,
        R: Encode,
    {
        let obj = unsafe { &*(self as *const _ as *const Object) };
        verify_message_signature::<A, R>(obj.class(), sel).map_err(MessageError::from)
    }
}

unsafe impl Message for Object {}

unsafe impl Message for Class {}

/// Types that may be used as the arguments of an Objective-C message.
pub trait MessageArguments: Sized {
    /// Invoke an [`Imp`] with the given object, selector, and arguments.
    ///
    /// This method is the primitive used when sending messages and should not
    /// be called directly; instead, use the `msg_send!` macro or, in cases
    /// with a dynamic selector, the [`Message::send_message`] method.
    unsafe fn invoke<R>(imp: Imp, obj: *mut Object, sel: Sel, args: Self) -> R;
}

macro_rules! message_args_impl {
    ($($a:ident : $t:ident),*) => (
        impl<$($t),*> MessageArguments for ($($t,)*) {
            unsafe fn invoke<R>(imp: Imp, obj: *mut Object, sel: Sel, ($($a,)*): Self) -> R {
                let imp: unsafe extern fn(*mut Object, Sel $(, $t)*) -> R =
                    mem::transmute(imp);
                imp(obj, sel $(, $a)*)
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

/**
An error encountered while attempting to send a message.

Currently, an error may be returned in two cases:

* an Objective-C exception is thrown and the `exception` feature is enabled
* the encodings of the arguments do not match the encoding of the method
  and the `verify_message` feature is enabled
*/
#[derive(Debug)]
pub struct MessageError(String);

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Error for MessageError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl<'a> From<VerificationError<'a>> for MessageError {
    fn from(err: VerificationError<'_>) -> MessageError {
        MessageError(err.to_string())
    }
}

#[doc(hidden)]
#[cfg_attr(feature = "verify_message", inline(always))]
pub unsafe fn send_message<T, A, R>(obj: *const T, sel: Sel, args: A) -> Result<R, MessageError>
where
    T: Message,
    A: MessageArguments + EncodeArguments,
    R: Encode,
{
    #[cfg(feature = "verify_message")]
    {
        let cls = if obj.is_null() {
            return Err(VerificationError::NilReceiver(sel).into());
        } else {
            (*(obj as *const Object)).class()
        };

        verify_message_signature::<A, R>(cls, sel)?;
    }
    send_unverified(obj, sel, args)
}

#[doc(hidden)]
#[cfg_attr(feature = "verify_message", inline(always))]
pub unsafe fn send_super_message<T, A, R>(
    obj: *const T,
    superclass: &Class,
    sel: Sel,
    args: A,
) -> Result<R, MessageError>
where
    T: Message,
    A: MessageArguments + EncodeArguments,
    R: Encode,
{
    #[cfg(feature = "verify_message")]
    {
        if obj.is_null() {
            return Err(VerificationError::NilReceiver(sel).into());
        }
        verify_message_signature::<A, R>(superclass, sel)?;
    }
    send_super_unverified(obj, superclass, sel, args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils;

    #[test]
    fn test_send_message() {
        let obj = test_utils::custom_object();
        let result: u32 = unsafe {
            let _: () = msg_send![obj, setFoo:4u32];
            msg_send![obj, foo]
        };
        assert!(result == 4);
    }

    #[test]
    fn test_send_message_stret() {
        let obj = test_utils::custom_object();
        let result: test_utils::CustomStruct = unsafe { msg_send![obj, customStruct] };
        let expected = test_utils::CustomStruct {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
        };
        assert!(result == expected);
    }

    #[cfg(not(feature = "verify_message"))]
    #[test]
    fn test_send_message_nil() {
        let nil: *mut Object = ::core::ptr::null_mut();
        let result: usize = unsafe { msg_send![nil, hash] };
        assert!(result == 0);

        let result: *mut Object = unsafe { msg_send![nil, description] };
        assert!(result.is_null());

        let result: f64 = unsafe { msg_send![nil, doubleValue] };
        assert!(result == 0.0);
    }

    #[test]
    fn test_send_message_super() {
        let obj = test_utils::custom_subclass_object();
        let superclass = test_utils::custom_class();
        unsafe {
            let _: () = msg_send![obj, setFoo:4u32];
            let foo: u32 = msg_send![super(obj, superclass), foo];
            assert!(foo == 4);

            // The subclass is overriden to return foo + 2
            let foo: u32 = msg_send![obj, foo];
            assert!(foo == 6);
        }
    }

    #[test]
    fn test_verify_message() {
        let obj = test_utils::custom_object();
        assert!(obj.verify_message::<(), u32>(sel!(foo)).is_ok());
        assert!(obj.verify_message::<(u32,), ()>(sel!(setFoo:)).is_ok());

        // Incorrect types
        assert!(obj.verify_message::<(), u64>(sel!(setFoo:)).is_err());
        // Unimplemented selector
        assert!(obj.verify_message::<(u32,), ()>(sel!(setFoo)).is_err());
    }
}
