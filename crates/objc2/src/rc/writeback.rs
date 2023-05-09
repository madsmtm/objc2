//! Support for passing "out"-parameters to `msg_send!` and family.
//!
//! See clang's documentation:
//! <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#passing-to-an-out-parameter-by-writeback>
//!
//! Note: We differ from that in that we do not create a temporary, whoose
//! address we then work on; instead, we just directly reuse the pointer that
//! the user provides (since, if it's a mutable pointer, we know that it's not
//! shared elsewhere in the program, and hence it is safe to modify directly).
use core::mem::ManuallyDrop;
use core::ptr::NonNull;

use crate::encode::__unstable::EncodeConvertArgument;
use crate::rc::Id;
use crate::Message;

// Note the `'static` bound here - this may not be necessary, but I'm unsure
// of the exact requirements, so we better just keep it for now.
impl<T: Message + 'static> EncodeConvertArgument for &mut Id<T> {
    // We use `*mut T` as the inner value instead of `NonNull<T>`, since we
    // want to do debug checking that the value hasn't unexpectedly been
    // overwritten to contain NULL (which is clear UB, but the user might have
    // made a mistake).
    type __Inner = NonNull<*mut T>;

    type __StoredBeforeMessage = (
        // A copy of the argument, so that we can retain it after the message
        // send. Ideally, we'd just work with e.g. `&mut *mut T`, but we can't
        // do that inside the generic context of `MessageArguments::__invoke`.
        Self::__Inner,
        // A pointer to the old value stored in the `Id`, so that we can
        // release if after the message send.
        NonNull<T>,
    );

    #[inline]
    fn __from_declared_param(_inner: Self::__Inner) -> Self {
        todo!("`&mut Id<_>` is not supported in `declare_class!` yet")
    }

    #[inline]
    fn __into_argument(self) -> (Self::__Inner, Self::__StoredBeforeMessage) {
        let ptr: NonNull<Id<T>> = NonNull::from(self);
        // `Id` is `#[repr(transparent)]` over `NonNull`.
        let ptr: NonNull<NonNull<T>> = ptr.cast();

        // SAFETY: The value came from `&mut _`, and we only read a pointer.
        let old: NonNull<T> = unsafe { *ptr.as_ptr() };

        // `NonNull<T>` has the same layout as `*mut T`.
        let ptr: NonNull<*mut T> = ptr.cast();

        (ptr, (ptr, old))
    }

    #[inline]
    unsafe fn __process_after_message_send((ptr, old): Self::__StoredBeforeMessage) {
        // In terms of provenance, we roughly just want to do the following:
        // ```
        // fn do(value: &mut Id<T>) {
        //     let old = value.clone();
        //     msg_send![... value ...];
        //     let _ = value.clone();
        //     drop(old);
        // }
        // ```
        //
        // Which is definitly valid under stacked borrows! See also this
        // playground link for testing something equivalent in Miri:
        // <https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ef8ecfb54a11b9a59ae17cc7edfbef3d>
        //
        //
        //
        // In Objective-C terms, we want to retain the new value and release
        // the old, and importantly, in that order (such that we don't dealloc
        // the value if it didn't change). So something like this:
        // ```
        // fn do(value: &mut Id) {
        //     let old = *value;
        //     msg_send![... value ...];
        //     objc_retain(*value);
        //     objc_release(old);
        // }
        // ```
        //
        // Note that using a mutable `Id<T>` is perfectly sound, since while
        // we may intermittently have a retain count of 2 to the value, after
        // the function returns we're guaranteed to be back to 1.

        // SAFETY: Caller ensures that the pointer is either left as-is, or is
        // safe to retain at this point.
        let new: Option<Id<T>> = unsafe { Id::retain(*ptr.as_ptr()) };
        // We ignore the result of `retain`, since it always returns the same
        // value as was given (and it would just be unnecessary work to write
        // that value back into `ptr` again).
        let _new = ManuallyDrop::new(new);
        #[cfg(debug_assertions)]
        if _new.is_none() {
            panic!("found that NULL was written to `&mut Id<_>`, which is UB! You should handle this with `&mut Option<Id<_>>` instead");
        }

        // SAFETY: The old pointer was valid when it was constructed.
        //
        // If the message send modified `ptr`, they would have left a +1
        // retain count on the old pointer; so either we have +1 from that, or
        // the message send didn't modify the pointer and we instead have +1
        // retain count from the `retain` above.
        let _: Id<T> = unsafe { Id::new_nonnull(old) };
    }
}

