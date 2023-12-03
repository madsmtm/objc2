use objc2::rc::{Allocated, Id};
use objc2::runtime::NSObject;
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
        #[method_id(initNotSameGenerics)]
        fn test_init_not_same_generics(this: Allocated<Self>) -> Id<NSObject> {
            unimplemented!()
        }

        #[method_id(methodIdNotId)]
        fn test_method_id_not_id(&self) -> i32 {
            unimplemented!()
        }
    }
);

fn main() {}
