//! Support for passing "out"-parameters to `define_class!`, `msg_send!` etc.
//!
//! See clang's documentation:
//! <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#passing-to-an-out-parameter-by-writeback>
//!
//! The routines in here convert from `T *__strong *` (which, ABI-wise is
//! analogous to `&mut Retained<T>`) to `T *__autoreleasing *`, and back.
//!
//! Note: We differ from Clang in that we do not create a temporary, whose
//! address we then work on; instead, we directly reuse the pointer that the
//! user provides (since, if it's a mutable pointer, we know that it's not
//! shared elsewhere in the program, and hence it is safe to modify directly).
//!
//! Another important consideration is unwinding; I haven't researched how
//! Clang handles that, but the correct thing is to do the writeback
//! retain/release dance regardless of whether the function unwinded or not.
//! We ensure this by doing it in `Drop`.
//!
//!
//! ## Message sending
//!
//! We want to retain the new value and release the old, and importantly, in
//! that order (such that we don't dealloc the value if it didn't change).
//!
//! So something like this:
//! ```ignore
//! fn call_method(param: &mut Retained) {
//!     let old = *param;
//!
//!     msg_send![... param ...];
//!
//!     objc_retain(*param);
//!     objc_release(old);
//! }
//! ```
//!
//!
//! ## Defined methods
//!
//! With ARC, you can write a method like:
//!
//! ```objc
//! +(void) method: (id* out) {
//!     *out = [NSObject new];
//! }
//! ```
//!
//! And Clang will rewrite it to:
//!
//! ```objc
//! +(void) method: (id* out) {
//!     id __tmp = [NSObject new];
//!     *out = [__tmp autorelease];
//! }
//! ```
//!
//! This is unsound though if the code is inside an autorelease pool, see
//! <https://github.com/madsmtm/objc2/issues/283#issuecomment-5183833060>. So
//! what we do instead is slightly less efficient, we retain the object upon
//! entry, and autorelease it only at the end. This also allows us to keep the
//! signature as `&mut Retained`, instead of having to introduce a new
//! wrapper.
//!
//! So we want to end up with something like:
//!
//! ```objc
//! +(void) method: (id* out) {
//!     // Prelude
//!     [*out retain];
//!
//!     // User code
//!     id __tmp = [NSObject new];
//!     id __old = *out;
//!     *out = __tmp;
//!     [__tmp release];
//!
//!     // Postlude
//!     [*out autorelease];
//! }
//! ```
use core::hint::unreachable_unchecked;
use core::mem::ManuallyDrop;
use core::ptr::NonNull;

use super::ConvertArgument;
use crate::rc::Retained;
use crate::Message;

// Note the `'static` bound here - this may not be necessary, but I'm unsure
// of the exact requirements, so we better keep it for now.
impl<T: Message + 'static> ConvertArgument for &mut Retained<T> {
    // We use `Option<NonNull<*mut T>>` as the value instead of
    // `NonNull<NonNull<T>>`, since we want to do debug checking that the
    // value wasn't NULL or hasn't unexpectedly been overwritten to contain
    // NULL (both of these would be UB, but the user might reasonably have
    // made a mistake when writing their signature).
    type __Inner = Option<NonNull<*mut T>>;

    type __DropAfterMsgSend = RetainNewReleaseOldOnDrop<T, false>;

    type __DropBeforeReturn = AutoreleaseOnDrop<T>;

    #[inline]
    unsafe fn __into_argument(self) -> (Self::__Inner, Self::__DropAfterMsgSend) {
        let ptr: NonNull<Retained<T>> = NonNull::from(self);
        // `Retained` is `#[repr(transparent)]` over `NonNull`.
        let ptr: NonNull<NonNull<T>> = ptr.cast();
        // `NonNull<T>` has the same layout as `*mut T`.
        let ptr: NonNull<*mut T> = ptr.cast();

        // SAFETY: The value came from `&mut _`, and we only read a pointer.
        let old: *mut T = unsafe { *ptr.as_ptr() };

        (Some(ptr), RetainNewReleaseOldOnDrop { ptr, old })
    }

    #[inline]
    fn __from_defined_param(inner: Self::__Inner) -> (Self, Self::__DropBeforeReturn) {
        if let Some(inner) = inner {
            // SAFETY: We have exclusive access to the pointer
            // (it's `&mut Retained<T>`), so we can read it.
            let value = unsafe { *inner.as_ptr() };

            // Debug check the pointer.
            if cfg!(debug_assertions) && value.is_null() {
                panic!("defined class with `&mut Retained<_>` parameter, but was passed `&mut None`. You should handle this with `&mut Option<Retained<_>>` instead!");
            }

            // SAFETY: The value is valid, and now has +1 retain count.
            let value = unsafe { Retained::retain(value) };
            // Leak the value that we just retained. It will be released by
            // either `AutoreleaseOnDrop` or by the method if that writes a
            // new value to the pointer.
            let _ = ManuallyDrop::new(value);

            // `*mut T` has the same memory layout as `Option<Retained<T>>`,
            // and we checked above that the value is not nullable.
            let mut this: NonNull<Retained<T>> = inner.cast();

            // SAFETY: The signature declares that the pointer is a `&mut`
            // reference.
            let this: &mut Retained<T> = unsafe { this.as_mut() };

            (this, AutoreleaseOnDrop { ptr: inner })
        } else {
            if cfg!(debug_assertions) {
                panic!("defined class with `&mut Retained<_>` parameter, but was passed `None`. You should handle this with `Option<&mut Retained<_>>` instead!");
            } else {
                // SAFETY: The pointer is defined as non-null.
                // TODO: Should we make this always a runtime panic?
                unsafe { unreachable_unchecked() }
            }
        }
    }
}

