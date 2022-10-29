#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDrawer;
    unsafe impl ClassType for NSDrawer {
        type Super = NSResponder;
    }
);
extern_methods!(
    unsafe impl NSDrawer {
        #[method_id(initWithContentSize:preferredEdge:)]
        pub unsafe fn initWithContentSize_preferredEdge(
            &self,
            contentSize: NSSize,
            edge: NSRectEdge,
        ) -> Id<Self, Shared>;
        #[method_id(parentWindow)]
        pub unsafe fn parentWindow(&self) -> Option<Id<NSWindow, Shared>>;
        #[method(setParentWindow:)]
        pub unsafe fn setParentWindow(&self, parentWindow: Option<&NSWindow>);
        #[method_id(contentView)]
        pub unsafe fn contentView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, contentView: Option<&NSView>);
        #[method(preferredEdge)]
        pub unsafe fn preferredEdge(&self) -> NSRectEdge;
        #[method(setPreferredEdge:)]
        pub unsafe fn setPreferredEdge(&self, preferredEdge: NSRectEdge);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSDrawerDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSDrawerDelegate>);
        #[method(open)]
        pub unsafe fn open(&self);
        #[method(openOnEdge:)]
        pub unsafe fn openOnEdge(&self, edge: NSRectEdge);
        #[method(close)]
        pub unsafe fn close(&self);
        #[method(open:)]
        pub unsafe fn open(&self, sender: Option<&Object>);
        #[method(close:)]
        pub unsafe fn close(&self, sender: Option<&Object>);
        #[method(toggle:)]
        pub unsafe fn toggle(&self, sender: Option<&Object>);
        #[method(state)]
        pub unsafe fn state(&self) -> NSInteger;
        #[method(edge)]
        pub unsafe fn edge(&self) -> NSRectEdge;
        #[method(contentSize)]
        pub unsafe fn contentSize(&self) -> NSSize;
        #[method(setContentSize:)]
        pub unsafe fn setContentSize(&self, contentSize: NSSize);
        #[method(minContentSize)]
        pub unsafe fn minContentSize(&self) -> NSSize;
        #[method(setMinContentSize:)]
        pub unsafe fn setMinContentSize(&self, minContentSize: NSSize);
        #[method(maxContentSize)]
        pub unsafe fn maxContentSize(&self) -> NSSize;
        #[method(setMaxContentSize:)]
        pub unsafe fn setMaxContentSize(&self, maxContentSize: NSSize);
        #[method(leadingOffset)]
        pub unsafe fn leadingOffset(&self) -> CGFloat;
        #[method(setLeadingOffset:)]
        pub unsafe fn setLeadingOffset(&self, leadingOffset: CGFloat);
        #[method(trailingOffset)]
        pub unsafe fn trailingOffset(&self) -> CGFloat;
        #[method(setTrailingOffset:)]
        pub unsafe fn setTrailingOffset(&self, trailingOffset: CGFloat);
    }
);
extern_methods!(
    #[doc = "NSDrawers"]
    unsafe impl NSWindow {
        #[method_id(drawers)]
        pub unsafe fn drawers(&self) -> Option<Id<NSArray<NSDrawer>, Shared>>;
    }
);
pub type NSDrawerDelegate = NSObject;