use objc2::runtime::NSObject;
use objc2::{declare_class, mutability, ClassType, DeclaredClass};

declare_class!(
    struct MyMainThreadOnlyClass;

    unsafe impl ClassType for MyMainThreadOnlyClass {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "MyMainThreadOnlyClass";
    }

    impl DeclaredClass for MyMainThreadOnlyClass {}
);

fn main() {
    let _ = MyMainThreadOnlyClass::alloc();
}
