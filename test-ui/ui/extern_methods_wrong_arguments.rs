use objc2::{extern_class, extern_methods, ClassType};
use objc2::foundation::NSObject;

extern_class!(
    pub struct MyObject;

    unsafe impl ClassType for MyObject {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[sel(a:)]
        fn a();
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[sel(b)]
        fn b(arg: i32);
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[sel(c:d:e:)]
        fn c(arg1: i32, arg2: u32);
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[sel(x:)]
        fn x(&self);
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[sel(y)]
        fn y(&self, arg: i32);
    }
);

fn main() {}
