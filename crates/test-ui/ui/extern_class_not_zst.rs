use objc2::runtime::NSObject;
use objc2::{extern_class, ClassType};

extern_class!(
    pub struct NSNumber {
        var: u32,
    }

    unsafe impl ClassType for NSNumber {
        type Super = NSObject;
    }
);

fn main() {}
