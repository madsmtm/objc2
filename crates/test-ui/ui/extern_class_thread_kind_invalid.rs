use objc2::runtime::NSObject;
use objc2::{extern_class, ClassType, ThreadKind};

extern_class!(
    struct BogusThreadKind;

    unsafe impl ClassType for BogusThreadKind {
        type Super = NSObject;
        type ThreadKind = dyn ThreadKind;
    }
);

fn main() {}
