//! Check that `MainThreadOnly`/`AllocAnyThread` traits are not implementable manually.
use objc2::runtime::NSObject;
use objc2::{define_class, AllocAnyThread, MainThreadOnly};

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = AllocAnyThread]
    #[name = "Normal"]
    #[ivars = *mut ()]
    struct Normal;
);

unsafe impl MainThreadOnly for Normal {}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "OnlyMain"]
    struct OnlyMain;
);

unsafe impl AllocAnyThread for OnlyMain {}

fn main() {}
