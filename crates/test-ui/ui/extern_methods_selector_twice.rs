use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
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
