use icrate::Foundation::MainThreadMarker;
use objc2::rc::Id;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods, mutability, ClassType};

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
        fn a(&self) -> Id<Self>;
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
        #[method_id(init)]
        fn init(&mut self) -> Option<Id<Self>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(error:)]
        fn error(arg: i32) -> Result<(), Id<NSObject>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id(error:)]
        fn error_id(arg: i32) -> Result<Id<Self>, Id<NSObject>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(mainThreadMarkerAsReturn)]
        fn main_thread_marker_as_return() -> MainThreadMarker;
    }
);

fn main() {}
