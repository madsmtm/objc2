//! Test that `&Retained<Self>` is not a valid receiver by construction.
use objc2::define_class;
use objc2::rc::Retained;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    struct CustomObject;

    impl CustomObject {
        #[unsafe(method(invalid))]
        fn invalid(_this: &Retained<Self>) {}
    }
);

fn main() {}
