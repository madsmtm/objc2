//! Test various usage of the method_family attribute.
use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id(noUnsafe)]
        #[method_family = none]
        fn no_unsafe(&self) -> Retained<Self>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id(unknownFamily)]
        #[unsafe(method_family = unknown)]
        fn unknown_family(&self) -> Retained<Self>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(notOnMethodId)]
        #[unsafe(method_family = none)]
        fn not_on_method_id(&self);
    }
);

fn main() {}
