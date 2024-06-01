#![allow(unused_variables)]
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, NSObject};
use objc2::{declare_class, mutability, ClassType, DeclaredClass};

declare_class!(
    struct CustomObject;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "CustomObject";
    }

    impl DeclaredClass for CustomObject {}

    unsafe impl CustomObject {
        #[method(testBox)]
        fn test_box(self: Box<Self>) {
            unimplemented!()
        }

        #[method(testIdSelf)]
        fn test_id_self(this: Retained<Self>) {
            unimplemented!()
        }

        #[method(testSelf)]
        fn test_self(this: Self) {
            unimplemented!()
        }

        #[method(testClass)]
        fn test_class(this: &AnyClass) {
            unimplemented!()
        }

        #[method(testClass)]
        fn test_object(this: &NSObject) {
            unimplemented!()
        }
    }

    unsafe impl CustomObject {
        #[method_id(testBoxId)]
        fn test_box_id(self: Box<Self>) -> Retained<Self> {
            unimplemented!()
        }

        #[method_id(testIdSelfId)]
        fn test_id_self_id(this: Retained<Self>) -> Retained<Self> {
            unimplemented!()
        }

        #[method_id(testSelfId)]
        fn test_self_id(this: Self) -> Retained<Self> {
            unimplemented!()
        }
    }

    unsafe impl CustomObject {
        #[method_id(testAlloc)]
        fn test_alloc(this: Allocated<Self>) -> Retained<Self> {
            unimplemented!()
        }

        #[method_id(initTestNotAlloc)]
        fn test_init_not_alloc(&self) -> Retained<Self> {
            unimplemented!()
        }
    }
);

fn main() {}
