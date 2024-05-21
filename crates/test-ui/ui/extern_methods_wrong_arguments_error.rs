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
        #[method(too:few:_)]
        fn class_too_few() -> Result<(), Retained<NSObject>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(tooMany:_)]
        fn class_too_many(arg: i32) -> Result<(), Retained<NSObject>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(too:few:_)]
        fn too_few(&self) -> Result<(), Retained<NSObject>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(tooMany:_)]
        fn too_many(&self, arg: i32) -> Result<(), Retained<NSObject>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(tooFew:withMtm:_)]
        fn too_few_with_mtm(&self, mtm: MainThreadMarker) -> Result<(), Retained<NSObject>>;
    }
);

fn main() {}
