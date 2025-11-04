//! Test old syntax for ivars in define_class!
use objc2::define_class;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    #[ivars = i32]
    struct OldIvars;
);

fn main() {}
