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
        #[method(a)]
        #[method(a)]
        fn a();
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id(b)]
        #[method_id(b)]
        fn b();
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(c)]
        #[method_id(c)]
        fn c();
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id(d)]
        #[method(d)]
        fn d();
    }
);

fn main() {}
