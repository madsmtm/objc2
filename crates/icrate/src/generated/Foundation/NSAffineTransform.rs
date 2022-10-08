use crate::CoreGraphics::generated::CGAffineTransform::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAffineTransform;
    unsafe impl ClassType for NSAffineTransform {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAffineTransform {
        pub unsafe fn transform() -> Id<NSAffineTransform, Shared> {
            msg_send_id![Self::class(), transform]
        }
        pub unsafe fn initWithTransform(&self, transform: &NSAffineTransform) -> Id<Self, Shared> {
            msg_send_id![self, initWithTransform: transform]
        }
        pub unsafe fn init(&self) -> Id<Self, Shared> {
            msg_send_id![self, init]
        }
        pub unsafe fn translateXBy_yBy(&self, deltaX: CGFloat, deltaY: CGFloat) {
            msg_send![self, translateXBy: deltaX, yBy: deltaY]
        }
        pub unsafe fn rotateByDegrees(&self, angle: CGFloat) {
            msg_send![self, rotateByDegrees: angle]
        }
        pub unsafe fn rotateByRadians(&self, angle: CGFloat) {
            msg_send![self, rotateByRadians: angle]
        }
        pub unsafe fn scaleBy(&self, scale: CGFloat) {
            msg_send![self, scaleBy: scale]
        }
        pub unsafe fn scaleXBy_yBy(&self, scaleX: CGFloat, scaleY: CGFloat) {
            msg_send![self, scaleXBy: scaleX, yBy: scaleY]
        }
        pub unsafe fn invert(&self) {
            msg_send![self, invert]
        }
        pub unsafe fn appendTransform(&self, transform: &NSAffineTransform) {
            msg_send![self, appendTransform: transform]
        }
        pub unsafe fn prependTransform(&self, transform: &NSAffineTransform) {
            msg_send![self, prependTransform: transform]
        }
        pub unsafe fn transformPoint(&self, aPoint: NSPoint) -> NSPoint {
            msg_send![self, transformPoint: aPoint]
        }
        pub unsafe fn transformSize(&self, aSize: NSSize) -> NSSize {
            msg_send![self, transformSize: aSize]
        }
        pub unsafe fn transformStruct(&self) -> NSAffineTransformStruct {
            msg_send![self, transformStruct]
        }
        pub unsafe fn setTransformStruct(&self, transformStruct: NSAffineTransformStruct) {
            msg_send![self, setTransformStruct: transformStruct]
        }
    }
);
