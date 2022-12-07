use objc2::{extern_class, ClassType};
use objc2::runtime::NSObject;

extern_class!(
    pub struct NSNumber {
        var: u32,
    }

    unsafe impl ClassType for NSNumber {
        type Super = NSObject;
    }
);

fn main() {}
