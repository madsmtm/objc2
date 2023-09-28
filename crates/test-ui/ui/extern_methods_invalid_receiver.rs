use objc2::rc::{Allocated, Id};
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
        #[method_id(initWithMut)]
        fn init_with_mut(&mut self) -> Option<Id<Self>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id(initWithOptionAllocated)]
        fn init_with_option_allocated(this: Option<Allocated<Self>>) -> Option<Id<Self>>;
    }
);

fn main() {}
