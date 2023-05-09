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
        #[method(too:few:_)]
        fn class_too_few() -> Result<(), Id<NSObject>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(tooMany:_)]
        fn class_too_many(arg: i32) -> Result<(), Id<NSObject>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(too:few:_)]
        fn too_few(&self) -> Result<(), Id<NSObject>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(tooMany:_)]
        fn too_many(&self, arg: i32) -> Result<(), Id<NSObject>>;
    }
);

fn main() {}
