use objc2::{declare_class, ClassType};
use objc2::runtime::NSObject;
use objc2::rc::{Id, Shared};

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
        fn test_method_id_no_return() {
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

    unsafe impl CustomObject {
        #[method_id(alloc)]
        fn test_method_id_bad_selector1() -> Id<Self, Shared> {
            unimplemented!()
        }

        #[method_id(retain)]
        fn test_method_id_bad_selector2() -> Id<Self, Shared> {
            unimplemented!()
        }

        #[method_id(release)]
        fn test_method_id_bad_selector3() -> Id<Self, Shared> {
            unimplemented!()
        }

        #[method_id(autorelease)]
        fn test_method_id_bad_selector4() -> Id<Self, Shared> {
            unimplemented!()
        }

        #[method_id(dealloc)]
        fn test_method_id_bad_selector5() -> Id<Self, Shared> {
            unimplemented!()
        }
    }
);

fn main() {}
