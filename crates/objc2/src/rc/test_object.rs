use core::cell::RefCell;
use core::mem::ManuallyDrop;
use core::ptr;

use super::{Id, Owned};
use crate::foundation::{NSObject, NSZone};
use crate::{declare_class, msg_send, msg_send_id, ClassType};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ThreadTestData {
    pub(crate) alloc: usize,
    pub(crate) dealloc: usize,
    pub(crate) init: usize,
    pub(crate) retain: usize,
    pub(crate) copy: usize,
    pub(crate) mutable_copy: usize,
    pub(crate) release: usize,
    pub(crate) autorelease: usize,
    pub(crate) try_retain: usize,
    pub(crate) try_retain_fail: usize,
}

impl ThreadTestData {
    /// Get the amount of method calls performed on the current thread.
    pub(crate) fn current() -> ThreadTestData {
        TEST_DATA.with(|data| data.borrow().clone())
    }

    #[track_caller]
    pub(crate) fn assert_current(&self) {
        let current = Self::current();
        let mut expected = self.clone();
        if cfg!(feature = "gnustep-1-7") {
            // GNUStep doesn't have `tryRetain`, it uses `retain` directly
            let retain_diff = expected.try_retain - current.try_retain;
            expected.retain += retain_diff;
            expected.try_retain -= retain_diff;

            // GNUStep doesn't call `autorelease` if it's overridden
            expected.autorelease = 0;
        }
        assert_eq!(current, expected);
    }
}

std::thread_local! {
    pub(crate) static TEST_DATA: RefCell<ThreadTestData> = RefCell::new(Default::default());
}

