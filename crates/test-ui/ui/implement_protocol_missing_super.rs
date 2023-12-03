//! Test that implementing traits like `NSApplicationDelegate` requires super
//! protocols like `NSObjectProtocol` to also be implemented.
use icrate::AppKit::NSApplicationDelegate;
use icrate::Foundation::NSObject;
use objc2::{declare_class, mutability, ClassType, DeclaredClass};

declare_class!(
    struct CustomObject;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "CustomObject";
    }

    impl DeclaredClass for CustomObject {}

    unsafe impl NSApplicationDelegate for CustomObject {}
);

fn main() {}
