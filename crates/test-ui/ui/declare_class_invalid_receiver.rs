use objc2::rc::{Id, Shared};
use objc2::{declare_class, ClassType};
use objc2::runtime::NSObject;

declare_class!(
    struct CustomObject;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
    }

    unsafe impl CustomObject {
        #[method(test1)]
        fn test1(self: Box<Self>) {
            unimplemented!()
        }

        #[method(test2)]
        fn test2(this: Id<Self, Shared>) {
            unimplemented!()
        }

        #[method(test3)]
        fn test3(this: Self) {
            unimplemented!()
        }
    }
);

fn main() {}