declare_class!(
    /// A helper object that counts how many times various reference-counting
    /// primitives are called.
    #[derive(Debug, PartialEq)]
    pub(crate) struct RcTestObject {}

    unsafe impl ClassType for RcTestObject {
        type Super = NSObject;
    }

    unsafe impl RcTestObject {
        #[method(newReturningNull)]
        fn new_returning_null() -> *mut Self {
            ptr::null_mut()
        }

        #[method(newMethodOnInstance)]
        fn new_method_on_instance(&self) -> *mut Self {
            Id::consume_as_ptr(ManuallyDrop::new(Self::new()))
        }

        #[method(newMethodOnInstanceNull)]
        fn new_method_on_instance_null(&self) -> *mut Self {
            ptr::null_mut()
        }

        #[method(alloc)]
        fn alloc_() -> *mut Self {
            TEST_DATA.with(|data| data.borrow_mut().alloc += 1);
            let superclass = NSObject::class().metaclass();
            let zone: *const NSZone = ptr::null();
            unsafe { msg_send![super(Self::class(), superclass), allocWithZone: zone] }
        }

        #[method(allocWithZone:)]
        fn alloc_with_zone(zone: *const NSZone) -> *mut Self {
            TEST_DATA.with(|data| data.borrow_mut().alloc += 1);
            let superclass = NSObject::class().metaclass();
            unsafe { msg_send![super(Self::class(), superclass), allocWithZone: zone] }
        }

        #[method(allocReturningNull)]
        fn alloc_returning_null() -> *mut Self {
            ptr::null_mut()
        }

        #[method(init)]
        fn init(this: &mut Self) -> *mut Self {
            TEST_DATA.with(|data| data.borrow_mut().init += 1);
            unsafe { msg_send![super(this), init] }
        }

        #[method(initReturningNull)]
        fn init_returning_null(&mut self) -> *mut Self {
            let _: () = unsafe { msg_send![self, release] };
            ptr::null_mut()
        }

        #[method(retain)]
        fn retain(&self) -> *mut Self {
            TEST_DATA.with(|data| data.borrow_mut().retain += 1);
            unsafe { msg_send![super(self), retain] }
        }

        #[method(release)]
        fn release(&self) {
            TEST_DATA.with(|data| data.borrow_mut().release += 1);
            unsafe { msg_send![super(self), release] }
        }

        #[method(autorelease)]
        fn autorelease(&self) -> *mut Self {
            TEST_DATA.with(|data| data.borrow_mut().autorelease += 1);
            unsafe { msg_send![super(self), autorelease] }
        }

        #[method(dealloc)]
        unsafe fn dealloc(&mut self) {
            TEST_DATA.with(|data| data.borrow_mut().dealloc += 1);
            unsafe { msg_send![super(self), dealloc] }
        }

        #[method(_tryRetain)]
        unsafe fn try_retain(&self) -> bool {
            TEST_DATA.with(|data| data.borrow_mut().try_retain += 1);
            let res: bool = unsafe { msg_send![super(self), _tryRetain] };
            if !res {
                TEST_DATA.with(|data| data.borrow_mut().try_retain -= 1);
                TEST_DATA.with(|data| data.borrow_mut().try_retain_fail += 1);
            }
            res
        }

        #[method(copyWithZone:)]
        fn copy_with_zone(&self, _zone: *const NSZone) -> *const Self {
            TEST_DATA.with(|data| data.borrow_mut().copy += 1);
            Id::consume_as_ptr(ManuallyDrop::new(Self::new()))
        }

        #[method(mutableCopyWithZone:)]
        fn mutable_copy_with_zone(&self, _zone: *const NSZone) -> *const Self {
            TEST_DATA.with(|data| data.borrow_mut().mutable_copy += 1);
            Id::consume_as_ptr(ManuallyDrop::new(Self::new()))
        }

        #[method(copyReturningNull)]
        fn copy_returning_null(_this: &Self) -> *const Self {
            ptr::null()
        }

        #[method(methodReturningNull)]
        fn method_returning_null(self: &Self) -> *const Self {
            ptr::null()
        }

        #[method(boolAndShouldError:error:)]
        fn class_error_bool(should_error: bool, err: Option<&mut *mut RcTestObject>) -> bool {
            if should_error {
                if let Some(err) = err {
                    *err = RcTestObject::new().autorelease_inner();
                }
                false
            } else {
                true
            }
        }

        #[method(boolAndShouldError:error:)]
        fn instance_error_bool(
            &self,
            should_error: bool,
            err: Option<&mut *mut RcTestObject>,
        ) -> bool {
            if should_error {
                if let Some(err) = err {
                    *err = RcTestObject::new().autorelease_inner();
                }
                false
            } else {
                true
            }
        }

        #[method(idAndShouldError:error:)]
        fn class_error_id(
            should_error: bool,
            err: Option<&mut *mut RcTestObject>,
        ) -> *mut RcTestObject {
            if should_error {
                if let Some(err) = err {
                    *err = RcTestObject::new().autorelease_inner();
                }
                ptr::null_mut()
            } else {
                RcTestObject::new().autorelease_return()
            }
        }

        #[method(idAndShouldError:error:)]
        fn instance_error_id(
            self: &Self,
            should_error: bool,
            err: Option<&mut *mut RcTestObject>,
        ) -> *mut RcTestObject {
            if should_error {
                if let Some(err) = err {
                    *err = RcTestObject::new().autorelease_inner();
                }
                ptr::null_mut()
            } else {
                RcTestObject::new().autorelease_return()
            }
        }

        #[method(newAndShouldError:error:)]
        fn new_error(should_error: bool, err: Option<&mut *mut RcTestObject>) -> *mut Self {
            if should_error {
                if let Some(err) = err {
                    *err = RcTestObject::new().autorelease_inner();
                }
                ptr::null_mut()
            } else {
                unsafe { msg_send![Self::class(), new] }
            }
        }

        #[method(allocAndShouldError:error:)]
        fn alloc_error(should_error: bool, err: Option<&mut *mut RcTestObject>) -> *mut Self {
            if should_error {
                if let Some(err) = err {
                    *err = RcTestObject::new().autorelease_inner();
                }
                ptr::null_mut()
            } else {
                unsafe { msg_send![Self::class(), alloc] }
            }
        }

        #[method(initAndShouldError:error:)]
        fn init_error(
            this: &mut Self,
            should_error: bool,
            err: Option<&mut *mut RcTestObject>,
        ) -> *mut Self {
            if should_error {
                if let Some(err) = err {
                    *err = RcTestObject::new().autorelease_inner();
                }
                let _: () = unsafe { msg_send![this, release] };
                ptr::null_mut()
            } else {
                unsafe { msg_send![this, init] }
            }
        }
    }
);

unsafe impl Send for RcTestObject {}
unsafe impl Sync for RcTestObject {}

impl RcTestObject {
    pub(crate) fn new() -> Id<Self, Owned> {
        // Use msg_send! - msg_send_id! is tested elsewhere!
        unsafe { Id::new(msg_send![Self::class(), new]) }.unwrap()
    }
}

declare_class!(
    #[derive(Debug, PartialEq)]
    pub(crate) struct RcTestObjectSubclass {}

    unsafe impl ClassType for RcTestObjectSubclass {
        #[inherits(NSObject)]
        type Super = RcTestObject;
    }
);

