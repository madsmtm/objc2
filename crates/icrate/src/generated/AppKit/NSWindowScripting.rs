#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_methods!(
    #[doc = "NSScripting"]
    unsafe impl NSWindow {
        #[method(hasCloseBox)]
        pub unsafe fn hasCloseBox(&self) -> bool;
        #[method(hasTitleBar)]
        pub unsafe fn hasTitleBar(&self) -> bool;
        #[method(isFloatingPanel)]
        pub unsafe fn isFloatingPanel(&self) -> bool;
        #[method(isMiniaturizable)]
        pub unsafe fn isMiniaturizable(&self) -> bool;
        #[method(isModalPanel)]
        pub unsafe fn isModalPanel(&self) -> bool;
        #[method(isResizable)]
        pub unsafe fn isResizable(&self) -> bool;
        #[method(isZoomable)]
        pub unsafe fn isZoomable(&self) -> bool;
        #[method(orderedIndex)]
        pub unsafe fn orderedIndex(&self) -> NSInteger;
        #[method(setOrderedIndex:)]
        pub unsafe fn setOrderedIndex(&self, orderedIndex: NSInteger);
        #[method(setIsMiniaturized:)]
        pub unsafe fn setIsMiniaturized(&self, flag: bool);
        #[method(setIsVisible:)]
        pub unsafe fn setIsVisible(&self, flag: bool);
        #[method(setIsZoomed:)]
        pub unsafe fn setIsZoomed(&self, flag: bool);
        #[method_id(handleCloseScriptCommand:)]
        pub unsafe fn handleCloseScriptCommand(
            &self,
            command: &NSCloseCommand,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(handlePrintScriptCommand:)]
        pub unsafe fn handlePrintScriptCommand(
            &self,
            command: &NSScriptCommand,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(handleSaveScriptCommand:)]
        pub unsafe fn handleSaveScriptCommand(
            &self,
            command: &NSScriptCommand,
        ) -> Option<Id<Object, Shared>>;
    }
);
