//! Check that `mutability` traits are not implementable manually.
use objc2::runtime::NSObject;
use objc2::{declare_class, mutability, ClassType, DeclaredClass};

declare_class!(
    struct CustomObject;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "CustomObject";
    }

    impl DeclaredClass for CustomObject {}
);

unsafe impl mutability::IsMutable for CustomObject {}

fn main() {}
