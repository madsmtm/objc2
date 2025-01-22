use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
);

impl MyObject {
    extern_methods!(
        #[unsafe(method(a:))]
        fn a();
    );

    extern_methods!(
        #[unsafe(method(b))]
        fn b(arg: i32);
    );

    extern_methods!(
        #[unsafe(method(c:d:e:))]
        fn c(arg1: i32, arg2: u32);
    );

    extern_methods!(
        #[unsafe(method(f:g:))]
        fn f();
    );

    extern_methods!(
        #[unsafe(method(x:))]
        fn x(&self);
    );

    extern_methods!(
        #[unsafe(method(y))]
        fn y(&self, arg: i32);
    );

    extern_methods!(
        #[unsafe(method(z:))]
        fn z(this: &Self);
    );

    extern_methods!(
        #[unsafe(method(w))]
        fn w(this: &Self, arg: i32);
    );
}

fn main() {}
