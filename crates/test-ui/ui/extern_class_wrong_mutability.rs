use objc2::runtime::NSObject;
use objc2::{extern_class, mutability, ClassType};

extern_class!(
    pub struct MyMainThreadClass;

    unsafe impl ClassType for MyMainThreadClass {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
    }
);

extern_class!(
    pub struct MyAnyThreadClass;

    unsafe impl ClassType for MyAnyThreadClass {
        type Super = MyMainThreadClass;
        type Mutability = mutability::InteriorMutable;
    }
);

fn main() {}
