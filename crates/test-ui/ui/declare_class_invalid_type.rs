use objc2::{declare_class, ClassType};
use objc2::rc::Id;
use objc2::runtime::NSObject;

declare_class!(
    struct CustomObject;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
        const NAME: &'static str = "CustomObject";
    }

    unsafe impl CustomObject {
        #[method(test1)]
        fn test1() -> Id<Self> {
            unimplemented!()
        }

        #[method(test2)]
        fn test2() -> Vec<()> {
            unimplemented!()
        }

        #[method(test3)]
        fn test3(&self, arg: Box<u32>) {
            unimplemented!()
        }

        #[method(test4)]
        fn test4(&self, arg: Self) {
            unimplemented!()
        }
    }
);

fn main() {}
