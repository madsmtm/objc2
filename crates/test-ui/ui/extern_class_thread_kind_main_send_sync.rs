use objc2::runtime::NSObject;
use objc2::{extern_class, MainThreadOnly};

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    struct SendAndOnlyMain;
);

unsafe impl Send for SendAndOnlyMain {}

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    struct SyncAndOnlyMain;
);

unsafe impl Sync for SyncAndOnlyMain {}

fn main() {}
