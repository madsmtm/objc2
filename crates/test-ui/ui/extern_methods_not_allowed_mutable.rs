//! Test extern_methods! with mutable receivers that are not IsAllowedMutable.
use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods, ClassType};

extern_class!(
    pub struct MyObject;

    unsafe impl ClassType for MyObject {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(test)]
        fn test(&mut self);
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id(testId)]
        fn test_id(&mut self) -> Retained<Self>;
    }
);

fn main() {}
