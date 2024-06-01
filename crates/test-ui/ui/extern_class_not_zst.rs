use objc2::runtime::NSObject;
use objc2::{extern_class, mutability, ClassType};

extern_class!(
    pub struct NSNumber {
        var: u32,
    }

    unsafe impl ClassType for NSNumber {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

fn main() {}
