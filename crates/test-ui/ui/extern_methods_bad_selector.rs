use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
);

extern_methods!(
    unsafe impl MyObject {
        #[unsafe(method())]
        fn no_selector_class();
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[unsafe(method())]
        fn no_selector_self(&self);
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[unsafe(method(_))]
        fn underscore(&self);
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[unsafe(method(ab:c))]
        fn missing_colon(&self);
    }
);

fn main() {}
