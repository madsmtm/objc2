//! Check that `MainThreadOnly`/`AllocAnyThread` traits are not implementable manually.
use objc2::runtime::NSObject;
use objc2::{define_class, AllocAnyThread, MainThreadOnly};

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = AllocAnyThread]
    #[ivars = *mut ()]
    struct Normal;
);

unsafe impl MainThreadOnly for Normal {}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    struct OnlyMain;
);

unsafe impl AllocAnyThread for OnlyMain {}

fn main() {}
