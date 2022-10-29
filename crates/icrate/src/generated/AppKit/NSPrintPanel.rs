#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSPrintPanelJobStyleHint = NSString;
pub type NSPrintPanelAccessorySummaryKey = NSString;
pub type NSPrintPanelAccessorizing = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSPrintPanel;
    unsafe impl ClassType for NSPrintPanel {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPrintPanel {
        #[method_id(printPanel)]
        pub unsafe fn printPanel() -> Id<NSPrintPanel, Shared>;
        #[method(addAccessoryController:)]
        pub unsafe fn addAccessoryController(&self, accessoryController: &TodoProtocols);
        #[method(removeAccessoryController:)]
        pub unsafe fn removeAccessoryController(&self, accessoryController: &TodoProtocols);
        #[method_id(accessoryControllers)]
        pub unsafe fn accessoryControllers(&self) -> Id<NSArray<NSViewController>, Shared>;
        #[method(options)]
        pub unsafe fn options(&self) -> NSPrintPanelOptions;
        #[method(setOptions:)]
        pub unsafe fn setOptions(&self, options: NSPrintPanelOptions);
        #[method(setDefaultButtonTitle:)]
        pub unsafe fn setDefaultButtonTitle(&self, defaultButtonTitle: Option<&NSString>);
        #[method_id(defaultButtonTitle)]
        pub unsafe fn defaultButtonTitle(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(helpAnchor)]
        pub unsafe fn helpAnchor(&self) -> Option<Id<NSHelpAnchorName, Shared>>;
        #[method(setHelpAnchor:)]
        pub unsafe fn setHelpAnchor(&self, helpAnchor: Option<&NSHelpAnchorName>);
        #[method_id(jobStyleHint)]
        pub unsafe fn jobStyleHint(&self) -> Option<Id<NSPrintPanelJobStyleHint, Shared>>;
        #[method(setJobStyleHint:)]
        pub unsafe fn setJobStyleHint(&self, jobStyleHint: Option<&NSPrintPanelJobStyleHint>);
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
        pub unsafe fn printInfo(&self) -> Id<NSPrintInfo, Shared>;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSPrintPanel {
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessoryView: Option<&NSView>);
        #[method_id(accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView, Shared>>;
        #[method(updateFromPrintInfo)]
        pub unsafe fn updateFromPrintInfo(&self);
        #[method(finalWritePrintInfo)]
        pub unsafe fn finalWritePrintInfo(&self);
    }
);