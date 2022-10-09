use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSGestureRecognizer::*;
use crate::Foundation::generated::Foundation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPanGestureRecognizer;
    unsafe impl ClassType for NSPanGestureRecognizer {
        type Super = NSGestureRecognizer;
    }
);
extern_methods!(
    unsafe impl NSPanGestureRecognizer {
        #[method(buttonMask)]
        pub unsafe fn buttonMask(&self) -> NSUInteger;
        #[method(setButtonMask:)]
        pub unsafe fn setButtonMask(&self, buttonMask: NSUInteger);
        #[method(translationInView:)]
        pub unsafe fn translationInView(&self, view: Option<&NSView>) -> NSPoint;
        #[method(setTranslation:inView:)]
        pub unsafe fn setTranslation_inView(&self, translation: NSPoint, view: Option<&NSView>);
        #[method(velocityInView:)]
        pub unsafe fn velocityInView(&self, view: Option<&NSView>) -> NSPoint;
        #[method(numberOfTouchesRequired)]
        pub unsafe fn numberOfTouchesRequired(&self) -> NSInteger;
        #[method(setNumberOfTouchesRequired:)]
        pub unsafe fn setNumberOfTouchesRequired(&self, numberOfTouchesRequired: NSInteger);
    }
);
