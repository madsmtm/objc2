use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{define_class, extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
);

extern_methods!(
    unsafe impl MyObject {
        #[method(a)]
        /// Doc comment
        #[optional]
        fn a(&self);
    }
);

extern_methods!(
    unsafe impl MyObject {
        /// Doc comment
        #[optional]
        #[method(b)]
        fn b(&self) -> Retained<Self>;
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "CustomObject1"]
    struct CustomObject1;

    unsafe impl CustomObject1 {
        #[method(c)]
        #[optional]
        /// Doc comment
        fn c(&self) {}
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "CustomObject2"]
    struct CustomObject2;

    unsafe impl CustomObject2 {
        #[optional]
        /// Doc comment
        #[method_id(d)]
        fn d(&self) -> Retained<Self> {
            unimplemented!()
        }
    }
);

fn main() {}
