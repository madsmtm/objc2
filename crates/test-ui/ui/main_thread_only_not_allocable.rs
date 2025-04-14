use objc2::runtime::NSObject;
#[expect(unused_imports)]
use objc2::AnyThread;
use objc2::{define_class, MainThreadOnly};

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    struct MyMainThreadOnlyClass;
);

fn main() {
    let _ = MyMainThreadOnlyClass::alloc();
}
