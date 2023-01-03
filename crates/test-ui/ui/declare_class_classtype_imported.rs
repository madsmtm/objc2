use objc2::declare_class;
#[allow(unused_imports)]
use objc2::runtime::NSObject;

declare_class!(
    struct CustomObject;

    unsafe impl objc2::ClassType for CustomObject {
        type Super = NSObject;
    }
);

fn main() {}
