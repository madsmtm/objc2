use objc2::runtime::Object;
use objc2::{extern_class, mutability, ClassType};

extern_class!(
    pub struct MyRootClass;

    unsafe impl ClassType for MyRootClass {
        type Super = Object;
        type Mutability = mutability::InteriorMutable;
    }
);

fn main() {}
