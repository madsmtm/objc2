use core::ptr;

use crate::encode::Encode;
use crate::rc::{Allocated, Id};
use crate::runtime::{AnyClass, AnyObject, MessageReceiver, Sel};
use crate::{sel, Message};

use super::{Alloc, ConvertArguments, CopyOrMutCopy, Init, New, Other, TupleExtender};

pub trait MsgSendId<T, U> {
    #[track_caller]
    unsafe fn send_message_id<A: ConvertArguments, R: MaybeUnwrap<Input = U>>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> R;

    /// Add an extra error argument to the argument list, call
    /// `send_message_id` with that, and return an error if one occurred.
    #[inline]
    #[track_caller]
    unsafe fn send_message_id_error<A, E>(obj: T, sel: Sel, args: A) -> Result<U, Id<E>>
    where
        *mut *mut E: Encode,
        A: TupleExtender<*mut *mut E>,
        <A as TupleExtender<*mut *mut E>>::PlusOneArgument: ConvertArguments,
        E: Message,
        Option<U>: MaybeUnwrap<Input = U>,
    {
        let mut err: *mut E = ptr::null_mut();
        let args = args.add_argument(&mut err);
        let res: Option<U> = unsafe { Self::send_message_id(obj, sel, args) };
        // As per the Cocoa documentation:
        // > Success or failure is indicated by the return value of the
        // > method. Although Cocoa methods that indirectly return error
        // > objects in the Cocoa error domain are guaranteed to return such
        // > objects if the method indicates failure by directly returning
        // > `nil` or `NO`, you should always check that the return value is
        // > `nil` or `NO` before attempting to do anything with the `NSError`
        // > object.
        if let Some(res) = res {
            // In this case, the error is likely not created. If it is, it is
            // autoreleased anyhow, so it would be a waste to retain and
            // release it here.
            Ok(res)
        } else {
            // In this case, the error has very likely been created, but has
            // been autoreleased (as is common for "out parameters", see
            // `src/__macro_helpers/writeback.rs`). Hence we need to retain it
            // if we want it to live across autorelease pools.
            //
            // SAFETY: The message send is guaranteed to populate the error
            // object, or leave it as NULL. The error is shared, and all
            // holders of the error know this, so is safe to retain.
            Err(unsafe { encountered_error(err) })
        }
    }
}

// Marked `cold` to tell the optimizer that errors are comparatively rare.
// And intentionally not inlined, for much the same reason.
#[cold]
#[track_caller]
unsafe fn encountered_error<E: Message>(err: *mut E) -> Id<E> {
    // SAFETY: Ensured by caller
    unsafe { Id::retain(err) }.expect("error parameter should be set if the method returns NULL")
}

impl<T: MessageReceiver, U: ?Sized + Message> MsgSendId<T, Id<U>> for New {
    #[inline]
    unsafe fn send_message_id<A: ConvertArguments, R: MaybeUnwrap<Input = Id<U>>>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> R {
        let ptr = obj.__as_raw_receiver();
        // SAFETY: Checked by caller
        let obj = unsafe { MessageReceiver::send_message(ptr, sel, args) };
        // SAFETY: The selector is `new`, so this has +1 retain count
        let obj = unsafe { Id::new(obj) };

        // SAFETY: The object is still valid after a message send to a `new`
        // method - it would not be if the method was `init`.
        R::maybe_unwrap::<Self>(obj, (unsafe { ptr.as_ref() }, sel))
    }
}

impl<T: ?Sized + Message> MsgSendId<&'_ AnyClass, Allocated<T>> for Alloc {
    #[inline]
    unsafe fn send_message_id<A: ConvertArguments, R: MaybeUnwrap<Input = Allocated<T>>>(
        cls: &AnyClass,
        sel: Sel,
        args: A,
    ) -> R {
        // SAFETY: Checked by caller
        let obj = unsafe { MessageReceiver::send_message(cls, sel, args) };
        // SAFETY: The selector is `alloc`, so this has +1 retain count
        let obj = unsafe { Allocated::new(obj) };
        R::maybe_unwrap::<Self>(obj, (cls, sel))
    }
}

