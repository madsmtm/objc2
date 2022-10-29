#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSClickGestureRecognizer;
    unsafe impl ClassType for NSClickGestureRecognizer {
        type Super = NSGestureRecognizer;
    }
);
extern_methods!(
    unsafe impl NSClickGestureRecognizer {
        #[method(buttonMask)]
        pub unsafe fn buttonMask(&self) -> NSUInteger;
        #[method(setButtonMask:)]
        pub unsafe fn setButtonMask(&self, buttonMask: NSUInteger);
        #[method(numberOfClicksRequired)]
        pub unsafe fn numberOfClicksRequired(&self) -> NSInteger;
        #[method(setNumberOfClicksRequired:)]
        pub unsafe fn setNumberOfClicksRequired(&self, numberOfClicksRequired: NSInteger);
        #[method(numberOfTouchesRequired)]
        pub unsafe fn numberOfTouchesRequired(&self) -> NSInteger;
        #[method(setNumberOfTouchesRequired:)]
        pub unsafe fn setNumberOfTouchesRequired(&self, numberOfTouchesRequired: NSInteger);
    }
);
