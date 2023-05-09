use objc2::rc::Id;
use objc2::runtime::NSObject;
use objc2::{declare_class, extern_class, extern_methods, mutability, ClassType};

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
        fn b(&self) -> Id<Self>;
    }
);

declare_class!(
    struct CustomObject1;

    unsafe impl ClassType for CustomObject1 {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "CustomObject1";
    }

    unsafe impl CustomObject1 {
        #[method(c)]
        #[optional]
        /// Doc comment
        fn c(&self) {}
    }
);

declare_class!(
    struct CustomObject2;

    unsafe impl ClassType for CustomObject2 {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "CustomObject2";
    }

    unsafe impl CustomObject2 {
        #[optional]
        /// Doc comment
        #[method_id(d)]
        fn d(&self) -> Id<Self> {
            unimplemented!()
        }
    }
);

fn main() {}