impl RcTestObjectSubclass {
    pub(crate) fn new() -> Id<Self, Owned> {
        unsafe { msg_send_id![Self::class(), new] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rc::{autoreleasepool, Allocated, Shared};

    #[test]
    fn ensure_declared_name() {
        assert_eq!(RcTestObject::class().name(), RcTestObject::NAME);
    }

    macro_rules! test_error_bool {
        ($expected:expr, $($obj:tt)*) => {
            // Succeeds
            let res: Result<(), Id<RcTestObject, Shared>> = unsafe {
                msg_send![$($obj)*, boolAndShouldError: false, error: _]
            };
            assert_eq!(res, Ok(()));
            $expected.assert_current();

            // Errors
            let res = autoreleasepool(|_pool| {
                // `Ok` type is inferred to be `()`
                let res: Id<RcTestObject, Shared> = unsafe {
                    msg_send![$($obj)*, boolAndShouldError: true, error: _]
                }.expect_err("not err");
                $expected.alloc += 1;
                $expected.init += 1;
                $expected.autorelease += 1;
                $expected.retain += 1;
                $expected.assert_current();
                res
            });
            $expected.release += 1;
            $expected.assert_current();

            drop(res);
            $expected.release += 1;
            $expected.dealloc += 1;
            $expected.assert_current();
        }
    }

    #[test]
    fn test_error_bool() {
        let mut expected = ThreadTestData::current();

        let cls = RcTestObject::class();
        test_error_bool!(expected, cls);

        let obj = RcTestObject::new();
        expected.alloc += 1;
        expected.init += 1;
        test_error_bool!(expected, &obj);

        let obj = RcTestObjectSubclass::new();
        expected.alloc += 1;
        expected.init += 1;
        test_error_bool!(expected, &obj);
        test_error_bool!(expected, super(&obj));
        test_error_bool!(expected, super(&obj, RcTestObjectSubclass::class()));
        test_error_bool!(expected, super(&obj, RcTestObject::class()));
    }

    // This is imperfect, but will do for now.
    // See also `tests/id_retain_autoreleased.rs`.
    //
    // Work around https://github.com/rust-lang/rust-clippy/issues/9737:
    const IF_AUTORELEASE_NOT_SKIPPED: usize = if cfg!(feature = "gnustep-1-7") {
        1
    } else if cfg!(any(
        debug_assertions,
        feature = "exception",
        feature = "verify_message"
    )) {
        2
    } else {
        1
    } - 1;

    macro_rules! test_error_id {
        ($expected:expr, $if_autorelease_not_skipped:expr, $sel:ident, $($obj:tt)*) => {
            // Succeeds
            let res = autoreleasepool(|_pool| {
                let res: Result<Id<RcTestObject, Owned>, Id<RcTestObject, Shared>> = unsafe {
                    msg_send_id![$($obj)*, $sel: false, error: _]
                };
                let res = res.expect("not ok");
                $expected.alloc += 1;
                $expected.init += 1;
                $expected.autorelease += $if_autorelease_not_skipped;
                $expected.retain += $if_autorelease_not_skipped;
                $expected.assert_current();
                res
            });
            $expected.release += $if_autorelease_not_skipped;
            $expected.assert_current();

            drop(res);
            $expected.release += 1;
            $expected.dealloc += 1;
            $expected.assert_current();

            // Errors
            let res = autoreleasepool(|_pool| {
                let res: Result<Id<RcTestObject, Owned>, Id<RcTestObject, Shared>> = unsafe {
                    msg_send_id![$($obj)*, $sel: true, error: _]
                };
                $expected.alloc += 1;
                $expected.init += 1;
                $expected.autorelease += 1;
                $expected.retain += 1;
                $expected.assert_current();
                res.expect_err("not err")
            });
            $expected.release += 1;
            $expected.assert_current();

            drop(res);
            $expected.release += 1;
            $expected.dealloc += 1;
            $expected.assert_current();
        }
    }

    #[test]
    fn test_error_id() {
        let mut expected = ThreadTestData::current();

        let cls = RcTestObject::class();
        test_error_id!(expected, IF_AUTORELEASE_NOT_SKIPPED, idAndShouldError, cls);
        test_error_id!(expected, 0, newAndShouldError, cls);

        let obj = RcTestObject::new();
        expected.alloc += 1;
        expected.init += 1;
        test_error_id!(expected, IF_AUTORELEASE_NOT_SKIPPED, idAndShouldError, &obj);

        expected.alloc -= 1;
        expected.release -= 1;
        expected.dealloc -= 1;
        test_error_id!(expected, 0, initAndShouldError, {
            expected.alloc += 1;
            expected.release += 1;
            expected.dealloc += 1;
            RcTestObject::alloc()
        });
    }

    #[test]
    fn test_error_alloc() {
        let mut expected = ThreadTestData::current();

        // Succeeds
        let res: Result<Allocated<RcTestObject>, Id<RcTestObject, Shared>> =
            unsafe { msg_send_id![RcTestObject::class(), allocAndShouldError: false, error: _] };
        let res = res.expect("not ok");
        expected.alloc += 1;
        expected.assert_current();

        drop(res);
        expected.release += 1;
        expected.dealloc += 1;
        expected.assert_current();

        // Errors
        let res = autoreleasepool(|_pool| {
            let res: Result<Allocated<RcTestObject>, Id<RcTestObject, Shared>> =
                unsafe { msg_send_id![RcTestObject::class(), allocAndShouldError: true, error: _] };
            expected.alloc += 1;
            expected.init += 1;
            expected.autorelease += 1;
            expected.retain += 1;
            expected.assert_current();
            res.expect_err("not err")
        });
        expected.release += 1;
        expected.assert_current();

        drop(res);
        expected.release += 1;
        expected.dealloc += 1;
        expected.assert_current();
    }
}
