use objc2::runtime::AnyObject;
use objc2::{extern_class, AnyThread};

extern_class!(
    #[unsafe(super(AnyObject))]
    #[thread_kind = AnyThread]
    pub struct MyRootClass;
);

fn main() {}
