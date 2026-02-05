use objc2::define_class;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    struct ReturnSelf;

    impl ReturnSelf {
        #[unsafe(method(returnSelf))]
        fn return_self(&self) -> &Self {
            self
        }
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct ReturnIvar {
        ivar: i32,
    }

    impl ReturnIvar {
        #[unsafe(method(returnIvar))]
        fn return_ivar(&self) -> &i32 {
            self.ivar()
        }
    }
);

define_class!(
    #[unsafe(super(NSObject))]
    struct ReturnOther;

    impl ReturnOther {
        #[unsafe(method(returnOther:))]
        fn return_other(&self, other: &i32) -> &i32 {
            other
        }
    }
);

fn main() {}
