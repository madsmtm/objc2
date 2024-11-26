//! Check that `MainThreadOnly`/`AllocAnyThread` traits are not implementable manually.
use objc2::runtime::NSObject;
use objc2::{declare_class, AllocAnyThread, MainThreadOnly};

declare_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = AllocAnyThread]
    #[name = "Normal"]
    #[ivars = *mut ()]
    struct Normal;
);

unsafe impl MainThreadOnly for Normal {}

declare_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "OnlyMain"]
    struct OnlyMain;
);

unsafe impl AllocAnyThread for OnlyMain {}

fn main() {}
