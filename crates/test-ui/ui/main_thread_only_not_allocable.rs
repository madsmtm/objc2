use objc2::runtime::NSObject;
use objc2::{declare_class, mutability, ClassType};

declare_class!(
    struct MyMainThreadOnlyClass;

    unsafe impl ClassType for MyMainThreadOnlyClass {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "MyMainThreadOnlyClass";
    }
);

fn main() {
    let _ = MyMainThreadOnlyClass::alloc();
}