impl<T: ?Sized + Message> MsgSendId<Option<Allocated<T>>, Id<T>> for Init {
    #[inline]
    unsafe fn send_message_id<A: ConvertArguments, R: MaybeUnwrap<Input = Id<T>>>(
        obj: Option<Allocated<T>>,
        sel: Sel,
        args: A,
    ) -> R {
        let ptr = Allocated::option_into_ptr(obj);
        // SAFETY: `ptr` may be null here, but that's fine since the return
        // is `*mut T`, which is one of the few types where messages to nil is
        // allowed.
        //
        // We do this for efficiency, to avoid having a branch that the user
        // did not intend after every `alloc`.
        let obj = unsafe { MessageReceiver::send_message(ptr, sel, args) };
        // SAFETY: The selector is `init`, so this has +1 retain count
        let obj = unsafe { Id::new(obj) };
        R::maybe_unwrap::<Self>(obj, (ptr.cast(), sel))
    }
}

impl<T: MessageReceiver, U: ?Sized + Message> MsgSendId<T, Id<U>> for CopyOrMutCopy {
    #[inline]
    unsafe fn send_message_id<A: ConvertArguments, R: MaybeUnwrap<Input = Id<U>>>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> R {
        // SAFETY: Checked by caller
        let obj = unsafe { MessageReceiver::send_message(obj, sel, args) };
        // SAFETY: The selector is `copy` or `mutableCopy`, so this has +1
        // retain count
        let obj = unsafe { Id::new(obj) };
        R::maybe_unwrap::<Self>(obj, ())
    }
}

impl<T: MessageReceiver, U: Message> MsgSendId<T, Id<U>> for Other {
    #[inline]
    unsafe fn send_message_id<A: ConvertArguments, R: MaybeUnwrap<Input = Id<U>>>(
        obj: T,
        sel: Sel,
        args: A,
    ) -> R {
        let ptr = obj.__as_raw_receiver();
        // SAFETY: Checked by caller
        let obj = unsafe { MessageReceiver::send_message(ptr, sel, args) };
        // All code between the message send and the `retain_autoreleased`
        // must be able to be optimized away for this to work.

        // SAFETY: The selector is not `new`, `alloc`, `init`, `copy` nor
        // `mutableCopy`, so the object must be manually retained.
        let obj = unsafe { Id::retain_autoreleased(obj) };

        // SAFETY: The object is still valid after a message send to a
        // normal method - it would not be if the method was `init`.
        R::maybe_unwrap::<Self>(obj, (unsafe { ptr.as_ref() }, sel))
    }
}

pub trait MaybeUnwrap {
    type Input;
    #[track_caller]
    fn maybe_unwrap<'a, F: MsgSendIdFailed<'a>>(obj: Option<Self::Input>, args: F::Args) -> Self;
}

impl<T: ?Sized> MaybeUnwrap for Option<Id<T>> {
    type Input = Id<T>;

    #[inline]
    fn maybe_unwrap<'a, F: MsgSendIdFailed<'a>>(obj: Option<Id<T>>, _args: F::Args) -> Self {
        obj
    }
}

impl<T: ?Sized> MaybeUnwrap for Id<T> {
    type Input = Id<T>;

    #[inline]
    fn maybe_unwrap<'a, F: MsgSendIdFailed<'a>>(obj: Option<Id<T>>, args: F::Args) -> Self {
        match obj {
            Some(obj) => obj,
            None => F::failed(args),
        }
    }
}

impl<T: ?Sized> MaybeUnwrap for Option<Allocated<T>> {
    type Input = Allocated<T>;

    #[inline]
    fn maybe_unwrap<'a, F: MsgSendIdFailed<'a>>(obj: Option<Allocated<T>>, _args: F::Args) -> Self {
        obj
    }
}

