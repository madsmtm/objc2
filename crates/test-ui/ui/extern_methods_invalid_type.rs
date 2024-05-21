use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods, mutability, ClassType};
use objc2_foundation::MainThreadMarker;

extern_class!(
    pub struct MyObject;

    unsafe impl ClassType for MyObject {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(a)]
        fn a(&self) -> Retained<Self>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id(b)]
        fn b(&self) -> i32;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(c:)]
        fn c(&self, arg: Box<i32>);
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(error:)]
        fn error(arg: i32) -> Result<(), Retained<NSObject>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id(error:)]
        fn error_id(arg: i32) -> Result<Retained<Self>, Retained<NSObject>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(mainThreadMarkerAsReturn)]
        fn main_thread_marker_as_return() -> MainThreadMarker;
    }
);

fn main() {}
