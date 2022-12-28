use objc2::{extern_class, extern_methods, ClassType};
use objc2::rc::Id;
use objc2::runtime::NSObject;

extern_class!(
    pub struct MyObject;

    unsafe impl ClassType for MyObject {
        type Super = NSObject;
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

fn main() {}
