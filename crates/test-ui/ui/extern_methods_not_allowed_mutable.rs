//! Test extern_methods! with mutable receivers that are not IsAllowedMutable.
use objc2::rc::Id;
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
        #[method(test)]
        fn test(&mut self);
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id(testId)]
        fn test_id(&mut self) -> Id<Self>;
    }
);

fn main() {}
