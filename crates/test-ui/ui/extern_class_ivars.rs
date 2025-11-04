//! Test specifying ivars in extern_class!
use objc2::extern_class;
use objc2::runtime::NSObject;

extern_class!(
    #[unsafe(super(NSObject))]
    struct Ivars {
        ivar: i32,
    }
);

extern_class!(
    #[unsafe(super(NSObject))]
    #[ivars = i32]
    struct IvarsAttribute;
);

fn main() {}
