use objc2::{declare_class, ClassType};
use objc2::rc::{Allocated, Id};
use objc2::runtime::NSObject;

declare_class!(
    struct CustomObject;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
        const NAME: &'static str = "CustomObject";
    }

    unsafe impl CustomObject {
        #[method(test1)]
        fn test1(self: Box<Self>) {
            unimplemented!()
        }

        #[method(test2)]
        fn test2(this: Id<Self>) {
            unimplemented!()
        }

        #[method(test3)]
        fn test3(this: Self) {
            unimplemented!()
        }
    }

    unsafe impl CustomObject {
        #[method_id(test4)]
        fn test4(self: Box<Self>) -> Id<Self> {
            unimplemented!()
        }

        #[method_id(test5)]
        fn test5(this: Id<Self>) -> Id<Self> {
            unimplemented!()
        }

        #[method_id(test6)]
        fn test6(this: Self) -> Id<Self> {
            unimplemented!()
        }
    }

    unsafe impl CustomObject {
        #[method_id(test7)]
        fn test7(this: Allocated<Self>) -> Id<Self> {
            unimplemented!()
        }

        #[method_id(initTest8)]
        fn test8(&self) -> Id<Self> {
            unimplemented!()
        }
    }
);

fn main() {}
