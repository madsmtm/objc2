//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

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
