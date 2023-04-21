use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods, mutability, ClassType};

extern_class!(
    pub struct MyTest;

    unsafe impl ClassType for MyTest {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

extern_methods!(
    #[cfg(not(feature = "mytest"))]
    unsafe impl MyTest {
        #[method(enabled)]
        fn enabled();
    }

    #[cfg(feature = "mytest")]
    unsafe impl MyTest {
        #[method(disabled)]
        fn disabled();
    }

    unsafe impl MyTest {
        #[method(enabled)]
        #[cfg(not(feature = "mytest"))]
        fn enabled_inner1();

        #[cfg(not(feature = "mytest"))]
        #[method(enabled)]
        fn enabled_inner2();

        #[method(disabled)]
        #[cfg(feature = "mytest")]
        fn disabled_inner1();

        #[cfg(feature = "mytest")]
        #[method(disabled)]
        fn disabled_inner2();
    }
);

fn main() {
    MyTest::enabled();
    MyTest::disabled();

    MyTest::enabled_inner1();
    MyTest::enabled_inner2();
    MyTest::disabled_inner1();
    MyTest::disabled_inner2();
}
