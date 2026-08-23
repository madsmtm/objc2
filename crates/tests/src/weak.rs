use std::pin::Pin;

use objc2::rc::{Allocated, Retained, Weak};
use objc2::{extern_class, extern_methods, AnyThread, ClassType};
use objc2_foundation::NSObject;

use crate::rc_test_object::{RcTestObject, ThreadTestData};

extern_class!(
    #[unsafe(super(NSObject))]
    struct HasWeakIvar;
);

#[allow(non_snake_case)]
impl HasWeakIvar {
    extern_methods!(
        #[unsafe(method(initWithObject:))]
        fn init(this: Allocated<Self>, object: &NSObject) -> Retained<Self>;

        #[unsafe(method(copyObject))]
        fn copyObject(&self) -> Option<Retained<NSObject>>;

        #[unsafe(method(rawObject))]
        fn rawObject(&self) -> &*mut NSObject;
    );

    fn ivar(&self) -> Pin<&Weak<NSObject>> {
        let ivar = Self::class().instance_variable(c"_object").unwrap();
        let ptr = unsafe { ivar.load::<*mut NSObject>(self) };
        // TODO: Allow loading ivars that only have `RefEncode` impl?
        let ptr = unsafe { core::mem::transmute::<&*mut NSObject, &Weak<NSObject>>(ptr) };
        unsafe { Pin::new_unchecked(ptr) }
    }

    fn ivar_ptr(&self) -> Pin<&&Weak<NSObject>> {
        let ivar = Self::class().instance_variable(c"_objPtr").unwrap();
        let ptr = unsafe { ivar.load::<&*mut NSObject>(self) };
        // TODO: Impl RefEncode for Weak?
        let ptr = unsafe { core::mem::transmute::<&&*mut NSObject, &&Weak<NSObject>>(ptr) };
        unsafe { Pin::new_unchecked(ptr) }
    }
}

#[cfg(all(target_vendor = "apple", not(target_arch = "x86")))]
#[used]
static FIX_LINKING: &objc2::runtime::AnyClass = {
    extern "C" {
        #[link_name = "OBJC_CLASS_$_HasWeakIvar"]
        static CLASS: objc2::runtime::AnyClass;
    }
    unsafe { &CLASS }
};

#[test]
fn weak() {
    let obj = RcTestObject::new().into_super();
    let mut expected = ThreadTestData::current();

    let container = HasWeakIvar::init(HasWeakIvar::alloc(), &obj);
    expected.retain += 1;
    expected.release += 1;
    expected.assert_current();

    assert_eq!(container.copyObject().unwrap(), obj);
    expected.try_retain += 1;
    expected.release += 1;
    expected.assert_current();

    assert_eq!(container.ivar().load().unwrap(), obj);
    expected.try_retain += 1;
    expected.release += 1;
    expected.assert_current();

    container.ivar().store(None);
    assert_eq!(container.copyObject(), None);
    assert_eq!(container.ivar().load(), None);
    expected.assert_current();

    let ptr = unsafe { container.ivar_ptr().map_unchecked(|x| &**x) };
    ptr.store(Some(&obj));
    assert_eq!(container.ivar().load().unwrap(), obj);
    expected.try_retain += 1;
    expected.release += 1;
    expected.assert_current();

    let raw = container.rawObject();
    let raw = unsafe { core::mem::transmute::<&*mut NSObject, &Weak<NSObject>>(raw) };
    let raw = unsafe { Pin::new_unchecked(raw) };
    expected.assert_current();

    assert_eq!(raw.load().unwrap(), obj);
    expected.try_retain += 1;
    expected.release += 1;
    expected.assert_current();

    container.ivar().store(None);
    raw.load();
    assert_eq!(raw.load(), None);
    expected.assert_current();
}
