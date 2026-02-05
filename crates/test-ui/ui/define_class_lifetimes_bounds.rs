//! Ensure that you cannot force two input lifetimes to be the same.
use objc2::define_class;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    struct LifetimesBound;

    impl LifetimesBound {
        #[unsafe(method(lifetimesBound))]
        fn lifetimes_bound<'a>(_input1: &'a i32, _input2: &'a i32) {}
    }
);

fn main() {}
