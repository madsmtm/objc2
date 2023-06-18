use objc2::runtime::AnyObject;
use objc2::{extern_class, mutability, ClassType};

extern_class!(
    pub struct MyRootClass;

    unsafe impl ClassType for MyRootClass {
        type Super = AnyObject;
        type Mutability = mutability::InteriorMutable;
    }
);

fn main() {}
