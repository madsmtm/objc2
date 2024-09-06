use objc2::runtime::AnyObject;
use objc2::{extern_class, AllocAnyThread, ClassType};

extern_class!(
    pub struct MyRootClass;

    unsafe impl ClassType for MyRootClass {
        type Super = AnyObject;
        type ThreadKind = dyn AllocAnyThread;
    }
);

fn main() {}
