//! Test extern_class! without `unsafe` in front of `super(...)`
use objc2::extern_class;
use objc2::runtime::NSObject;

extern_class!(
    #[super(NSObject)]
    struct NoSuper;
);

fn main() {}
