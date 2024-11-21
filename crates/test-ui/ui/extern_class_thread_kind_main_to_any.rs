use objc2::runtime::NSObject;
use objc2::{extern_class, AllocAnyThread, MainThreadOnly};

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    struct OnlyMain;
);

extern_class!(
    #[unsafe(super(OnlyMain))]
    #[thread_kind = AllocAnyThread]
    struct AnyThreadButSubclassesOnlyMain;
);

fn main() {}
