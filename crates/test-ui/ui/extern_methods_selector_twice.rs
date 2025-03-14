use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
);

impl MyObject {
    extern_methods!(
        #[unsafe(method(foo))]
        #[unsafe(method(bar))]
        fn selector_twice();
    );
}

fn main() {}
