use objc2::define_class;
use objc2::rc::{Allocated, Retained};
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    struct CustomObject;

    impl CustomObject {
        #[unsafe(method(initNotSameGenerics))]
        fn test_init_not_same_generics(_this: Allocated<Self>) -> Retained<NSObject> {
            unimplemented!()
        }
    }
);

fn main() {}