impl<T: Message + 'static> EncodeConvertArgument for &mut Option<Id<T>> {
    type __Inner = NonNull<*mut T>;

    type __StoredBeforeMessage = (Self::__Inner, *mut T);

    #[inline]
    fn __from_declared_param(_inner: Self::__Inner) -> Self {
        todo!("`&mut Option<Id<_>>` is not supported in `declare_class!` yet")
    }

    #[inline]
    fn __into_argument(self) -> (Self::__Inner, Self::__StoredBeforeMessage) {
        let ptr: NonNull<Option<Id<T>>> = NonNull::from(self);
        // `Option<Id<T>>` has the same memory layout as `*mut T`.
        let ptr: NonNull<*mut T> = ptr.cast();
        // SAFETY: Same as for `&mut Id`
        let old: *mut T = unsafe { *ptr.as_ptr() };

        (ptr, (ptr, old))
    }

    #[inline]
    unsafe fn __process_after_message_send((ptr, old): Self::__StoredBeforeMessage) {
        // SAFETY: Same as for `&mut Id`
        let new: Option<Id<T>> = unsafe { Id::retain(*ptr.as_ptr()) };
        let _ = ManuallyDrop::new(new);

        // SAFETY: Same as for `&mut Id`
        //
        // Note: We explicitly keep the `if old == nil { objc_release(old) }`
        // check, since we expect that the user would often do:
        //
        // ```
        // let mut value = None
        // do(&mut value);
        // ```
        //
        // And in that case, we can elide the `objc_release`!
        let _: Option<Id<T>> = unsafe { Id::new(old) };
    }
}

// Note: For `Option<&mut ...>` we explicitly want to do the `if Some` checks
// before anything else, since whether `None` or `Some` was passed is often
// known at compile-time, and for the `None` case it would be detrimental to
// have extra `retain/release` calls here.

impl<T: Message + 'static> EncodeConvertArgument for Option<&mut Id<T>> {
    type __Inner = Option<NonNull<*mut T>>;

    type __StoredBeforeMessage = Option<(NonNull<*mut T>, NonNull<T>)>;

    #[inline]
    fn __from_declared_param(_inner: Self::__Inner) -> Self {
        todo!("`Option<&mut Id<_>>` is not supported in `declare_class!` yet")
    }

    #[inline]
    fn __into_argument(self) -> (Self::__Inner, Self::__StoredBeforeMessage) {
        if let Some(this) = self {
            let (ptr, stored) = this.__into_argument();
            (Some(ptr), Some(stored))
        } else {
            (None, None)
        }
    }

    #[inline]
    unsafe fn __process_after_message_send(stored: Self::__StoredBeforeMessage) {
        if let Some(stored) = stored {
            // SAFETY: Checked by caller
            unsafe { <&mut Id<T>>::__process_after_message_send(stored) };
        }
    }
}

