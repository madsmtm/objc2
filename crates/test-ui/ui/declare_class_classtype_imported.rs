#![allow(unused_imports)]
use objc2::declare_class;
use objc2::runtime::NSObject;

declare_class!(
    struct CustomObject;

    unsafe impl objc2::ClassType for CustomObject {
        type Super = NSObject;
        const NAME: &'static str = "CustomObject";
    }

    impl objc2::DeclaredClass for CustomObject {}
);

fn main() {}
