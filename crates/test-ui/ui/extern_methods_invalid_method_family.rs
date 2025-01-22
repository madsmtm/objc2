//! Test various usage of the method_family attribute.
use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
);

impl MyObject {
    extern_methods!(
        #[unsafe(method(noUnsafe))]
        #[method_family = none]
        fn no_unsafe(&self) -> Retained<Self>;
    );

    extern_methods!(
        #[unsafe(method(unknownFamily))]
        #[unsafe(method_family = unknown)]
        fn unknown_family(&self) -> Retained<Self>;
    );

    extern_methods!(
        #[unsafe(method(familyTwice))]
        #[unsafe(method_family = copy)]
        #[unsafe(method_family = none)]
        fn family_twice(&self) -> Retained<Self>;
    );
}

fn main() {}
