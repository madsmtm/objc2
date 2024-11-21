use objc2::runtime::NSObject;
use objc2::{extern_class, ThreadKind};

extern_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = ThreadKind]
    struct BogusThreadKind;
);

fn main() {}
