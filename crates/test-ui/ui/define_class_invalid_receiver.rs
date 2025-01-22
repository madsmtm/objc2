#![allow(unused_variables)]
use objc2::define_class;
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, NSObject};

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "CustomObject"]
    struct CustomObject;

    impl CustomObject {
        #[unsafe(method(testBox))]
        fn test_box(self: Box<Self>) {
            unimplemented!()
        }

        #[unsafe(method(testRetainedSelf))]
        fn test_retained_self(this: Retained<Self>) {
            unimplemented!()
        }

        #[unsafe(method(testSelf))]
        fn test_self(this: Self) {
            unimplemented!()
        }

        #[unsafe(method(testClass))]
        fn test_class(this: &AnyClass) {
            unimplemented!()
        }

        #[unsafe(method(testClass))]
        fn test_object(this: &NSObject) {
            unimplemented!()
        }
    }

    impl CustomObject {
        #[unsafe(method_id(testBoxRetained))]
        fn test_box_retained(self: Box<Self>) -> Retained<Self> {
            unimplemented!()
        }

        #[unsafe(method_id(testRetainedSelfRetained))]
        fn test_retained_self_retained(this: Retained<Self>) -> Retained<Self> {
            unimplemented!()
        }

        #[unsafe(method_id(testSelfRetained))]
        fn test_self_retained(this: Self) -> Retained<Self> {
            unimplemented!()
        }
    }

    impl CustomObject {
        #[unsafe(method_id(testAlloc))]
        fn test_alloc(this: Allocated<Self>) -> Retained<Self> {
            unimplemented!()
        }

        #[unsafe(method_id(initTestNotAlloc))]
        fn test_init_not_alloc(&self) -> Retained<Self> {
            unimplemented!()
        }
    }
);

fn main() {}
