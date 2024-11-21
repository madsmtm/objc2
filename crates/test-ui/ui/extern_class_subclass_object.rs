use objc2::runtime::AnyObject;
use objc2::{extern_class, AllocAnyThread};

extern_class!(
    #[unsafe(super(AnyObject))]
    #[thread_kind = AllocAnyThread]
    pub struct MyRootClass;
);

fn main() {}
