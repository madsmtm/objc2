use objc2::declare::IvarEncode;
use objc2::runtime::NSObject;
use objc2::{declare_class, mutability, ClassType};

declare_class!(
    struct CustomObject {
        field: IvarEncode<(), "_field">,
    }

    mod ivars;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "CustomObject";
    }
);

fn main() {}
