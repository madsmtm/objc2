//! Check that `MainThreadOnly`/`AllocAnyThread` traits are not implementable manually.
use objc2::runtime::NSObject;
use objc2::{declare_class, AllocAnyThread, ClassType, DeclaredClass, MainThreadOnly};

declare_class!(
    struct Normal;

    unsafe impl ClassType for Normal {
        type Super = NSObject;
        type ThreadKind = dyn AllocAnyThread;
        const NAME: &'static str = "Normal";
    }

    impl DeclaredClass for Normal {
        type Ivars = *mut ();
    }
);

unsafe impl MainThreadOnly for Normal {}

declare_class!(
    struct OnlyMain;

    unsafe impl ClassType for OnlyMain {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
        const NAME: &'static str = "OnlyMain";
    }

    impl DeclaredClass for OnlyMain {}
);

unsafe impl AllocAnyThread for OnlyMain {}

fn main() {}
