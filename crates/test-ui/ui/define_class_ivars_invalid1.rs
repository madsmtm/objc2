//! Test invalid syntax for ivars in define_class!
use objc2::define_class;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    struct TupleStruct(i32);
);

fn main() {}
