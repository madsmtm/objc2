//! Test using deprecated `#[method_id(...)]`.
#![deny(warnings)]
use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id(myMethod:)]
        fn my_method(param: i32) -> Retained<Self>;
    }
);

fn main() {}
