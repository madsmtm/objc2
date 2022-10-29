#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPressGestureRecognizer;
    unsafe impl ClassType for NSPressGestureRecognizer {
        type Super = NSGestureRecognizer;
    }
);
extern_methods!(
    unsafe impl NSPressGestureRecognizer {
        #[method(buttonMask)]
        pub unsafe fn buttonMask(&self) -> NSUInteger;
        #[method(setButtonMask:)]
        pub unsafe fn setButtonMask(&self, buttonMask: NSUInteger);
        #[method(minimumPressDuration)]
        pub unsafe fn minimumPressDuration(&self) -> NSTimeInterval;
        #[method(setMinimumPressDuration:)]
        pub unsafe fn setMinimumPressDuration(&self, minimumPressDuration: NSTimeInterval);
        #[method(allowableMovement)]
        pub unsafe fn allowableMovement(&self) -> CGFloat;
        #[method(setAllowableMovement:)]
        pub unsafe fn setAllowableMovement(&self, allowableMovement: CGFloat);
        #[method(numberOfTouchesRequired)]
        pub unsafe fn numberOfTouchesRequired(&self) -> NSInteger;
        #[method(setNumberOfTouchesRequired:)]
        pub unsafe fn setNumberOfTouchesRequired(&self, numberOfTouchesRequired: NSInteger);
    }
);
