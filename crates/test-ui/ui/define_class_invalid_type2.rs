use objc2::define_class;
use objc2::rc::{Allocated, Retained};
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "CustomObject"]
    struct CustomObject;

    unsafe impl CustomObject {
        #[method_id(initNotSameGenerics)]
        fn test_init_not_same_generics(this: Allocated<Self>) -> Retained<NSObject> {
            unimplemented!()
        }

        #[method_id(methodIdNotId)]
        fn test_method_id_not_id(&self) -> i32 {
            unimplemented!()
        }
    }
);

fn main() {}