impl<T: Message + 'static> ConvertArgument for &mut Option<Retained<T>> {
    type __Inner = Option<NonNull<*mut T>>;

    type __DropAfterMsgSend = RetainNewReleaseOldOnDrop<T, true>;

    type __DropBeforeReturn = AutoreleaseOnDrop<T>;

    #[inline]
    unsafe fn __into_argument(self) -> (Self::__Inner, Self::__DropAfterMsgSend) {
        let ptr: NonNull<Option<Retained<T>>> = NonNull::from(self);
        // `Option<Retained<T>>` has the same memory layout as `*mut T`.
        let ptr: NonNull<*mut T> = ptr.cast();
        // SAFETY: Same as for `&mut Retained`.
        let old: *mut T = unsafe { *ptr.as_ptr() };

        (Some(ptr), RetainNewReleaseOldOnDrop { ptr, old })
    }

    #[inline]
    fn __from_defined_param(inner: Self::__Inner) -> (Self, Self::__DropBeforeReturn) {
        if let Some(inner) = inner {
            // SAFETY: Same as for `&mut Retained`.
            let value = unsafe { *inner.as_ptr() };

            // We don't check null-ness of the value here, it can be NULL.

            // SAFETY: Same as for `&mut Retained`.
            let value = unsafe { Retained::retain(value) };
            let _ = ManuallyDrop::new(value);

            // `*mut T` has the same memory layout as `Option<Retained<T>>`.
            let mut this: NonNull<Option<Retained<T>>> = inner.cast();

            // SAFETY: Same as for `&mut Retained`.
            let this: &mut Option<Retained<T>> = unsafe { this.as_mut() };

            (this, AutoreleaseOnDrop { ptr: inner })
        } else {
            if cfg!(debug_assertions) {
                panic!("defined class with `&mut Option<Retained<_>>` parameter, but was passed `None`. You should handle this with `Option<&mut Option<Retained<_>>>` instead!");
            } else {
                // SAFETY: The pointer is defined as non-null.
                // TODO: Should we make this always a runtime panic?
                unsafe { unreachable_unchecked() }
            }
        }
    }
}

