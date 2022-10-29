#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPopover;
    unsafe impl ClassType for NSPopover {
        type Super = NSResponder;
    }
);
extern_methods!(
    unsafe impl NSPopover {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSPopoverDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSPopoverDelegate>);
        #[method(appearance)]
        pub unsafe fn appearance(&self) -> NSPopoverAppearance;
        #[method(setAppearance:)]
        pub unsafe fn setAppearance(&self, appearance: NSPopoverAppearance);
        #[method(behavior)]
        pub unsafe fn behavior(&self) -> NSPopoverBehavior;
        #[method(setBehavior:)]
        pub unsafe fn setBehavior(&self, behavior: NSPopoverBehavior);
        #[method(animates)]
        pub unsafe fn animates(&self) -> bool;
        #[method(setAnimates:)]
        pub unsafe fn setAnimates(&self, animates: bool);
        #[method_id(contentViewController)]
        pub unsafe fn contentViewController(&self) -> Option<Id<NSViewController, Shared>>;
        #[method(setContentViewController:)]
        pub unsafe fn setContentViewController(
            &self,
            contentViewController: Option<&NSViewController>,
        );
        #[method(contentSize)]
        pub unsafe fn contentSize(&self) -> NSSize;
        #[method(setContentSize:)]
        pub unsafe fn setContentSize(&self, contentSize: NSSize);
        #[method(isShown)]
        pub unsafe fn isShown(&self) -> bool;
        #[method(isDetached)]
        pub unsafe fn isDetached(&self) -> bool;
        #[method(positioningRect)]
        pub unsafe fn positioningRect(&self) -> NSRect;
        #[method(setPositioningRect:)]
        pub unsafe fn setPositioningRect(&self, positioningRect: NSRect);
        #[method(showRelativeToRect:ofView:preferredEdge:)]
        pub unsafe fn showRelativeToRect_ofView_preferredEdge(
            &self,
            positioningRect: NSRect,
            positioningView: &NSView,
            preferredEdge: NSRectEdge,
        );
        #[method(performClose:)]
        pub unsafe fn performClose(&self, sender: Option<&Object>);
        #[method(close)]
        pub unsafe fn close(&self);
    }
);
pub type NSPopoverCloseReasonValue = NSString;
pub type NSPopoverDelegate = NSObject;
