use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods, mutability, ClassType};

extern_class!(
    pub struct MyObject;

    unsafe impl ClassType for MyObject {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method()]
        fn no_selector_class();
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id()]
        fn no_selector_id_self(&self);
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(_)]
        fn underscore(&self);
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(ab:c)]
        fn missing_colon(&self);
    }
);

fn main() {}
