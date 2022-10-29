#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTitlebarAccessoryViewController;
    unsafe impl ClassType for NSTitlebarAccessoryViewController {
        type Super = NSViewController;
    }
);
extern_methods!(
    unsafe impl NSTitlebarAccessoryViewController {
        #[method(layoutAttribute)]
        pub unsafe fn layoutAttribute(&self) -> NSLayoutAttribute;
        #[method(setLayoutAttribute:)]
        pub unsafe fn setLayoutAttribute(&self, layoutAttribute: NSLayoutAttribute);
        #[method(fullScreenMinHeight)]
        pub unsafe fn fullScreenMinHeight(&self) -> CGFloat;
        #[method(setFullScreenMinHeight:)]
        pub unsafe fn setFullScreenMinHeight(&self, fullScreenMinHeight: CGFloat);
        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;
        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);
        #[method(automaticallyAdjustsSize)]
        pub unsafe fn automaticallyAdjustsSize(&self) -> bool;
        #[method(setAutomaticallyAdjustsSize:)]
        pub unsafe fn setAutomaticallyAdjustsSize(&self, automaticallyAdjustsSize: bool);
        #[method(viewWillAppear)]
        pub unsafe fn viewWillAppear(&self);
        #[method(viewDidAppear)]
        pub unsafe fn viewDidAppear(&self);
        #[method(viewDidDisappear)]
        pub unsafe fn viewDidDisappear(&self);
    }
);
