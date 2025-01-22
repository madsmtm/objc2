#![allow(deprecated)]
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
);

impl MyObject {
    extern_methods!(
        #[method(foo)]
        fn method();

        #[method_id(foo)]
        fn method_id();
    );
}

fn main() {}
