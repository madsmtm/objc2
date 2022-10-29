#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSNull;
    unsafe impl ClassType for NSNull {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSNull {
        #[method_id(null)]
        pub unsafe fn null() -> Id<NSNull, Shared>;
    }
);
