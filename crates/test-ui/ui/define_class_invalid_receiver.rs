#![allow(unused_variables)]
use objc2::define_class;
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, NSObject};

define_class!(
    #[unsafe(super(NSObject))]
    struct TestBox;

    impl TestBox {
        #[unsafe(method(testBox))]
        fn test_box(self: Box<Self>) {
            unimplemented!()
        }
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct TestRetainedSelf;

    impl TestRetainedSelf {
        #[unsafe(method(testRetainedSelf))]
        fn test_retained_self(this: Retained<Self>) {
            unimplemented!()
        }
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct TestSelf;

    impl TestSelf {
        #[unsafe(method(testSelf))]
        fn test_self(this: Self) {
            unimplemented!()
        }
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct TestClass;

    impl TestClass {
        #[unsafe(method(testClass))]
        fn test_class(this: &AnyClass) {
            unimplemented!()
        }
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct TestObject;

    impl TestObject {
        #[unsafe(method(testObject))]
        fn test_object(this: &NSObject) {
            unimplemented!()
        }
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct TestBoxReturnRetained;

    impl TestBoxReturnRetained {
        #[unsafe(method(testBoxRetained))]
        fn test_box_retained(self: Box<Self>) -> Retained<Self> {
            unimplemented!()
        }
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct TestRetainedReturnRetained;

    impl TestRetainedReturnRetained {
        #[unsafe(method(testRetainedSelfRetained))]
        fn test_retained_self_retained(this: Retained<Self>) -> Retained<Self> {
            unimplemented!()
        }
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct NormalNotInit;

    impl NormalNotInit {
        #[unsafe(method(testAlloc))]
        fn test_alloc(this: Allocated<Self>) -> Retained<Self> {
            unimplemented!()
        }
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct InitNotAlloc;

    impl InitNotAlloc {
        #[unsafe(method(initTestNotAlloc))]
        fn test_init_not_alloc(&self) -> Retained<Self> {
            unimplemented!()
        }
    }
);

fn main() {}
