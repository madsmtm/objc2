use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyTest;
);

impl MyTest {
    extern_methods!(
        #[unsafe(method(enabled))]
        #[cfg(not(test))]
        fn enabled1();

        #[cfg(not(test))]
        #[unsafe(method(enabled))]
        fn enabled2();

        #[unsafe(method(disabled))]
        #[cfg(test)]
        fn disabled1();

        #[cfg(test)]
        #[unsafe(method(disabled))]
        fn disabled2();
    );
}

fn main() {
    MyTest::enabled1();
    MyTest::enabled2();
    MyTest::disabled1();
    MyTest::disabled2();
}
