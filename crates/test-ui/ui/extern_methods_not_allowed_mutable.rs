//! Test extern_methods! with mutable receivers.
use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
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
