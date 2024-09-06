use objc2::runtime::NSObject;
use objc2::{extern_class, ClassType, MainThreadOnly};

extern_class!(
    struct SendAndOnlyMain;

    unsafe impl ClassType for SendAndOnlyMain {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl Send for SendAndOnlyMain {}

extern_class!(
    struct SyncAndOnlyMain;

    unsafe impl ClassType for SyncAndOnlyMain {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
    }
);

unsafe impl Sync for SyncAndOnlyMain {}

fn main() {}
