use objc2::define_class;
use objc2::rc::Retained;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "CustomObject"]
    struct CustomObject;

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
