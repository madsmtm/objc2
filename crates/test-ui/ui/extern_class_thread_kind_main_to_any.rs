use objc2::runtime::NSObject;
use objc2::{extern_class, AnyThread, MainThreadOnly};

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    struct OnlyMain;
);

extern_class!(
    #[unsafe(super(OnlyMain))]
    #[thread_kind = AnyThread]
    struct AnyThreadButSubclassesOnlyMain;
);

fn main() {}
