use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAutoreleasePool;
    unsafe impl ClassType for NSAutoreleasePool {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAutoreleasePool {
        # [method (addObject :)]
        pub unsafe fn addObject(anObject: &Object);
        # [method (addObject :)]
        pub unsafe fn addObject(&self, anObject: &Object);
        #[method(drain)]
        pub unsafe fn drain(&self);
    }
);