impl<T: Message + 'static> EncodeConvertArgument for Option<&mut Option<Id<T>>> {
    type __Inner = Option<NonNull<*mut T>>;

    type __StoredBeforeMessage = Option<(NonNull<*mut T>, *mut T)>;

    #[inline]
    fn __from_declared_param(_inner: Self::__Inner) -> Self {
        todo!("`Option<&mut Option<Id<_>>>` is not supported in `declare_class!` yet")
    }

    #[inline]
    fn __into_argument(self) -> (Self::__Inner, Self::__StoredBeforeMessage) {
        if let Some(this) = self {
            let (ptr, stored) = this.__into_argument();
            (Some(ptr), Some(stored))
        } else {
            (None, None)
        }
    }

    #[inline]
    unsafe fn __process_after_message_send(stored: Self::__StoredBeforeMessage) {
        if let Some(stored) = stored {
            // SAFETY: Checked by caller
            unsafe { <&mut Option<Id<T>>>::__process_after_message_send(stored) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rc::{__RcTestObject, __ThreadTestData, autoreleasepool};
    use crate::{msg_send, msg_send_id, ClassType};

    #[test]
    fn test_bool_error() {
        let mut expected = __ThreadTestData::current();

        fn bool_error(should_error: bool, error: Option<&mut Option<Id<__RcTestObject>>>) {
            let cls = __RcTestObject::class();
            let did_succeed: bool =
                unsafe { msg_send![cls, boolAndShouldError: should_error, error: error] };
            assert_ne!(should_error, did_succeed);
        }

        bool_error(false, None);
        bool_error(true, None);
        expected.assert_current();

        fn helper(
            expected: &mut __ThreadTestData,
            should_error: bool,
            mut error: Option<Id<__RcTestObject>>,
        ) {
            std::dbg!(should_error, &error);
            autoreleasepool(|_| {
                bool_error(should_error, Some(&mut error));
                if should_error {
                    expected.alloc += 1;
                    expected.init += 1;
                    expected.autorelease += 1;
                }
                expected.assert_current();
            });

            if should_error {
                expected.release += 1;
            }
            expected.assert_current();

            if error.is_some() {
                expected.release += 1;
                expected.dealloc += 1;
            }
            drop(error);
            expected.assert_current();
        }

        helper(&mut expected, false, None);

        expected.retain += 1;
        helper(&mut expected, true, None);

        expected.alloc += 1;
        expected.init += 1;
        expected.retain += 1;
        expected.release += 1;
        helper(&mut expected, false, Some(__RcTestObject::new()));

        expected.alloc += 1;
        expected.init += 1;
        expected.retain += 1;
        expected.release += 1;
        expected.dealloc += 1;
        helper(&mut expected, true, Some(__RcTestObject::new()));
    }

    #[test]
    #[cfg_attr(
        any(
            not(debug_assertions),
            all(not(target_pointer_width = "64"), feature = "catch-all")
        ),
        ignore = "invokes UB which is only caught with debug_assertions"
    )]
    #[should_panic = "found that NULL was written to `&mut Id<_>`, which is UB! You should handle this with `&mut Option<Id<_>>` instead"]
    fn test_debug_check_ub() {
        let cls = __RcTestObject::class();
        let mut param: Id<_> = __RcTestObject::new();
        let _: () = unsafe { msg_send![cls, outParamNull: &mut param] };
    }

    // TODO: Fix this in release mode with Apple's runtime
    const AUTORELEASE_SKIPPED: bool = cfg!(feature = "gnustep-1-7");

    #[test]
    fn test_id_interaction() {
        let mut expected = __ThreadTestData::current();
        let cls = __RcTestObject::class();

        let mut err: Id<__RcTestObject> = __RcTestObject::new();
        expected.alloc += 1;
        expected.init += 1;
        expected.assert_current();

        autoreleasepool(|_| {
            let obj: Option<Id<__RcTestObject>> =
                unsafe { msg_send_id![cls, idAndShouldError: false, error: &mut err] };
            expected.alloc += 1;
            expected.init += 1;
            if !AUTORELEASE_SKIPPED {
                expected.autorelease += 1;
                expected.retain += 1;
            }

            expected.retain += 1;
            expected.release += 1;
            expected.assert_current();

            drop(obj);
            expected.release += 1;
            if AUTORELEASE_SKIPPED {
                expected.dealloc += 1;
            }
            expected.assert_current();
        });
        if !AUTORELEASE_SKIPPED {
            expected.release += 1;
            expected.dealloc += 1;
        }
        expected.assert_current();

        drop(err);
        expected.release += 1;
        expected.dealloc += 1;
        expected.assert_current();
    }
}
