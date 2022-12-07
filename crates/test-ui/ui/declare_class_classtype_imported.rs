use objc2::declare_class;
use objc2::foundation::NSObject;

declare_class!(
    struct CustomObject {}

    unsafe impl objc2::ClassType for CustomObject {
        type Super = NSObject;
    }
);

fn main() {}