impl<T: Message + 'static> ConvertArgument for Option<&mut Retained<T>> {
    type __Inner = Option<NonNull<*mut T>>;

    // Use `Option` because we explicitly want to do the `if Some` checks
    // before the retain/release, since whether `None` or `Some` was passed is
    // often known at compile-time, and if `None` was provided, it would be
    // detrimental to have extra retain/release calls here.
    type __DropAfterMsgSend = Option<RetainNewReleaseOldOnDrop<T, false>>;

    // Use `Option` because we need to check whether the user provided a
    // pointer to write into or not.
    //
    // (And we need this check both for the initial retain, and for the
    // autorelease in `Drop`).
    type __DropBeforeReturn = Option<AutoreleaseOnDrop<T>>;

    #[inline]
    unsafe fn __into_argument(self) -> (Self::__Inner, Self::__DropAfterMsgSend) {
        if let Some(this) = self {
            // SAFETY: Upheld by caller.
            let (ptr, helper) = unsafe { this.__into_argument() };
            (ptr, Some(helper))
        } else {
            (None, None)
        }
    }

    #[inline]
    fn __from_defined_param(inner: Self::__Inner) -> (Self, Self::__DropBeforeReturn) {
        if let Some(inner) = inner {
            let (ptr, helper) = <&mut Retained<T>>::__from_defined_param(Some(inner));
            (Some(ptr), Some(helper))
        } else {
            (None, None)
        }
    }
}

impl<T: Message + 'static> ConvertArgument for Option<&mut Option<Retained<T>>> {
    type __Inner = Option<NonNull<*mut T>>;

    type __DropAfterMsgSend = Option<RetainNewReleaseOldOnDrop<T, true>>;

    type __DropBeforeReturn = Option<AutoreleaseOnDrop<T>>;

    #[inline]
    unsafe fn __into_argument(self) -> (Self::__Inner, Self::__DropAfterMsgSend) {
        if let Some(this) = self {
            // SAFETY: Upheld by caller.
            let (ptr, stored) = unsafe { this.__into_argument() };
            (ptr, Some(stored))
        } else {
            (None, None)
        }
    }

    #[inline]
    fn __from_defined_param(inner: Self::__Inner) -> (Self, Self::__DropBeforeReturn) {
        if let Some(inner) = inner {
            let (ptr, helper) = <&mut Option<Retained<T>>>::__from_defined_param(Some(inner));
            (Some(ptr), Some(helper))
        } else {
            (None, None)
        }
    }
}

/// On `Drop`, autorelease the value behind `ptr`.
#[derive(Debug)]
pub struct AutoreleaseOnDrop<T: Message> {
    /// A copy of the parameter so that we can autorelease it before returning
    /// from the method.
    ptr: NonNull<*mut T>,
}

impl<T: Message> Drop for AutoreleaseOnDrop<T> {
    #[inline]
    fn drop(&mut self) {
        // Read the (potentially newly written) value from the pointer.
        let ptr = unsafe { *self.ptr.as_ptr() };

        // Grab the +1 retain count.
        //
        // SAFETY: The pointer has +1 retain count from either
        // `__from_defined_param` above (if the value wasn't written), or from
        // overwriting the value with a value with +1 retain count.
        let obj = unsafe { Retained::from_raw(ptr) };

        // And autorelease it.
        //
        // If the caller wants to use the value, it is expected as part of the
        // ABI to retain the value before the next autorelease pool completes.
        let _ = Retained::autorelease_option(obj);
    }
}

/// On `Drop`, retain the new value in the `ptr` and release `old`.
#[derive(Debug)]
pub struct RetainNewReleaseOldOnDrop<T: Message, const INNER_NULLABLE: bool> {
    /// A copy of the argument, so that we can retain it after the message
    /// send.
    ///
    /// Ideally, we'd work with e.g. `&mut *mut T`, but we can't do that
    /// inside the generic context of `MessageArguments::__invoke`, while
    /// working within Rust's aliasing rules.
    ptr: NonNull<*mut T>,
    /// The old value, stored so that we can release if after the message
    /// send.
    old: *mut T,
}

