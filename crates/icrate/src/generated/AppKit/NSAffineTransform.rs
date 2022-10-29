#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_methods!(
    #[doc = "NSAppKitAdditions"]
    unsafe impl NSAffineTransform {
        #[method_id(transformBezierPath:)]
        pub unsafe fn transformBezierPath(&self, path: &NSBezierPath) -> Id<NSBezierPath, Shared>;
        #[method(set)]
        pub unsafe fn set(&self);
        #[method(concat)]
        pub unsafe fn concat(&self);
    }
);
