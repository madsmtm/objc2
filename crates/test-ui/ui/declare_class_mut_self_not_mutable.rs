use objc2::rc::Id;
use objc2::runtime::NSObject;
use objc2::{declare_class, mutability, ClassType};

declare_class!(
    struct CustomObject;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "CustomObject";
    }

    unsafe impl CustomObject {
        #[method(initTest)]
        fn init_test(&mut self) -> &mut Self {
            unimplemented!()
        }

        #[method(testMethod)]
        fn test_method(&mut self) {
            unimplemented!()
        }

        #[method_id(testMethodId)]
        fn test_method_id(&mut self) -> Id<Self> {
            unimplemented!()
        }
    }
);

fn main() {}