impl<T: Message, const INNER_NULLABLE: bool> Drop for RetainNewReleaseOldOnDrop<T, INNER_NULLABLE> {
    #[inline]
    fn drop(&mut self) {
        // In terms of provenance, we roughly want to do the following:
        // ```ignore
        // fn call_method(value: &mut Retained<T>) {
        //     let old = value.clone();
        //     msg_send![... value ...];
        //     let _ = value.clone();
        //     drop(old);
        // }
        // ```
        //
        // Which is definitely valid under stacked borrows! See also this
        // playground link for testing something equivalent in Miri:
        // <https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=5ad8fcff1f870819081aa534ec754b86>

        // SAFETY: Caller ensures that the pointer is either left as-is, or is
        // safe to retain at this point.
        let new: Option<Retained<T>> = unsafe { Retained::retain(*self.ptr.as_ptr()) };
        // We ignore the result of `retain`, since it always returns the same
        // value as was given (and it would be unnecessary work to write that
        // value back into `ptr` again).
        let _new = ManuallyDrop::new(new);

        // SAFETY: The old pointer was valid when it was constructed.
        // (or could be NULL if INNER_NULLABLE).
        //
        // If the message send modified the argument, they would have left a
        // +1 retain count on the old pointer; so either we have +1 from that,
        // or the message send didn't modify the pointer and we instead have
        // +1 retain count from the `retain` above.
        if INNER_NULLABLE {
            // Note: We keep the `if old != nil { objc_release(old) }` check,
            // since we expect that the user would often do:
            //
            // ```ignore
            // let mut value = None
            // call_method(&mut value);
            // ```
            //
            // And in that case, we can elide the `objc_release` here!
            let _: Option<Retained<T>> = unsafe { Retained::from_raw(self.old) };
        } else {
            // Upheld by `INNER_NULLABLE`.
            //
            // TODO: Should we make this be always a runtime panic? It seems
            // likely that there's Objective-C code out there that would
            // declare the pointer as _Nonnull, but would write NULL on
            // allocation failure or similar.
            let old = unsafe { NonNull::new_unchecked(self.old) };
            let _: Retained<T> = unsafe { Retained::new_nonnull(old) };
        }

        // Check for NULL in new value _after_ we've released the old value,
        // so that we don't leak it in that case.
        if cfg!(debug_assertions) && !INNER_NULLABLE && _new.is_none() {
            panic!("found that NULL was written to `&mut Retained<_>`, which is UB! You should handle this with `&mut Option<Retained<_>>` instead");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    use super::*;
    use crate::rc::{autoreleasepool, Allocated, RcTestObject, ThreadTestData};
    use crate::runtime::NSObject;
    use crate::{define_class, extern_methods, msg_send, ClassType};

    #[test]
    fn test_bool_error() {
        let mut expected = ThreadTestData::current();

        fn bool_error(should_error: bool, error: Option<&mut Option<Retained<RcTestObject>>>) {
            let cls = RcTestObject::class();
            let did_succeed: bool =
                unsafe { msg_send![cls, boolAndShouldError: should_error, error: error] };
            assert_ne!(should_error, did_succeed);
        }

        bool_error(false, None);
        expected.assert_current();

        bool_error(true, None);
        expected.alloc += 1;
        expected.init += 1;
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();

        fn helper(
            expected: &mut ThreadTestData,
            should_error: bool,
            mut error: Option<Retained<RcTestObject>>,
        ) {
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
                expected.drop += 1;
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
        helper(&mut expected, false, Some(RcTestObject::new()));

        expected.alloc += 1;
        expected.init += 1;
        expected.retain += 1;
        expected.release += 1;
        expected.drop += 1;
        helper(&mut expected, true, Some(RcTestObject::new()));
    }

    #[test]
    #[cfg_attr(
        any(
            not(debug_assertions),
            all(not(target_pointer_width = "64"), feature = "catch-all")
        ),
        ignore = "invokes UB which is only caught with debug_assertions"
    )]
    #[should_panic = "found that NULL was written to `&mut Retained<_>`, which is UB! You should handle this with `&mut Option<Retained<_>>` instead"]
    fn test_debug_check_ub() {
        let cls = RcTestObject::class();
        let mut param: Retained<_> = RcTestObject::new();
        let _: () = unsafe { msg_send![cls, outParamNull: &mut param] };
    }

    // TODO: Fix this in release mode with Apple's runtime
    const AUTORELEASE_SKIPPED: bool = cfg!(feature = "gnustep-1-7");

    #[test]
    fn test_retained_interaction() {
        let mut expected = ThreadTestData::current();
        let cls = RcTestObject::class();

        let mut err: Retained<RcTestObject> = RcTestObject::new();
        expected.alloc += 1;
        expected.init += 1;
        expected.assert_current();

        autoreleasepool(|_| {
            let obj: Option<Retained<RcTestObject>> =
                unsafe { msg_send![cls, idAndShouldError: false, error: &mut err] };
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
                expected.drop += 1;
            }
            expected.assert_current();
        });
        if !AUTORELEASE_SKIPPED {
            expected.release += 1;
            expected.drop += 1;
        }
        expected.assert_current();

        drop(err);
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();
    }

