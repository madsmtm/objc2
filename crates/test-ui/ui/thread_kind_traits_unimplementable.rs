//! Check that `MainThreadOnly`/`AnyThread` traits are not implementable manually.
use objc2::runtime::NSObject;
use objc2::{define_class, AnyThread, MainThreadOnly};

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = AnyThread]
    struct Normal {
        marker: *mut (),
    }
);

unsafe impl MainThreadOnly for Normal {}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    struct OnlyMain;
);

unsafe impl AnyThread for OnlyMain {}

fn main() {}
