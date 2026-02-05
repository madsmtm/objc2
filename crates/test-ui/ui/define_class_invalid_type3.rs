use objc2::define_class;
use objc2::rc::Retained;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    struct BadSelectors;

    impl BadSelectors {
        #[unsafe(method(retain))]
        fn retain() -> Retained<Self> {
            unimplemented!()
        }

        #[unsafe(method(release))]
        fn release() -> Retained<Self> {
            unimplemented!()
        }

        #[unsafe(method(autorelease))]
        fn autorelease() -> Retained<Self> {
            unimplemented!()
        }
    }
);

fn main() {}