    #[test]
    fn test_error_alloc() {
        let mut expected = ThreadTestData::current();

        // Succeeds
        let mut error: Option<Retained<RcTestObject>> = None;
        let res: Allocated<RcTestObject> = unsafe {
            msg_send![RcTestObject::class(), allocAndShouldError: false, error: &mut error]
        };
        expected.alloc += 1;
        expected.assert_current();
        assert!(!Allocated::as_ptr(&res).is_null());
        assert!(error.is_none());

        drop(res);
        expected.release += 1;
        // Drop flag ensures uninitialized do not drop
        // expected.drop += 1;
        expected.assert_current();

        // Errors
        let res: Retained<RcTestObject> = autoreleasepool(|_pool| {
            let mut error = None;
            let res: Allocated<RcTestObject> = unsafe {
                msg_send![RcTestObject::class(), allocAndShouldError: true, error: &mut error]
            };
            expected.alloc += 1;
            expected.init += 1;
            expected.autorelease += 1;
            expected.retain += 1;
            expected.assert_current();
            assert!(Allocated::as_ptr(&res).is_null());
            error.unwrap()
        });
        expected.release += 1;
        expected.assert_current();

        drop(res);
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();
    }

    fn will_panic(param: Option<&mut Option<Retained<RcTestObject>>>, panic_after: bool) {
        unsafe { msg_send![RcTestObject::class(), willPanicWith: param, panicsAfter: panic_after] }
    }

