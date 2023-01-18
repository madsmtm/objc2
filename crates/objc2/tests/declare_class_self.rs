//! To remind myself that `Self` needs to work in methods in `declare_class!`,
//! and hence we _must_ implement things by changing the generated method, we
//! can't just create an internal helper function (since we can't name the
//! types of such a function)!
use objc2::rc::{Allocated, Id, Shared};
use objc2::runtime::NSObject;
use objc2::{declare_class, ClassType};

trait GetSameType {
    type SameType: ?Sized;
}

impl<T: ?Sized> GetSameType for T {
    type SameType = T;
}

macro_rules! get_self {
    () => {
        Self
    };
}

declare_class!(
    struct MyTestObject;

    unsafe impl ClassType for MyTestObject {
        type Super = NSObject;
    }

    unsafe impl MyTestObject {
        #[method_id(init)]
        fn init(
            _this: Allocated<<Self as GetSameType>::SameType>,
            _param: <*const Self as GetSameType>::SameType,
        ) -> Id<<Self as GetSameType>::SameType, Shared> {
            unimplemented!()
        }

        #[method(compare:)]
        fn compare(&self, _other: &Self) -> bool {
            unimplemented!()
        }

        #[method_id(test4)]
        #[allow(unused_parens)]
        fn test4(_this: &<(Self) as GetSameType>::SameType) -> Id<get_self!(), Shared> {
            unimplemented!()
        }
    }
);

fn main() {}
