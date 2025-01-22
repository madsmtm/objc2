//! Test extern_methods! syntax with `unsafe impl`.
#![deny(deprecated)]
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
);

extern_methods!(
    unsafe impl MyObject {
        #[unsafe(method(foo))]
        fn foo();
    }
);

fn main() {}
