//! Test using deprecated `#[unsafe(method_id(...))]`.
#![deny(warnings)]
use objc2::define_class;
use objc2::rc::Retained;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    struct MyObject;

    impl MyObject {
        #[unsafe(method_id(myMethod:))]
        fn my_method(_param: i32) -> Retained<Self> {
            unimplemented!()
        }
    }
);

fn main() {}
