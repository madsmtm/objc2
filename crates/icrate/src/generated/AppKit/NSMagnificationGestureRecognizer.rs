use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSGestureRecognizer::*;
use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSMagnificationGestureRecognizer;
    unsafe impl ClassType for NSMagnificationGestureRecognizer {
        type Super = NSGestureRecognizer;
    }
);
extern_methods!(
    unsafe impl NSMagnificationGestureRecognizer {
        #[method(magnification)]
        pub unsafe fn magnification(&self) -> CGFloat;
        #[method(setMagnification:)]
        pub unsafe fn setMagnification(&self, magnification: CGFloat);
    }
);
