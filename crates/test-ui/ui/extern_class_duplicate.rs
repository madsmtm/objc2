//! Test duplicate fields in extern_class!
use objc2::runtime::NSObject;
use objc2::{extern_class, MainThreadOnly};

extern_class!(
    #[unsafe(super(NSObject))]
    #[unsafe(super(NSObject))]
    struct DuplicateSuper;
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[name = "Abc"]
    #[name = "Abc"]
    struct DuplicateName;
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[thread_kind = MainThreadOnly]
    struct DuplicateThreadKind;
);

fn main() {}
