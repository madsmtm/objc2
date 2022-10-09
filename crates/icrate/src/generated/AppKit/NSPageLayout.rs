use super::__exported::NSPrintInfo;
use super::__exported::NSView;
use super::__exported::NSViewController;
use super::__exported::NSWindow;
use super::__exported::NSWindowController;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSApplication::*;
use crate::Foundation::generated::NSArray::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPageLayout;
    unsafe impl ClassType for NSPageLayout {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPageLayout {
        #[method_id(pageLayout)]
        pub unsafe fn pageLayout() -> Id<NSPageLayout, Shared>;
        #[method(addAccessoryController:)]
        pub unsafe fn addAccessoryController(&self, accessoryController: &NSViewController);
        #[method(removeAccessoryController:)]
        pub unsafe fn removeAccessoryController(&self, accessoryController: &NSViewController);
        #[method_id(accessoryControllers)]
        pub unsafe fn accessoryControllers(&self) -> Id<NSArray<NSViewController>, Shared>;
        #[method(beginSheetWithPrintInfo:modalForWindow:delegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetWithPrintInfo_modalForWindow_delegate_didEndSelector_contextInfo(
            &self,
            printInfo: &NSPrintInfo,
            docWindow: &NSWindow,
            delegate: Option<&Object>,
            didEndSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );
        #[method(runModalWithPrintInfo:)]
        pub unsafe fn runModalWithPrintInfo(&self, printInfo: &NSPrintInfo) -> NSInteger;
        #[method(runModal)]
        pub unsafe fn runModal(&self) -> NSInteger;
        #[method_id(printInfo)]
        pub unsafe fn printInfo(&self) -> Option<Id<NSPrintInfo, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSPageLayout {
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessoryView: Option<&NSView>);
        #[method_id(accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView, Shared>>;
        #[method(readPrintInfo)]
        pub unsafe fn readPrintInfo(&self);
        #[method(writePrintInfo)]
        pub unsafe fn writePrintInfo(&self);
    }
);
extern_methods!(
    #[doc = "NSPageLayoutPanel"]
    unsafe impl NSApplication {
        #[method(runPageLayout:)]
        pub unsafe fn runPageLayout(&self, sender: Option<&Object>);
    }
);
