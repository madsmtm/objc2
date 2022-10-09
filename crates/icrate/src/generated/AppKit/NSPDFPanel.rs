use super::__exported::NSMutableArray;
use super::__exported::NSPDFInfo;
use super::__exported::NSString;
use super::__exported::NSViewController;
use super::__exported::NSWindow;
use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPDFPanel;
    unsafe impl ClassType for NSPDFPanel {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPDFPanel {
        #[method_id(panel)]
        pub unsafe fn panel() -> Id<NSPDFPanel, Shared>;
        #[method_id(accessoryController)]
        pub unsafe fn accessoryController(&self) -> Option<Id<NSViewController, Shared>>;
        #[method(setAccessoryController:)]
        pub unsafe fn setAccessoryController(&self, accessoryController: Option<&NSViewController>);
        #[method(options)]
        pub unsafe fn options(&self) -> NSPDFPanelOptions;
        #[method(setOptions:)]
        pub unsafe fn setOptions(&self, options: NSPDFPanelOptions);
        #[method_id(defaultFileName)]
        pub unsafe fn defaultFileName(&self) -> Id<NSString, Shared>;
        #[method(setDefaultFileName:)]
        pub unsafe fn setDefaultFileName(&self, defaultFileName: &NSString);
        #[method(beginSheetWithPDFInfo:modalForWindow:completionHandler:)]
        pub unsafe fn beginSheetWithPDFInfo_modalForWindow_completionHandler(
            &self,
            pdfInfo: &NSPDFInfo,
            docWindow: Option<&NSWindow>,
            completionHandler: TodoBlock,
        );
    }
);
