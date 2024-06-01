//! Test that implementing certain traits like `NSURLSessionDelegate` requires
//! super protocols like `NSObjectProtocol` to also be implemented.
use objc2::{declare_class, mutability, ClassType, DeclaredClass};
use objc2_foundation::{NSObject, NSURLSessionDelegate};

declare_class!(
    struct CustomObject;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "CustomObject";
    }

    impl DeclaredClass for CustomObject {}

    unsafe impl NSURLSessionDelegate for CustomObject {}
);

fn main() {}
