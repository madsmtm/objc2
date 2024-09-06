use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods, ClassType};

extern_class!(
    pub struct MyObject;

    unsafe impl ClassType for MyObject {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl MyObject {
        fn a();
    }
);

fn main() {}
