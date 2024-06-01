use objc2::rc::Retained;
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
        #[method(initTest)]
        fn init_test(&mut self) -> &mut Self {
            unimplemented!()
        }

        #[method(testMethod)]
        fn test_method(&mut self) {
            unimplemented!()
        }

        #[method_id(testMethodId)]
        fn test_method_id(&mut self) -> Retained<Self> {
            unimplemented!()
        }
    }
);

fn main() {}
