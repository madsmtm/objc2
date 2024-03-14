use core::cell::RefCell;
use core::ptr;

use super::{Allocated, DefaultId, Id};
use crate::mutability::Immutable;
use crate::runtime::{NSObject, NSZone};
use crate::{declare_class, msg_send, msg_send_id, ClassType, DeclaredClass};

// TODO: Put tests that use this in another crate
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[allow(missing_copy_implementations)]
#[doc(hidden)]
pub struct __ThreadTestData {
    pub alloc: usize,
    pub drop: usize,
    pub init: usize,
    pub retain: usize,
    pub copy: usize,
    pub mutable_copy: usize,
    pub release: usize,
    pub autorelease: usize,
    pub try_retain: usize,
    pub try_retain_fail: usize,
    // TODO: Is there some way we can test weak pointers? Or is that implemented entirely in Foundation?
    // Maybe `_setWeaklyReferenced` can be useful?
}

impl __ThreadTestData {
    /// Get the amount of method calls performed on the current thread.
    pub fn current() -> __ThreadTestData {
        TEST_DATA.with(|data| data.borrow().clone())
    }

    #[track_caller]
    #[allow(clippy::missing_panics_doc)]
    pub fn assert_current(&self) {
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
        if current != expected {
            panic!(
                "got differing amounts of calls:
   current: `{current:?}`,
  expected: `{expected:?}`"
            )
        }
    }
}

std::thread_local! {
    static TEST_DATA: RefCell<__ThreadTestData> = RefCell::default();
}

