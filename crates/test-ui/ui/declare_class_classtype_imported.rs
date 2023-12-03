#![allow(unused_imports)]
use objc2::runtime::NSObject;
use objc2::{declare_class, mutability};

declare_class!(
    struct CustomObject;

    unsafe impl objc2::ClassType for CustomObject {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "CustomObject";
    }

    impl objc2::DeclaredClass for CustomObject {}
);

fn main() {}
