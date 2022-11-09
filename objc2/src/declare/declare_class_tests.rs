#![deny(deprecated)]
use crate::foundation::NSObject;
use crate::rc::{Id, Owned};
use crate::{declare_class, extern_methods, sel, ClassType};

// Test that adding the `deprecated` attribute does not mean that warnings
// when using the method internally are output.
declare_class!(
    struct DeclareClassDepreactedMethod {}

    unsafe impl ClassType for DeclareClassDepreactedMethod {
        type Super = NSObject;
    }

    #[deprecated]
    unsafe impl DeclareClassDepreactedMethod {
        #[method(deprecatedOnImpl)]
        fn deprecated_on_impl() {}
    }

    unsafe impl DeclareClassDepreactedMethod {
        #[deprecated]
        #[method(deprecatedOnMethod)]
        fn deprecated_on_method() {}
    }
);

#[test]
fn test_deprecated() {
    let _cls = DeclareClassDepreactedMethod::class();
}

// Test that `cfg` works properly.
//
// We use `debug_assertions` here just because it's something that we know
// our CI already tests.
declare_class!(
    struct DeclareClassCfg {}

    unsafe impl ClassType for DeclareClassCfg {
        type Super = NSObject;
    }

    unsafe impl DeclareClassCfg {
        #[cfg(debug_assertions)]
        #[method(changesOnCfg1)]
        fn _changes_on_cfg1() -> i32 {
            1
        }

        #[cfg(not(debug_assertions))]
        #[method(changesOnCfg1)]
        fn _changes_on_cfg1() -> i32 {
            2
        }

        #[cfg(debug_assertions)]
        #[method(onlyWhenEnabled1)]
        fn _only_when_enabled1(&self) {}

        #[cfg(not(debug_assertions))]
        #[method(onlyWhenDisabled1)]
        fn _only_when_disabled1(&self) {}
    }

    #[cfg(debug_assertions)]
    unsafe impl DeclareClassCfg {
        #[method(changesOnCfg2)]
        fn _changes_on_cfg2(&self) -> i32 {
            1
        }

        #[method(onlyWhenEnabled2)]
        fn _only_when_enabled2() {}
    }

    #[cfg(not(debug_assertions))]
    unsafe impl DeclareClassCfg {
        #[method(changesOnCfg2)]
        fn _changes_on_cfg2(&self) -> i32 {
            2
        }

        #[method(onlyWhenDisabled2)]
        fn _only_when_disabled2() {}
    }

    #[cfg(debug_assertions)]
    unsafe impl DeclareClassCfg {
        #[cfg(not(debug_assertions))]
        #[method(never)]
        fn _never(&self) {}

        #[cfg(not(debug_assertions))]
        #[method(never)]
        fn _never_class() {}
    }
);

extern_methods!(
    unsafe impl DeclareClassCfg {
        #[method_id(new)]
        fn new() -> Id<Self, Owned>;
    }

    unsafe impl DeclareClassCfg {
        #[method(changesOnCfg1)]
        fn changes_on_cfg1() -> i32;

        #[method(changesOnCfg2)]
        fn changes_on_cfg2(&self) -> i32;

        #[cfg(debug_assertions)]
        #[method(onlyWhenEnabled1)]
        fn only_when_enabled1(&self);

        #[cfg(not(debug_assertions))]
        #[method(onlyWhenDisabled1)]
        fn only_when_disabled1(&self);
    }

    #[cfg(debug_assertions)]
    unsafe impl DeclareClassCfg {
        #[method(onlyWhenEnabled2)]
        fn only_when_enabled2();
    }

    #[cfg(not(debug_assertions))]
    unsafe impl DeclareClassCfg {
        #[method(onlyWhenDisabled2)]
        fn only_when_disabled2();
    }
);

#[test]
fn test_method_that_changes_based_on_cfg() {
    let expected = if cfg!(debug_assertions) { 1 } else { 2 };
    let actual = DeclareClassCfg::changes_on_cfg1();
    assert_eq!(expected, actual, "changes_on_cfg1");

    let actual = DeclareClassCfg::new().changes_on_cfg2();
    assert_eq!(expected, actual, "changes_on_cfg2");
}

#[test]
fn test_method_that_is_only_available_based_on_cfg() {
    let cls = DeclareClassCfg::class();
    let metacls = cls.metaclass();
    let obj = DeclareClassCfg::new();

    #[cfg(debug_assertions)]
    {
        assert!(!cls.responds_to(sel!(onlyWhenDisabled1)));
        assert!(!metacls.responds_to(sel!(onlyWhenDisabled2)));

        obj.only_when_enabled1();
        DeclareClassCfg::only_when_enabled2();
    }
    #[cfg(not(debug_assertions))]
    {
        assert!(!cls.responds_to(sel!(onlyWhenEnabled1)));
        assert!(!metacls.responds_to(sel!(onlyWhenEnabled2)));

        obj.only_when_disabled1();
        DeclareClassCfg::only_when_disabled2();
    }
}

#[test]
fn test_method_that_is_never_available() {
    let cls = DeclareClassCfg::class();
    let metacls = cls.metaclass();
    assert!(!cls.responds_to(sel!(never)));
    assert!(!metacls.responds_to(sel!(never)));
}
