use core::panic::AssertUnwindSafe;
use std::panic::catch_unwind;

use objc2::{
    define_class, extern_class, extern_methods,
    rc::{autoreleasepool, Retained},
    ClassType, Message,
};
use objc2_foundation::NSObject;

use crate::rc_test_object::{RcTestObject, ThreadTestData};

extern_class!(
    #[unsafe(super(NSObject))]
    struct Writeback;
);

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "WritebackRust"]
    struct WritebackRust;

    /// This doc comment is here to make rustfmt work.
    impl WritebackRust {
        #[unsafe(method(write:toParamNonNullNonNull:))]
        fn _write_nonnull_nonnull(obj: Option<&NSObject>, param: &mut Retained<NSObject>) {
            *param = obj.expect("null obj").retain();
        }

        #[unsafe(method(write:toParamNonNullNullable:))]
        fn _write_nonnull_nullable(obj: Option<&NSObject>, param: &mut Option<Retained<NSObject>>) {
            *param = obj.map(|obj| obj.retain());
        }

        #[unsafe(method(write:toParamNullableNonNull:))]
        fn _write_nullable_nonnull(obj: Option<&NSObject>, param: Option<&mut Retained<NSObject>>) {
            if let Some(param) = param {
                *param = obj.expect("null obj").retain();
            }
        }

        #[unsafe(method(write:toParamNullableNullable:))]
        fn _write_nullable_nullable(
            obj: Option<&NSObject>,
            param: Option<&mut Option<Retained<NSObject>>>,
        ) {
            if let Some(param) = param {
                *param = obj.map(|obj| obj.retain());
            }
        }
    }
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

        #[unsafe(method(forward:toParamNonNullNonNull:))]
        fn forward_nonnull_nonnull(obj: Option<&NSObject>, param: &mut Retained<NSObject>);

        #[unsafe(method(forward:toParamNonNullNullable:))]
        fn forward_nonnull_nullable(obj: Option<&NSObject>, param: &mut Option<Retained<NSObject>>);

        #[unsafe(method(forward:toParamNullableNonNull:))]
        fn forward_nullable_nonnull(obj: Option<&NSObject>, param: Option<&mut Retained<NSObject>>);

        #[unsafe(method(forward:toParamNullableNullable:))]
        fn forward_nullable_nullable(
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
    // Ensure `WritebackRust` is defined before this runs.
    let _ = WritebackRust::class();

    let obj = RcTestObject::new();
    let mut expected = ThreadTestData::current();

    let fns: [fn(_, param: &mut Option<_>); _] = [
        |obj, param| Writeback::write_nonnull_nonnull(Some(obj), param.as_mut().unwrap()),
        |obj, param| Writeback::write_nonnull_nullable(Some(obj), param),
        |obj, param| Writeback::write_nullable_nonnull(Some(obj), Some(param.as_mut().unwrap())),
        |obj, param| Writeback::write_nullable_nullable(Some(obj), Some(param)),
        |obj, param| Writeback::forward_nonnull_nonnull(Some(obj), param.as_mut().unwrap()),
        |obj, param| Writeback::forward_nonnull_nullable(Some(obj), param),
        |obj, param| Writeback::forward_nullable_nonnull(Some(obj), Some(param.as_mut().unwrap())),
        |obj, param| Writeback::forward_nullable_nullable(Some(obj), Some(param)),
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

            // `define_class!` will insert additional retain/release calls,
            // and noptimized ARC may do so as well.
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
#[cfg_attr(
    any(feature = "catch-all", panic = "abort"),
    ignore = "panics intentionally"
)]
fn write_null() {
    let obj = RcTestObject::new();
    let mut expected = ThreadTestData::current();

    let fns: [fn(param: &mut Option<Retained<NSObject>>); _] = [
        |param| Writeback::write_nonnull_nonnull(None, param.as_mut().unwrap()),
        |param| Writeback::write_nullable_nonnull(None, Some(param.as_mut().unwrap())),
    ];

    for f in fns {
        let mut param = Some(Retained::into_super(obj.clone()));
        expected.retain += 1;
        expected.assert_current();

        autoreleasepool(|_| {
            let res = catch_unwind(AssertUnwindSafe(|| {
                f(&mut param);
            }));

            expected.release += 1;
            assert_eq!(res.is_err(), cfg!(debug_assertions));

            expected.assert_current();
        });

        drop(param);
        expected.assert_current();
    }

    let fns: [fn(param: &mut Option<Retained<NSObject>>); _] = [
        |param| Writeback::write_nonnull_nullable(None, param),
        |param| Writeback::write_nullable_nullable(None, Some(param)),
    ];

    for f in fns {
        let mut param = Some(Retained::into_super(obj.clone()));
        expected.retain += 1;
        expected.assert_current();

        autoreleasepool(|_| {
            f(&mut param);
            expected.release += 1;
            expected.assert_current();
        });

        drop(param);
        expected.assert_current();
    }
}