    #[test]
    #[cfg_attr(
        feature = "catch-all",
        ignore = "panics intentionally, which catch-all interferes with"
    )]
    fn basic_method_panics() {
        let expected = ThreadTestData::current();

        let res = catch_unwind(|| {
            will_panic(None, false);
        });
        assert!(res.is_err());
        expected.assert_current();

        let res = catch_unwind(|| {
            will_panic(None, true);
        });
        assert!(res.is_err());
        expected.assert_current();
    }

    #[test]
    #[cfg_attr(
        any(feature = "catch-all", panic = "abort"),
        ignore = "panics intentionally"
    )]
    fn method_panics() {
        let cases = [
            (false, None),
            (true, None),
            // Pre-existing parameter passed in.
            (false, Some(RcTestObject::new())),
            (true, Some(RcTestObject::new())),
        ];

        let mut expected = ThreadTestData::current();

        for (panic_after, mut param) in cases {
            let initially_set = param.is_some();

            autoreleasepool(|_| {
                let unwindsafe = AssertUnwindSafe(&mut param);
                let res = catch_unwind(|| {
                    let param = unwindsafe;
                    will_panic(Some(param.0), panic_after);
                });
                assert!(res.is_err());

                if panic_after {
                    expected.alloc += 1;
                    expected.init += 1;
                    expected.autorelease += 1;
                }
                if panic_after || initially_set {
                    expected.retain += 1;
                }
                if initially_set {
                    expected.release += 1;
                    if panic_after {
                        expected.drop += 1;
                    }
                }
                expected.assert_current();
            });

            if panic_after {
                expected.release += 1;
            }
            expected.assert_current();

            drop(param);
            if panic_after || initially_set {
                expected.release += 1;
                expected.drop += 1;
            }
            expected.assert_current();
        }
    }

    #[test]
    fn define_out_param() {
        define_class!(
            #[unsafe(super(NSObject))]
            struct OutParam;

            /// This doc comment is here to make rustfmt work.
            impl OutParam {
                #[unsafe(method(nonNullNonNull:value:))]
                fn _nonnull_nonnull(param: &mut Retained<NSObject>, value: Option<&NSObject>) {
                    *param = value.expect("null obj").retain();
                }

                #[unsafe(method(nonNullNullable:value:))]
                fn _nonnull_nullable(
                    param: &mut Option<Retained<NSObject>>,
                    value: Option<&NSObject>,
                ) {
                    *param = value.map(|value| value.retain());
                }

                #[unsafe(method(nullableNonNull:value:))]
                fn _nullable_nonnull(
                    param: Option<&mut Retained<NSObject>>,
                    value: Option<&NSObject>,
                ) {
                    if let Some(param) = param {
                        *param = value.expect("null obj").retain();
                    }
                }

                #[unsafe(method(nullableNullable:value:))]
                fn _nullable_nullable(
                    param: Option<&mut Option<Retained<NSObject>>>,
                    value: Option<&NSObject>,
                ) {
                    if let Some(param) = param {
                        *param = value.map(|value| value.retain());
                    }
                }
            }
        );

        impl OutParam {
            // Deliberately incorrectly map these methods, to allow testing how it
            // behaves when given invalid input.
            extern_methods!(
                #[unsafe(method(nonNullNonNull:value:))]
                fn nonnull_nonnull(
                    param: Option<&mut Option<Retained<NSObject>>>,
                    value: Option<&NSObject>,
                );

                #[unsafe(method(nonNullNullable:value:))]
                fn nonnull_nullable(
                    param: Option<&mut Option<Retained<NSObject>>>,
                    value: Option<&NSObject>,
                );

                #[unsafe(method(nullableNonNull:value:))]
                fn nullable_nonnull(
                    param: Option<&mut Option<Retained<NSObject>>>,
                    value: Option<&NSObject>,
                );

                #[unsafe(method(nullableNullable:value:))]
                fn nullable_nullable(
                    param: Option<&mut Option<Retained<NSObject>>>,
                    value: Option<&NSObject>,
                );
            );
        }

        let obj = RcTestObject::new();
        let mut expected = ThreadTestData::current();

        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        enum ParamKind {
            None,
            SomeNone,
            SomeSome,
        }

        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        enum PanicKind {
            None,
            Method,
            DefineClass,
            Ub,
        }

        let fns: [(fn(param: Option<&mut _>, obj: _), _, _, _); _] = [
            // &mut Retained<NSObject>
            (
                OutParam::nonnull_nonnull,
                None,
                ParamKind::SomeSome,
                PanicKind::Method,
            ),
            (
                OutParam::nonnull_nonnull,
                Some(&obj),
                ParamKind::SomeSome,
                PanicKind::None,
            ),
            (
                OutParam::nonnull_nonnull,
                None,
                ParamKind::SomeNone,
                if cfg!(debug_assertions) {
                    PanicKind::DefineClass
                } else {
                    PanicKind::Method
                },
            ),
            (
                OutParam::nonnull_nonnull,
                Some(&obj),
                ParamKind::SomeNone,
                if cfg!(debug_assertions) {
                    PanicKind::DefineClass
                } else {
                    PanicKind::Ub
                },
            ),
            (
                OutParam::nonnull_nonnull,
                None,
                ParamKind::None,
                if cfg!(debug_assertions) {
                    PanicKind::DefineClass
                } else {
                    PanicKind::Ub
                },
            ),
            (
                OutParam::nonnull_nonnull,
                Some(&obj),
                ParamKind::None,
                if cfg!(debug_assertions) {
                    PanicKind::DefineClass
                } else {
                    PanicKind::Ub
                },
            ),
            // &mut Option<Retained<NSObject>>
            (
                OutParam::nonnull_nullable,
                None,
                ParamKind::SomeSome,
                PanicKind::None,
            ),
            (
                OutParam::nonnull_nullable,
                Some(&obj),
                ParamKind::SomeSome,
                PanicKind::None,
            ),
            (
                OutParam::nonnull_nullable,
                None,
                ParamKind::SomeNone,
                PanicKind::None,
            ),
            (
                OutParam::nonnull_nullable,
                Some(&obj),
                ParamKind::SomeNone,
                PanicKind::None,
            ),
            (
                OutParam::nonnull_nullable,
                None,
                ParamKind::None,
                if cfg!(debug_assertions) {
                    PanicKind::DefineClass
                } else {
                    PanicKind::Ub
                },
            ),
            (
                OutParam::nonnull_nullable,
                Some(&obj),
                ParamKind::None,
                if cfg!(debug_assertions) {
                    PanicKind::DefineClass
                } else {
                    PanicKind::Ub
                },
            ),
            // Option<&mut Retained<NSObject>>
            (
                OutParam::nullable_nonnull,
                None,
                ParamKind::SomeSome,
                PanicKind::Method,
            ),
            (
                OutParam::nullable_nonnull,
                Some(&obj),
                ParamKind::SomeSome,
                PanicKind::None,
            ),
            (
                OutParam::nullable_nonnull,
                None,
                ParamKind::SomeNone,
                if cfg!(debug_assertions) {
                    PanicKind::DefineClass
                } else {
                    PanicKind::Method
                },
            ),
            (
                OutParam::nullable_nonnull,
                Some(&obj),
                ParamKind::SomeNone,
                if cfg!(debug_assertions) {
                    PanicKind::DefineClass
                } else {
                    PanicKind::Ub
                },
            ),
            (
                OutParam::nullable_nonnull,
                None,
                ParamKind::None,
                PanicKind::None,
            ),
            (
                OutParam::nullable_nonnull,
                Some(&obj),
                ParamKind::None,
                PanicKind::None,
            ),
            // Option<&mut Option<Retained<NSObject>>>
            (
                OutParam::nullable_nullable,
                None,
                ParamKind::SomeSome,
                PanicKind::None,
            ),
            (
                OutParam::nullable_nullable,
                Some(&obj),
                ParamKind::SomeSome,
                PanicKind::None,
            ),
            (
                OutParam::nullable_nullable,
                None,
                ParamKind::SomeNone,
                PanicKind::None,
            ),
            (
                OutParam::nullable_nullable,
                Some(&obj),
                ParamKind::SomeNone,
                PanicKind::None,
            ),
            (
                OutParam::nullable_nullable,
                None,
                ParamKind::None,
                PanicKind::None,
            ),
            (
                OutParam::nullable_nullable,
                Some(&obj),
                ParamKind::None,
                PanicKind::None,
            ),
        ];

        for (f, value, param_kind, panic_kind) in fns {
            std::dbg!((f, value, param_kind, panic_kind));

            // Skip UB tests.
            if panic_kind == PanicKind::Ub {
                continue;
            }

            let has_param = param_kind != ParamKind::None;
            let has_param_value = param_kind == ParamKind::SomeSome;
            let define_class_panic = panic_kind == PanicKind::DefineClass;
            let method_panic = panic_kind == PanicKind::Method;
            let panics = matches!(panic_kind, PanicKind::DefineClass | PanicKind::Method);

            let value_ends_as_some = has_param
                && (value.is_some() || (method_panic && has_param_value))
                && !define_class_panic;

            let mut param = has_param_value.then(|| {
                expected.retain += 1;
                Retained::into_super(obj.clone())
            });
            expected.assert_current();

            autoreleasepool(|_| {
                let res = catch_unwind(AssertUnwindSafe(|| {
                    f(has_param.then_some(&mut param), value.map(|x| x.as_ref()))
                }));
                assert_eq!(res.is_err(), panic_kind != PanicKind::None);

                // `define_class!` retains `param`.
                if has_param_value && !define_class_panic {
                    expected.retain += 1;
                }

                // Method retains `value`.
                if has_param && value.is_some() && !panics {
                    expected.retain += 1;
                }

                // Method writes `value` to `param`, which `Drop`s `param`.
                if has_param_value && !panics {
                    expected.release += 1;
                }

                // `define_class!` autoreleases (new) `param`.
                if value_ends_as_some {
                    expected.autorelease += 1;
                }

                // `extern_methods!` retains (new) `param`.
                if value_ends_as_some {
                    expected.retain += 1;
                }

                // `extern_methods!` releases old `param`.
                if has_param_value {
                    expected.release += 1;
                }

                expected.assert_current();
            });
            // From autorelease.
            if value_ends_as_some {
                expected.release += 1;
            }
            expected.assert_current();

            assert_eq!(value_ends_as_some, param.is_some());
            // If anything is still in `param`, we'll release it.
            if let Some(param) = param {
                drop(param);
                expected.release += 1;
            }
            expected.assert_current();
        }

        drop(obj);
        expected.release += 1;
        expected.drop += 1;
        expected.assert_current();
    }
}
