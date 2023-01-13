use objc2::{extern_class, extern_methods, ClassType};
use objc2::rc::{Id, Shared};
use objc2::runtime::NSObject;

extern_class!(
    pub struct MyObject;

    unsafe impl ClassType for MyObject {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(a:)]
        fn variadic_class(arg: i32, arg2: ...);
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(a:)]
        fn variadic_instance(&self, arg: i32, ...);
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id(a:)]
        fn variadic_id(arg: i32, arg2: ...) -> Id<NSObject, Shared>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(a:_)]
        fn variadic_error(arg2: ...) -> Result<(), Id<NSObject, Shared>>;
    }
);

fn main() {}
