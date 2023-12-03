use objc2::rc::{Allocated, Id};
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
        fn test_id_self(this: Id<Self>) {
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
        fn test_box_id(self: Box<Self>) -> Id<Self> {
            unimplemented!()
        }

        #[method_id(testIdSelfId)]
        fn test_id_self_id(this: Id<Self>) -> Id<Self> {
            unimplemented!()
        }

        #[method_id(testSelfId)]
        fn test_self_id(this: Self) -> Id<Self> {
            unimplemented!()
        }
    }

    unsafe impl CustomObject {
        #[method_id(testAlloc)]
        fn test_alloc(this: Allocated<Self>) -> Id<Self> {
            unimplemented!()
        }

        #[method_id(initTestNotAlloc)]
        fn test_init_not_alloc(&self) -> Id<Self> {
            unimplemented!()
        }
    }
);

fn main() {}
