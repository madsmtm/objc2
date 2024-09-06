use objc2::runtime::NSObject;
#[expect(unused_imports)]
use objc2::AllocAnyThread;
use objc2::{declare_class, ClassType, DeclaredClass, MainThreadOnly};

declare_class!(
    struct MyMainThreadOnlyClass;

    unsafe impl ClassType for MyMainThreadOnlyClass {
        type Super = NSObject;
        type ThreadKind = dyn MainThreadOnly;
        const NAME: &'static str = "MyMainThreadOnlyClass";
    }

    impl DeclaredClass for MyMainThreadOnlyClass {}
);

fn main() {
    let _ = MyMainThreadOnlyClass::alloc();
}
