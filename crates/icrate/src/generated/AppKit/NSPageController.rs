#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSPageControllerObjectIdentifier = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSPageController;
    unsafe impl ClassType for NSPageController {
        type Super = NSViewController;
    }
);
extern_methods!(
    unsafe impl NSPageController {
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSPageControllerDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSPageControllerDelegate>);
        #[method_id(selectedViewController)]
        pub unsafe fn selectedViewController(&self) -> Option<Id<NSViewController, Shared>>;
        #[method(transitionStyle)]
        pub unsafe fn transitionStyle(&self) -> NSPageControllerTransitionStyle;
        #[method(setTransitionStyle:)]
        pub unsafe fn setTransitionStyle(&self, transitionStyle: NSPageControllerTransitionStyle);
        #[method_id(arrangedObjects)]
        pub unsafe fn arrangedObjects(&self) -> Id<NSArray, Shared>;
        #[method(setArrangedObjects:)]
        pub unsafe fn setArrangedObjects(&self, arrangedObjects: &NSArray);
        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSInteger;
        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selectedIndex: NSInteger);
        #[method(navigateForwardToObject:)]
        pub unsafe fn navigateForwardToObject(&self, object: &Object);
        #[method(completeTransition)]
        pub unsafe fn completeTransition(&self);
        #[method(navigateBack:)]
        pub unsafe fn navigateBack(&self, sender: Option<&Object>);
        #[method(navigateForward:)]
        pub unsafe fn navigateForward(&self, sender: Option<&Object>);
        #[method(takeSelectedIndexFrom:)]
        pub unsafe fn takeSelectedIndexFrom(&self, sender: Option<&Object>);
    }
);
pub type NSPageControllerDelegate = NSObject;
