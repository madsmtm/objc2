use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSGestureRecognizer::*;
use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSRotationGestureRecognizer;
    unsafe impl ClassType for NSRotationGestureRecognizer {
        type Super = NSGestureRecognizer;
    }
);
extern_methods!(
    unsafe impl NSRotationGestureRecognizer {
        #[method(rotation)]
        pub unsafe fn rotation(&self) -> CGFloat;
        #[method(setRotation:)]
        pub unsafe fn setRotation(&self, rotation: CGFloat);
        #[method(rotationInDegrees)]
        pub unsafe fn rotationInDegrees(&self) -> CGFloat;
        #[method(setRotationInDegrees:)]
        pub unsafe fn setRotationInDegrees(&self, rotationInDegrees: CGFloat);
    }
);