impl<T: ?Sized> MaybeUnwrap for Allocated<T> {
    type Input = Allocated<T>;

    #[inline]
    fn maybe_unwrap<'a, F: MsgSendIdFailed<'a>>(obj: Option<Allocated<T>>, args: F::Args) -> Self {
        match obj {
            Some(obj) => obj,
            None => F::failed(args),
        }
    }
}

// Note: It would have been much easier to do this kind of thing using
// closures, but then `track_caller` doesn't work properly!
//
// Also note: This behavior (that #[method_id(...)] always unwraps instead of
// using `unwrap_unchecked`) is relied upon by `header-translator` for
// soundness, see e.g. `parse_property_return`.
pub trait MsgSendIdFailed<'a> {
    type Args;

    #[track_caller]
    fn failed(args: Self::Args) -> !;
}

impl<'a> MsgSendIdFailed<'a> for New {
    type Args = (Option<&'a AnyObject>, Sel);

    #[cold]
    fn failed((obj, sel): Self::Args) -> ! {
        if let Some(obj) = obj {
            let cls = obj.class();
            if cls.is_metaclass() {
                if sel == sel!(new) {
                    panic!("failed creating new instance of {cls}")
                } else {
                    panic!("failed creating new instance using +[{cls} {sel}]")
                }
            } else {
                panic!("unexpected NULL returned from -[{cls} {sel}]")
            }
        } else {
            panic!("unexpected NULL {sel}; receiver was NULL");
        }
    }
}

impl<'a> MsgSendIdFailed<'a> for Alloc {
    type Args = (&'a AnyClass, Sel);

    #[cold]
    fn failed((cls, sel): Self::Args) -> ! {
        if sel == sel!(alloc) {
            panic!("failed allocating {cls}")
        } else {
            panic!("failed allocating with +[{cls} {sel}]")
        }
    }
}

impl MsgSendIdFailed<'_> for Init {
    type Args = (*const AnyObject, Sel);

    #[cold]
    fn failed((ptr, sel): Self::Args) -> ! {
        if ptr.is_null() {
            panic!("failed allocating object")
        } else {
            // We can't really display a more descriptive message here since the
            // object is consumed by `init` and may not be valid any more.
            if sel == sel!(init) {
                panic!("failed initializing object")
            } else {
                panic!("failed initializing object with -{sel}")
            }
        }
    }
}

impl MsgSendIdFailed<'_> for CopyOrMutCopy {
    type Args = ();

    #[cold]
    fn failed(_: Self::Args) -> ! {
        panic!("failed copying object")
    }
}

