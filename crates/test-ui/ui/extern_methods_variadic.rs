use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
);

impl MyObject {
    extern_methods!(
        #[unsafe(method(a:))]
        fn variadic_class(arg: i32, arg2: ...);
    );

    extern_methods!(
        #[unsafe(method(a:))]
        fn variadic_instance(&self, arg: i32, ...);
    );

    extern_methods!(
        #[unsafe(method(a:_))]
        fn variadic_error(arg2: ...) -> Result<(), Retained<NSObject>>;
    );
}

fn main() {}
