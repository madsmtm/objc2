#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPrintOperation;
    unsafe impl ClassType for NSPrintOperation {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPrintOperation {
        #[method_id(printOperationWithView:printInfo:)]
        pub unsafe fn printOperationWithView_printInfo(
            view: &NSView,
            printInfo: &NSPrintInfo,
        ) -> Id<NSPrintOperation, Shared>;
        #[method_id(PDFOperationWithView:insideRect:toData:printInfo:)]
        pub unsafe fn PDFOperationWithView_insideRect_toData_printInfo(
            view: &NSView,
            rect: NSRect,
            data: &NSMutableData,
            printInfo: &NSPrintInfo,
        ) -> Id<NSPrintOperation, Shared>;
        #[method_id(PDFOperationWithView:insideRect:toPath:printInfo:)]
        pub unsafe fn PDFOperationWithView_insideRect_toPath_printInfo(
            view: &NSView,
            rect: NSRect,
            path: &NSString,
            printInfo: &NSPrintInfo,
        ) -> Id<NSPrintOperation, Shared>;
        #[method_id(EPSOperationWithView:insideRect:toData:printInfo:)]
        pub unsafe fn EPSOperationWithView_insideRect_toData_printInfo(
            view: &NSView,
            rect: NSRect,
            data: &NSMutableData,
            printInfo: &NSPrintInfo,
        ) -> Id<NSPrintOperation, Shared>;
        #[method_id(EPSOperationWithView:insideRect:toPath:printInfo:)]
        pub unsafe fn EPSOperationWithView_insideRect_toPath_printInfo(
            view: &NSView,
            rect: NSRect,
            path: &NSString,
            printInfo: &NSPrintInfo,
        ) -> Id<NSPrintOperation, Shared>;
        #[method_id(printOperationWithView:)]
        pub unsafe fn printOperationWithView(view: &NSView) -> Id<NSPrintOperation, Shared>;
        #[method_id(PDFOperationWithView:insideRect:toData:)]
        pub unsafe fn PDFOperationWithView_insideRect_toData(
            view: &NSView,
            rect: NSRect,
            data: &NSMutableData,
        ) -> Id<NSPrintOperation, Shared>;
        #[method_id(EPSOperationWithView:insideRect:toData:)]
        pub unsafe fn EPSOperationWithView_insideRect_toData(
            view: &NSView,
            rect: NSRect,
            data: Option<&NSMutableData>,
        ) -> Id<NSPrintOperation, Shared>;
        #[method_id(currentOperation)]
        pub unsafe fn currentOperation() -> Option<Id<NSPrintOperation, Shared>>;
        #[method(setCurrentOperation:)]
        pub unsafe fn setCurrentOperation(currentOperation: Option<&NSPrintOperation>);
        #[method(isCopyingOperation)]
        pub unsafe fn isCopyingOperation(&self) -> bool;
        #[method(preferredRenderingQuality)]
        pub unsafe fn preferredRenderingQuality(&self) -> NSPrintRenderingQuality;
        #[method_id(jobTitle)]
        pub unsafe fn jobTitle(&self) -> Option<Id<NSString, Shared>>;
        #[method(setJobTitle:)]
        pub unsafe fn setJobTitle(&self, jobTitle: Option<&NSString>);
        #[method(showsPrintPanel)]
        pub unsafe fn showsPrintPanel(&self) -> bool;
        #[method(setShowsPrintPanel:)]
        pub unsafe fn setShowsPrintPanel(&self, showsPrintPanel: bool);
        #[method(showsProgressPanel)]
        pub unsafe fn showsProgressPanel(&self) -> bool;
        #[method(setShowsProgressPanel:)]
        pub unsafe fn setShowsProgressPanel(&self, showsProgressPanel: bool);
        #[method_id(printPanel)]
        pub unsafe fn printPanel(&self) -> Id<NSPrintPanel, Shared>;
        #[method(setPrintPanel:)]
        pub unsafe fn setPrintPanel(&self, printPanel: &NSPrintPanel);
        #[method_id(PDFPanel)]
        pub unsafe fn PDFPanel(&self) -> Id<NSPDFPanel, Shared>;
        #[method(setPDFPanel:)]
        pub unsafe fn setPDFPanel(&self, PDFPanel: &NSPDFPanel);
        #[method(canSpawnSeparateThread)]
        pub unsafe fn canSpawnSeparateThread(&self) -> bool;
        #[method(setCanSpawnSeparateThread:)]
        pub unsafe fn setCanSpawnSeparateThread(&self, canSpawnSeparateThread: bool);
        #[method(pageOrder)]
        pub unsafe fn pageOrder(&self) -> NSPrintingPageOrder;
        #[method(setPageOrder:)]
        pub unsafe fn setPageOrder(&self, pageOrder: NSPrintingPageOrder);
        #[method(runOperationModalForWindow:delegate:didRunSelector:contextInfo:)]
        pub unsafe fn runOperationModalForWindow_delegate_didRunSelector_contextInfo(
            &self,
            docWindow: &NSWindow,
            delegate: Option<&Object>,
            didRunSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );
        #[method(runOperation)]
        pub unsafe fn runOperation(&self) -> bool;
        #[method_id(view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;
        #[method_id(printInfo)]
        pub unsafe fn printInfo(&self) -> Id<NSPrintInfo, Shared>;
        #[method(setPrintInfo:)]
        pub unsafe fn setPrintInfo(&self, printInfo: &NSPrintInfo);
        #[method_id(context)]
        pub unsafe fn context(&self) -> Option<Id<NSGraphicsContext, Shared>>;
        #[method(pageRange)]
        pub unsafe fn pageRange(&self) -> NSRange;
        #[method(currentPage)]
        pub unsafe fn currentPage(&self) -> NSInteger;
        #[method_id(createContext)]
        pub unsafe fn createContext(&self) -> Option<Id<NSGraphicsContext, Shared>>;
        #[method(destroyContext)]
        pub unsafe fn destroyContext(&self);
        #[method(deliverResult)]
        pub unsafe fn deliverResult(&self) -> bool;
        #[method(cleanUpOperation)]
        pub unsafe fn cleanUpOperation(&self);
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSPrintOperation {
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, view: Option<&NSView>);
        #[method_id(accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setJobStyleHint:)]
        pub unsafe fn setJobStyleHint(&self, hint: Option<&NSString>);
        #[method_id(jobStyleHint)]
        pub unsafe fn jobStyleHint(&self) -> Option<Id<NSString, Shared>>;
        #[method(setShowPanels:)]
        pub unsafe fn setShowPanels(&self, flag: bool);
        #[method(showPanels)]
        pub unsafe fn showPanels(&self) -> bool;
    }
);