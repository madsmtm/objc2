//! Test extern_class! without any superclasses.
use objc2::{extern_class, MainThreadMarker, MainThreadOnly};

extern_class!(
    #[thread_kind = MainThreadOnly]
    #[name = "NSObject"]
    struct NoSuper;
);

fn main() {
    let _ = NoSuper::alloc(MainThreadMarker::new().unwrap());
}
