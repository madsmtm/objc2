//! Test extern_methods! with mutable receivers.
use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
);

impl MyObject {
    extern_methods!(
        #[unsafe(method(test))]
        fn test(&mut self);
    );
}

impl MyObject {
    extern_methods!(
        #[unsafe(method(testRetained))]
        fn test_retained(&mut self) -> Retained<Self>;
    );
}

fn main() {}