impl<'a> MsgSendIdFailed<'a> for Other {
    type Args = (Option<&'a AnyObject>, Sel);

    #[cold]
    fn failed((obj, sel): Self::Args) -> ! {
        if let Some(obj) = obj {
            let cls = obj.class();
            panic!(
                "unexpected NULL returned from {}[{cls} {sel}]",
                if cls.is_metaclass() { "+" } else { "-" },
            )
        } else {
            panic!("unexpected NULL {sel}; receiver was NULL");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::ptr;

    use crate::rc::{__RcTestObject, __ThreadTestData};
    use crate::runtime::{AnyObject, NSObject, NSZone};
    use crate::{class, msg_send_id, ClassType};

    #[test]
    fn test_new() {
        let _obj: Id<AnyObject> = unsafe { msg_send_id![NSObject::class(), new] };
        let _obj: Option<Id<AnyObject>> = unsafe { msg_send_id![NSObject::class(), new] };
    }

    #[test]
    fn test_new_not_on_class() {
        let mut expected = __ThreadTestData::current();
        let obj = __RcTestObject::new();

        let _obj: Id<AnyObject> = unsafe { msg_send_id![&obj, newMethodOnInstance] };
        let _obj: Option<Id<AnyObject>> = unsafe { msg_send_id![&obj, newMethodOnInstance] };
        expected.alloc += 3;
        expected.init += 3;
        expected.assert_current();
    }

    #[test]
    // newScriptingObjectOfClass only available on macOS
    #[cfg_attr(not(all(feature = "apple", target_os = "macos")), ignore)]
    fn test_new_with_args() {
        let mut expected = __ThreadTestData::current();

        let object_class = __RcTestObject::class();
        let key: Id<AnyObject> = unsafe { msg_send_id![class!(NSString), new] };
        let contents_value: *const AnyObject = ptr::null();
        let properties: Id<AnyObject> = unsafe { msg_send_id![class!(NSDictionary), new] };

        let _obj: Option<Id<AnyObject>> = unsafe {
            msg_send_id![
                NSObject::class(),
                newScriptingObjectOfClass: object_class,
                forValueForKey: &*key,
                withContentsValue: contents_value,
                properties: &*properties,
            ]
        };
        expected.alloc += 1;
        expected.init += 1;
        expected.assert_current();
    }

    #[test]
    fn test_macro_alloc() {
        let mut expected = __ThreadTestData::current();
        let cls = __RcTestObject::class();

        let obj: Allocated<__RcTestObject> = unsafe { msg_send_id![cls, alloc] };
        expected.alloc += 1;
        expected.assert_current();

        drop(obj);
        expected.release += 1;
        expected.dealloc += 1;
        expected.assert_current();
    }

    #[test]
    fn test_alloc_with_zone() {
        let mut expected = __ThreadTestData::current();
        let cls = __RcTestObject::class();

        let zone: *const NSZone = ptr::null();
        let _obj: Allocated<__RcTestObject> = unsafe { msg_send_id![cls, allocWithZone: zone] };
        expected.alloc += 1;
        expected.assert_current();
    }

    #[test]
    fn test_macro_init() {
        let mut expected = __ThreadTestData::current();
        let cls = __RcTestObject::class();

        let obj: Option<Allocated<__RcTestObject>> = unsafe { msg_send_id![cls, alloc] };
        expected.alloc += 1;
        // Don't check allocation error
        let _obj: Id<__RcTestObject> = unsafe { msg_send_id![obj, init] };
        expected.init += 1;
        expected.assert_current();

        let obj: Option<Allocated<__RcTestObject>> = unsafe { msg_send_id![cls, alloc] };
        expected.alloc += 1;
        // Check allocation error before init
        let obj = obj.unwrap();
        let _obj: Id<__RcTestObject> = unsafe { msg_send_id![Some(obj), init] };
        expected.init += 1;
        expected.assert_current();
    }

    #[test]
    fn test_macro() {
        let mut expected = __ThreadTestData::current();
        let cls = __RcTestObject::class();
        crate::rc::autoreleasepool(|_| {
            let _obj: Id<__RcTestObject> = unsafe { msg_send_id![cls, new] };
            expected.alloc += 1;
            expected.init += 1;
            expected.assert_current();

            let obj = unsafe { msg_send_id![cls, alloc] };
            expected.alloc += 1;
            expected.assert_current();

            let obj: Id<__RcTestObject> = unsafe { msg_send_id![obj, init] };
            expected.init += 1;
            expected.assert_current();

            let _copy: Id<__RcTestObject> = unsafe { msg_send_id![&obj, copy] };
            expected.copy += 1;
            expected.alloc += 1;
            expected.init += 1;
            expected.assert_current();

            let _mutable_copy: Id<__RcTestObject> = unsafe { msg_send_id![&obj, mutableCopy] };
            expected.mutable_copy += 1;
            expected.alloc += 1;
            expected.init += 1;
            expected.assert_current();

            let _self: Id<__RcTestObject> = unsafe { msg_send_id![&obj, self] };
            expected.retain += 1;
            expected.assert_current();

            let _desc: Option<Id<__RcTestObject>> = unsafe { msg_send_id![&obj, description] };
            expected.assert_current();
        });
        expected.release += 5;
        expected.dealloc += 4;
        expected.assert_current();
    }

    #[test]
    #[should_panic = "failed creating new instance of NSValue"]
    // GNUStep instead returns an invalid instance that panics on accesses
    #[cfg_attr(feature = "gnustep-1-7", ignore)]
    fn new_nsvalue_fails() {
        let _val: Id<AnyObject> = unsafe { msg_send_id![class!(NSValue), new] };
    }

    #[test]
    #[should_panic = "failed creating new instance using +[__RcTestObject newReturningNull]"]
    fn test_new_with_null() {
        let _obj: Id<__RcTestObject> =
            unsafe { msg_send_id![__RcTestObject::class(), newReturningNull] };
    }

    #[test]
    #[should_panic = "unexpected NULL returned from -[__RcTestObject newMethodOnInstanceNull]"]
    fn test_new_any_with_null() {
        let obj = __RcTestObject::new();
        let _obj: Id<AnyObject> = unsafe { msg_send_id![&obj, newMethodOnInstanceNull] };
    }

    #[test]
    #[should_panic = "unexpected NULL newMethodOnInstance; receiver was NULL"]
    #[cfg(not(debug_assertions))] // Does NULL receiver checks
    fn test_new_any_with_null_receiver() {
        let obj: *const NSObject = ptr::null();
        let _obj: Id<AnyObject> = unsafe { msg_send_id![obj, newMethodOnInstance] };
    }

    #[test]
    #[should_panic = "failed allocating with +[__RcTestObject allocReturningNull]"]
    fn test_alloc_with_null() {
        let _obj: Allocated<__RcTestObject> =
            unsafe { msg_send_id![__RcTestObject::class(), allocReturningNull] };
    }

    #[test]
    #[should_panic = "failed initializing object with -initReturningNull"]
    fn test_init_with_null() {
        let obj: Option<Allocated<__RcTestObject>> =
            unsafe { msg_send_id![__RcTestObject::class(), alloc] };
        let _obj: Id<__RcTestObject> = unsafe { msg_send_id![obj, initReturningNull] };
    }

    #[test]
    #[should_panic = "failed allocating object"]
    #[cfg(not(debug_assertions))] // Does NULL receiver checks
    fn test_init_with_null_receiver() {
        let obj: Option<Allocated<__RcTestObject>> = None;
        let _obj: Id<__RcTestObject> = unsafe { msg_send_id![obj, init] };
    }

    #[test]
    #[should_panic = "failed copying object"]
    fn test_copy_with_null() {
        let obj = __RcTestObject::new();
        let _obj: Id<__RcTestObject> = unsafe { msg_send_id![&obj, copyReturningNull] };
    }

    #[test]
    #[should_panic = "unexpected NULL returned from -[__RcTestObject methodReturningNull]"]
    fn test_normal_with_null() {
        let obj = __RcTestObject::new();
        let _obj: Id<__RcTestObject> = unsafe { msg_send_id![&obj, methodReturningNull] };
    }

    #[test]
    #[should_panic = "unexpected NULL returned from -[__RcTestObject aMethod:]"]
    fn test_normal_with_param_and_null() {
        let obj = __RcTestObject::new();
        let _obj: Id<__RcTestObject> = unsafe { msg_send_id![&obj, aMethod: false] };
    }

    #[test]
    #[should_panic = "unexpected NULL description; receiver was NULL"]
    #[cfg(not(debug_assertions))] // Does NULL receiver checks
    fn test_normal_with_null_receiver() {
        let obj: *const NSObject = ptr::null();
        let _obj: Id<AnyObject> = unsafe { msg_send_id![obj, description] };
    }

    mod test_trait_disambugated {
        use super::*;

        trait Abc {
            fn send_message_id() {}
        }

        impl<T> Abc for T {}

        #[test]
        fn test_macro_still_works() {
            let cls = class!(NSObject);
            let _obj: Id<AnyObject> = unsafe { msg_send_id![cls, new] };
        }
    }
}
