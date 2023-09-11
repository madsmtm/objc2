//! Check that `mutability` traits are not implementable manually.
use objc2::runtime::NSObject;
use objc2::{declare_class, mutability, ClassType};

declare_class!(
    struct CustomObject;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "CustomObject";
    }
);

unsafe impl mutability::IsMutable for CustomObject {}

fn main() {}
