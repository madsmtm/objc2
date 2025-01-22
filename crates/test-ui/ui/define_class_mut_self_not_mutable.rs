use objc2::define_class;
use objc2::rc::Retained;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "CustomObject"]
    struct CustomObject;

    impl CustomObject {
        #[unsafe(method(initTest))]
        fn init_test(&mut self) -> &mut Self {
            unimplemented!()
        }

        #[unsafe(method(testMethod))]
        fn test_method(&mut self) {
            unimplemented!()
        }

        #[unsafe(method_id(testRetained))]
        fn test_retained(&mut self) -> Retained<Self> {
            unimplemented!()
        }
    }
);

fn main() {}