declare_class!(
    /// A helper object that counts how many times various reference-counting
    /// primitives are called.
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[doc(hidden)]
    pub struct __RcTestObject;

    unsafe impl ClassType for __RcTestObject {
        type Super = NSObject;
        type Mutability = Immutable;
        const NAME: &'static str = "__RcTestObject";
    }

    impl DeclaredClass for __RcTestObject {}

    unsafe impl __RcTestObject {
        #[method_id(newReturningNull)]
        fn new_returning_null() -> Option<Id<Self>> {
            None
        }

        #[method_id(newMethodOnInstance)]
        fn new_method_on_instance(&self) -> Id<Self> {
            Self::new()
        }

        #[method_id(newMethodOnInstanceNull)]
        fn new_method_on_instance_null(&self) -> Option<Id<Self>> {
            None
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

        #[method_id(init)]
        unsafe fn init(this: Allocated<Self>) -> Id<Self> {
            TEST_DATA.with(|data| data.borrow_mut().init += 1);
            let this = this.set_ivars(());
            unsafe { msg_send_id![super(this), init] }
        }

        #[method_id(initReturningNull)]
        fn init_returning_null(_this: Allocated<Self>) -> Option<Id<Self>> {
            None
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

        #[method_id(copyWithZone:)]
        fn copy_with_zone(&self, _zone: *const NSZone) -> Id<Self> {
            TEST_DATA.with(|data| data.borrow_mut().copy += 1);
            Self::new()
        }

        #[method_id(mutableCopyWithZone:)]
        fn mutable_copy_with_zone(&self, _zone: *const NSZone) -> Id<Self> {
            TEST_DATA.with(|data| data.borrow_mut().mutable_copy += 1);
            Self::new()
        }

        #[method_id(copyReturningNull)]
        fn copy_returning_null(_this: &Self) -> Option<Id<Self>> {
            None
        }

        #[method_id(methodReturningNull)]
        fn method_returning_null(self: &Self) -> Option<Id<Self>> {
            None
        }

        #[method_id(aMethod:)]
        fn a_method(&self, param: bool) -> Option<Id<Self>> {
            param.then(Self::new)
        }

        #[method(boolAndShouldError:error:)]
        fn class_error_bool(should_error: bool, err: Option<&mut *mut __RcTestObject>) -> bool {
            if should_error {
                if let Some(err) = err {
                    *err = Id::autorelease_inner(__RcTestObject::new());
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
            err: Option<&mut *mut __RcTestObject>,
        ) -> bool {
            if should_error {
                if let Some(err) = err {
                    *err = Id::autorelease_inner(__RcTestObject::new());
                }
                false
            } else {
                true
            }
        }

        #[method_id(idAndShouldError:error:)]
        fn class_error_id(
            should_error: bool,
            err: Option<&mut *mut __RcTestObject>,
        ) -> Option<Id<Self>> {
            if should_error {
                if let Some(err) = err {
                    *err = Id::autorelease_inner(__RcTestObject::new());
                }
                None
            } else {
                Some(Self::new())
            }
        }

        #[method_id(idAndShouldError:error:)]
        fn instance_error_id(
            self: &Self,
            should_error: bool,
            err: Option<&mut *mut __RcTestObject>,
        ) -> Option<Id<Self>> {
            if should_error {
                if let Some(err) = err {
                    *err = Id::autorelease_inner(__RcTestObject::new());
                }
                None
            } else {
                Some(Self::new())
            }
        }

        #[method_id(newAndShouldError:error:)]
        fn new_error(
            should_error: bool,
            err: Option<&mut *mut __RcTestObject>,
        ) -> Option<Id<Self>> {
            if should_error {
                if let Some(err) = err {
                    *err = Id::autorelease_inner(__RcTestObject::new());
                }
                None
            } else {
                unsafe { msg_send_id![Self::class(), new] }
            }
        }

        #[method(allocAndShouldError:error:)]
        fn alloc_error(should_error: bool, err: Option<&mut *mut __RcTestObject>) -> *mut Self {
            if should_error {
                if let Some(err) = err {
                    *err = Id::autorelease_inner(__RcTestObject::new());
                }
                ptr::null_mut()
            } else {
                unsafe { msg_send![Self::class(), alloc] }
            }
        }

        #[method_id(initAndShouldError:error:)]
        fn init_error(
            this: Allocated<Self>,
            should_error: bool,
            err: Option<&mut *mut __RcTestObject>,
        ) -> Option<Id<Self>> {
            if should_error {
                if let Some(err) = err {
                    *err = Id::autorelease_inner(__RcTestObject::new());
                }
                None
            } else {
                unsafe { msg_send_id![this, init] }
            }
        }

        #[method(outParamNull:)]
        fn out_param_null(param: Option<&mut *mut __RcTestObject>) {
            if let Some(param) = param {
                *param = ptr::null_mut();
            }
        }
    }
);

impl Drop for __RcTestObject {
    fn drop(&mut self) {
        TEST_DATA.with(|data| data.borrow_mut().drop += 1);
    }
}

impl DefaultId for __RcTestObject {
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

unsafe impl Send for __RcTestObject {}
unsafe impl Sync for __RcTestObject {}

impl __RcTestObject {
    #[doc(hidden)]
    pub fn new() -> Id<Self> {
        // Use msg_send! - msg_send_id! is tested elsewhere!
        unsafe { Id::from_raw(msg_send![Self::class(), new]) }.unwrap()
    }
}

declare_class!(
    #[derive(Debug, PartialEq, Eq)]
    struct RcTestObjectSubclass;

    unsafe impl ClassType for RcTestObjectSubclass {
        #[inherits(NSObject)]
        type Super = __RcTestObject;
        type Mutability = Immutable;
        const NAME: &'static str = "RcTestObjectSubclass";
    }

    impl DeclaredClass for RcTestObjectSubclass {}
);

#[cfg_attr(not(test), allow(unused))]
impl RcTestObjectSubclass {
    fn new() -> Id<Self> {
        unsafe { msg_send_id![Self::class(), new] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rc::autoreleasepool;

    #[test]
    fn ensure_declared_name() {
        assert_eq!(__RcTestObject::class().name(), __RcTestObject::NAME);
    }

    macro_rules! test_error_bool {
        ($expected:expr, $($obj:tt)*) => {
            // Succeeds
            let res: Result<(), Id<__RcTestObject>> = unsafe {
                msg_send![$($obj)*, boolAndShouldError: false, error: _]
            };
            assert_eq!(res, Ok(()));
            $expected.assert_current();

            // Errors
            let res = autoreleasepool(|_pool| {
                // `Ok` type is inferred to be `()`
                let res: Id<__RcTestObject> = unsafe {
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
            $expected.drop += 1;
            $expected.assert_current();
        }
    }

    #[test]
    fn test_error_bool() {
        let mut expected = __ThreadTestData::current();

        let cls = __RcTestObject::class();
        test_error_bool!(expected, cls);

        let obj = __RcTestObject::new();
        expected.alloc += 1;
        expected.init += 1;
        test_error_bool!(expected, &obj);

        let obj = RcTestObjectSubclass::new();
        expected.alloc += 1;
        expected.init += 1;
        test_error_bool!(expected, &obj);
        test_error_bool!(expected, super(&obj));
        test_error_bool!(expected, super(&obj, RcTestObjectSubclass::class()));
        test_error_bool!(expected, super(&obj, __RcTestObject::class()));
    }

    // This is imperfect, but will do for now.
    // See also `tests/id_retain_autoreleased.rs`.
    //
    // Work around https://github.com/rust-lang/rust-clippy/issues/9737:
    #[allow(clippy::if_same_then_else)]
    const IF_AUTORELEASE_NOT_SKIPPED: usize = if cfg!(feature = "gnustep-1-7") {
        1
    } else if cfg!(target_arch = "x86") {
        // x86 autorelease_return is not currently tail-called, so the
        // optimization doesn't work on declare_class! functions.
        2
    } else if cfg!(target_arch = "aarch64") {
        // Currently doesn't work
        2
    } else if cfg!(any(debug_assertions, feature = "catch-all")) {
        2
    } else {
        1
    } - 1;

    // 32-bit ARM unwinding sometimes interferes with the optimization
    const IF_AUTORELEASE_NOT_SKIPPED_ARM_HACK: usize = {
        if cfg!(all(target_arch = "arm", panic = "unwind")) {
            1
        } else {
            IF_AUTORELEASE_NOT_SKIPPED
        }
    };

    macro_rules! test_error_id {
        ($expected:expr, $if_autorelease_not_skipped:expr, $sel:ident, $($obj:tt)*) => {
            // Succeeds
            let res = autoreleasepool(|_pool| {
                let res: Result<Id<__RcTestObject>, Id<__RcTestObject>> = unsafe {
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
            $expected.drop += 1;
            $expected.assert_current();

            // Errors
            let res = autoreleasepool(|_pool| {
                let res: Result<Id<__RcTestObject>, Id<__RcTestObject>> = unsafe {
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
            $expected.drop += 1;
            $expected.assert_current();
        }
    }

    #[test]
    fn test_error_id() {
        let mut expected = __ThreadTestData::current();

        let cls = __RcTestObject::class();
        test_error_id!(expected, IF_AUTORELEASE_NOT_SKIPPED, idAndShouldError, cls);
        test_error_id!(expected, 0, newAndShouldError, cls);

        let obj = __RcTestObject::new();
        expected.alloc += 1;
        expected.init += 1;
        test_error_id!(
            expected,
            IF_AUTORELEASE_NOT_SKIPPED_ARM_HACK,
            idAndShouldError,
            &obj
        );

        expected.alloc -= 1;
        expected.release -= 1;
        test_error_id!(expected, 0, initAndShouldError, {
            expected.alloc += 1;
            expected.release += 1;
            // Drop flag ensures newly allocated objects do not drop
            // expected.drop += 1;
            __RcTestObject::alloc()
        });
    }

    #[test]
    fn test_error_alloc() {
        let mut expected = __ThreadTestData::current();

        // Succeeds
        let mut error: Option<Id<__RcTestObject>> = None;
        let res: Allocated<__RcTestObject> = unsafe {
            msg_send_id![__RcTestObject::class(), allocAndShouldError: false, error: &mut error]
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
        let res: Id<__RcTestObject> = autoreleasepool(|_pool| {
            let mut error = None;
            let res: Allocated<__RcTestObject> = unsafe {
                msg_send_id![__RcTestObject::class(), allocAndShouldError: true, error: &mut error]
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

    #[test]
    fn test_method_id_with_param() {
        let mut expected = __ThreadTestData::current();

        let obj = __RcTestObject::new();
        expected.alloc += 1;
        expected.init += 1;
        expected.assert_current();

        let res: Option<Id<__RcTestObject>> = unsafe { msg_send_id![&obj, aMethod: false] };
        assert!(res.is_none());
        expected.assert_current();

        let _res = autoreleasepool(|_pool| {
            let res: Option<Id<__RcTestObject>> = unsafe { msg_send_id![&obj, aMethod: true] };
            assert!(res.is_some());
            expected.alloc += 1;
            expected.init += 1;
            expected.autorelease += IF_AUTORELEASE_NOT_SKIPPED_ARM_HACK;
            expected.retain += IF_AUTORELEASE_NOT_SKIPPED_ARM_HACK;
            expected.assert_current();
            res
        });
        expected.release += IF_AUTORELEASE_NOT_SKIPPED_ARM_HACK;
        expected.assert_current();
    }
}
