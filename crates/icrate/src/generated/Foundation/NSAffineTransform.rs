use crate::CoreGraphics::generated::CGAffineTransform::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAffineTransform;
    unsafe impl ClassType for NSAffineTransform {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAffineTransform {
        #[method_id(transform)]
        pub unsafe fn transform() -> Id<NSAffineTransform, Shared>;
        #[method_id(initWithTransform:)]
        pub unsafe fn initWithTransform(&self, transform: &NSAffineTransform) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method(translateXBy:yBy:)]
        pub unsafe fn translateXBy_yBy(&self, deltaX: CGFloat, deltaY: CGFloat);
        #[method(rotateByDegrees:)]
        pub unsafe fn rotateByDegrees(&self, angle: CGFloat);
        #[method(rotateByRadians:)]
        pub unsafe fn rotateByRadians(&self, angle: CGFloat);
        #[method(scaleBy:)]
        pub unsafe fn scaleBy(&self, scale: CGFloat);
        #[method(scaleXBy:yBy:)]
        pub unsafe fn scaleXBy_yBy(&self, scaleX: CGFloat, scaleY: CGFloat);
        #[method(invert)]
        pub unsafe fn invert(&self);
        #[method(appendTransform:)]
        pub unsafe fn appendTransform(&self, transform: &NSAffineTransform);
        #[method(prependTransform:)]
        pub unsafe fn prependTransform(&self, transform: &NSAffineTransform);
        #[method(transformPoint:)]
        pub unsafe fn transformPoint(&self, aPoint: NSPoint) -> NSPoint;
        #[method(transformSize:)]
        pub unsafe fn transformSize(&self, aSize: NSSize) -> NSSize;
        #[method(transformStruct)]
        pub unsafe fn transformStruct(&self) -> NSAffineTransformStruct;
        #[method(setTransformStruct:)]
        pub unsafe fn setTransformStruct(&self, transformStruct: NSAffineTransformStruct);
    }
);
