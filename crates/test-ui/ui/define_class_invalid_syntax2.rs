use objc2::define_class;
use objc2::runtime::NSObject;

define_class!(
    #[unsafe(super(NSObject))]
    #[repr(transparent)]
    struct HasRepr;
);

fn main() {}
