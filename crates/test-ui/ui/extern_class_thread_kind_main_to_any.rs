use objc2::runtime::NSObject;
use objc2::{extern_class, AllocAnyThread, ClassType, MainThreadOnly};

extern_class!(
    struct OnlyMain;

    unsafe impl ClassType for OnlyMain {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

extern_class!(
    struct AnyThreadButSubclassesOnlyMain;

    unsafe impl ClassType for AnyThreadButSubclassesOnlyMain {
        type Super = OnlyMain;
        type ThreadKind = dyn AllocAnyThread;
    }
);

fn main() {}
