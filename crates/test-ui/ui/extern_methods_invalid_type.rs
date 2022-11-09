use objc2::{extern_class, extern_methods, ClassType};
use objc2::foundation::NSObject;
use objc2::rc::{Id, Owned};

extern_class!(
    pub struct MyObject;

    unsafe impl ClassType for MyObject {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(a)]
        fn a(&self) -> Id<Self, Owned>;
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
        fn init(&mut self) -> Option<Id<Self, Owned>>;
    }
);

fn main() {}
