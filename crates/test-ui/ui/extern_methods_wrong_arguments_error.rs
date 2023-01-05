use objc2::{extern_class, extern_methods, ClassType};
use objc2::runtime::NSObject;
use objc2::rc::{Id, Shared};

extern_class!(
    pub struct MyObject;

    unsafe impl ClassType for MyObject {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(too:few:_)]
        fn class_too_few() -> Result<(), Id<NSObject, Shared>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(tooMany:_)]
        fn class_too_many(arg: i32) -> Result<(), Id<NSObject, Shared>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(too:few:_)]
        fn too_few(&self) -> Result<(), Id<NSObject, Shared>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(tooMany:_)]
        fn too_many(&self, arg: i32) -> Result<(), Id<NSObject, Shared>>;
    }
);

fn main() {}
