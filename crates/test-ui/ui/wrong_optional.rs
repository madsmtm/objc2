use objc2::{extern_class, extern_methods, declare_class, ClassType};
use objc2::runtime::NSObject;
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
        /// Doc comment
        #[optional]
        fn a(&self);
    }
);

extern_methods!(
    unsafe impl MyObject {
        /// Doc comment
        #[optional]
        #[method_id(b)]
        fn b(&self) -> Id<Self, Owned>;
    }
);

declare_class!(
    struct CustomObject;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
    }

    unsafe impl CustomObject {
        #[method(c)]
        #[optional]
        /// Doc comment
        fn c(&self) {}

        #[optional]
        /// Doc comment
        #[method_id(d)]
        fn d(&self) {}
    }
);

fn main() {}
