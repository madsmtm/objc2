use objc2::{declare_class, ClassType};
use objc2::runtime::NSObject;

declare_class!(
    struct CustomObject;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
    }

    unsafe impl CustomObject {
        fn test_no_attribute() {
            unimplemented!()
        }

        #[method_id(testMethodId)]
        fn test_method_id() {
            unimplemented!()
        }

        #[method(testInvalid)]
        fn test_invalid() {
            a -
        }

        #[method(testPattern:)]
        fn test_pattern((a, b): (u32, i32)) {
            unimplemented!()
        }

        #[method(testSelf)]
        fn test_self(self) {
            unimplemented!()
        }
    }

    unsafe impl CustomObject {
        #[method(testPub)]
        pub fn test_pub() {}
    }

    unsafe impl CustomObject {
        #[method(testNoBody)]
        fn test_no_body(&self);
    }
);

fn main() {}
