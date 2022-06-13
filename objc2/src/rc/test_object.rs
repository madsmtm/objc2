use core::cell::RefCell;
use core::ops::{Deref, DerefMut};
use std::sync::Once;

use super::{Id, Owned};
use crate::declare::ClassBuilder;
use crate::runtime::{Bool, Class, Object, Sel};
use crate::{msg_send, msg_send_bool};
use crate::{Encoding, Message, RefEncode};

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct ThreadTestData {
    pub(crate) alloc: usize,
    pub(crate) dealloc: usize,
    pub(crate) init: usize,
    pub(crate) retain: usize,
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

/// A helper object that counts how many times various reference-counting
/// primitives are called.
#[repr(C)]
pub(crate) struct RcTestObject {
    inner: Object,
}

unsafe impl RefEncode for RcTestObject {
    const ENCODING_REF: Encoding<'static> = Object::ENCODING_REF;
}

unsafe impl Message for RcTestObject {}

unsafe impl Send for RcTestObject {}
unsafe impl Sync for RcTestObject {}

impl Deref for RcTestObject {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for RcTestObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl RcTestObject {
    fn class() -> &'static Class {
        static REGISTER_CLASS: Once = Once::new();

        REGISTER_CLASS.call_once(|| {
            extern "C" fn alloc(cls: &Class, _cmd: Sel) -> *mut RcTestObject {
                TEST_DATA.with(|data| data.borrow_mut().alloc += 1);
                let superclass = class!(NSObject).metaclass();
                unsafe { msg_send![super(cls, superclass), alloc] }
            }
            extern "C" fn init(this: &mut RcTestObject, _cmd: Sel) -> *mut RcTestObject {
                TEST_DATA.with(|data| data.borrow_mut().init += 1);
                unsafe { msg_send![super(this, class!(NSObject)), init] }
            }
            extern "C" fn retain(this: &RcTestObject, _cmd: Sel) -> *mut RcTestObject {
                TEST_DATA.with(|data| data.borrow_mut().retain += 1);
                unsafe { msg_send![super(this, class!(NSObject)), retain] }
            }
            extern "C" fn release(this: &RcTestObject, _cmd: Sel) {
                TEST_DATA.with(|data| data.borrow_mut().release += 1);
                unsafe { msg_send![super(this, class!(NSObject)), release] }
            }
            extern "C" fn autorelease(this: &RcTestObject, _cmd: Sel) -> *mut RcTestObject {
                TEST_DATA.with(|data| data.borrow_mut().autorelease += 1);
                unsafe { msg_send![super(this, class!(NSObject)), autorelease] }
            }
            unsafe extern "C" fn dealloc(_this: *mut RcTestObject, _cmd: Sel) {
                TEST_DATA.with(|data| data.borrow_mut().dealloc += 1);
                // Don't call superclass
            }
            unsafe extern "C" fn try_retain(this: &RcTestObject, _cmd: Sel) -> Bool {
                TEST_DATA.with(|data| data.borrow_mut().try_retain += 1);
                let res = unsafe { msg_send_bool![super(this, class!(NSObject)), _tryRetain] };
                if !res {
                    TEST_DATA.with(|data| data.borrow_mut().try_retain -= 1);
                    TEST_DATA.with(|data| data.borrow_mut().try_retain_fail += 1);
                }
                Bool::from(res)
            }

            let mut builder = ClassBuilder::new("RcTestObject", class!(NSObject)).unwrap();
            unsafe {
                builder.add_class_method(
                    sel!(alloc),
                    alloc as extern "C" fn(&Class, Sel) -> *mut RcTestObject,
                );
                builder.add_method(
                    sel!(init),
                    init as extern "C" fn(&mut RcTestObject, Sel) -> _,
                );
                builder.add_method(
                    sel!(retain),
                    retain as extern "C" fn(&RcTestObject, Sel) -> _,
                );
                builder.add_method(
                    sel!(_tryRetain),
                    try_retain as unsafe extern "C" fn(&RcTestObject, Sel) -> Bool,
                );
                builder.add_method(sel!(release), release as extern "C" fn(&RcTestObject, Sel));
                builder.add_method(
                    sel!(autorelease),
                    autorelease as extern "C" fn(&RcTestObject, Sel) -> _,
                );
                builder.add_method(
                    sel!(dealloc),
                    dealloc as unsafe extern "C" fn(*mut RcTestObject, Sel),
                );
            }

            builder.register();
        });

        class!(RcTestObject)
    }

    pub(crate) fn new() -> Id<Self, Owned> {
        unsafe { Id::new(msg_send![Self::class(), new]) }.unwrap()
    }
}
