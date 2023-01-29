use objc2::rc::{Allocated, Id, Shared};
use objc2::{declare_class, ClassType};
use objc2::runtime::NSObject;

declare_class!(
    struct CustomObject;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
        const NAME: &'static str = "CustomObject";
    }

    unsafe impl CustomObject {
        #[method_id(initNotSameGenerics)]
        fn test_init_not_same_generics(this: Allocated<Self>) -> Id<NSObject, Shared> {
            unimplemented!()
        }

        #[method_id(methodIdNotId)]
        fn test_method_id_not_id(&self) -> i32 {
            unimplemented!()
        }
    }
);

fn main() {}
