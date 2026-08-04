use core::panic::AssertUnwindSafe;
use std::panic::catch_unwind;

use objc2::{
    extern_class, extern_methods,
    rc::{autoreleasepool, Retained},
};
use objc2_foundation::NSObject;

use crate::rc_test_object::{RcTestObject, ThreadTestData};

extern_class!(
    #[unsafe(super(NSObject))]
    struct Writeback;
);

impl Writeback {
    extern_methods!(
        #[unsafe(method(write:toParam:))]
        fn write_nonnull_nonnull(obj: Option<&NSObject>, param: &mut Retained<NSObject>);

        #[unsafe(method(write:toParam:))]
        fn write_nonnull_nullable(obj: Option<&NSObject>, param: &mut Option<Retained<NSObject>>);

        #[unsafe(method(write:toParam:))]
        fn write_nullable_nonnull(obj: Option<&NSObject>, param: Option<&mut Retained<NSObject>>);

        #[unsafe(method(write:toParam:))]
        fn write_nullable_nullable(
            obj: Option<&NSObject>,
            param: Option<&mut Option<Retained<NSObject>>>,
        );
    );
}

#[cfg(all(target_vendor = "apple", not(target_arch = "x86")))]
#[used]
static FIX_LINKING: &objc2::runtime::AnyClass = {
    extern "C" {
        #[link_name = "OBJC_CLASS_$_Writeback"]
        static CLASS: objc2::runtime::AnyClass;
    }
    unsafe { &CLASS }
};

#[test]
fn write_nonnull() {
    let obj = RcTestObject::new();
    let mut expected = ThreadTestData::current();

    let fns: [fn(obj: &NSObject, param: &mut Option<Retained<NSObject>>); _] = [
        |obj, param| Writeback::write_nonnull_nonnull(Some(obj), param.as_mut().unwrap()),
        |obj, param| Writeback::write_nonnull_nullable(Some(obj), param),
        |obj, param| Writeback::write_nullable_nonnull(Some(obj), Some(param.as_mut().unwrap())),
        |obj, param| Writeback::write_nullable_nullable(Some(obj), Some(param)),
    ];

    for f in fns {
        let mut param = Some(Retained::into_super(obj.clone()));
        expected.retain += 1;
        expected.assert_current();

        autoreleasepool(|_| {
            f(&obj, &mut param);

            // The object is retained and autoreleased by ARC.
            expected.retain += 1;
            expected.autorelease += 1;

            // Then `extern_methods!` will retain the object.
            expected.retain += 1;

            // And finally, `extern_methods!` will release the old param.
            expected.release += 1;

            // Unoptimized ARC may insert additional retain/release calls.
            let extra_retain_release = ThreadTestData::current().release - expected.release;
            expected.retain += extra_retain_release;
            expected.release += extra_retain_release;

            expected.assert_current();
        });
        expected.release += 1;
        expected.assert_current();

        drop(param);
        expected.release += 1;
        expected.assert_current();
    }
}

#[test]
fn write_null() {
    let obj = RcTestObject::new();
    let mut expected = ThreadTestData::current();

    let fns: [(fn(param: &mut Option<Retained<NSObject>>), bool); _] = [
        (
            |param| Writeback::write_nonnull_nonnull(None, param.as_mut().unwrap()),
            true,
        ),
        (
            |param| Writeback::write_nonnull_nullable(None, param),
            false,
        ),
        (
            |param| Writeback::write_nullable_nonnull(None, Some(param.as_mut().unwrap())),
            true,
        ),
        (
            |param| Writeback::write_nullable_nullable(None, Some(param)),
            false,
        ),
    ];

    for (f, will_debug_assert) in fns {
        let mut param = Some(Retained::into_super(obj.clone()));
        expected.retain += 1;
        expected.assert_current();

        autoreleasepool(|_| {
            let res = catch_unwind(AssertUnwindSafe(|| {
                f(&mut param);
            }));

            assert_eq!(res.is_err(), cfg!(debug_assertions) && will_debug_assert);

            expected.release += 1;
            expected.assert_current();
        });

        drop(param);

        expected.assert_current();
    }
}
