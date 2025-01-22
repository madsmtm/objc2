use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyTest;
);

extern_methods!(
    #[cfg(not(test))]
    unsafe impl MyTest {
        #[unsafe(method(enabled))]
        fn enabled();
    }

    #[cfg(test)]
    unsafe impl MyTest {
        #[unsafe(method(disabled))]
        fn disabled();
    }

    unsafe impl MyTest {
        #[unsafe(method(enabled))]
        #[cfg(not(test))]
        fn enabled_inner1();

        #[cfg(not(test))]
        #[unsafe(method(enabled))]
        fn enabled_inner2();

        #[unsafe(method(disabled))]
        #[cfg(test)]
        fn disabled_inner1();

        #[cfg(test)]
        #[unsafe(method(disabled))]
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
