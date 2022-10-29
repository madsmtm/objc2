#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSWindowController;
    unsafe impl ClassType for NSWindowController {
        type Super = NSResponder;
    }
);
extern_methods!(
    unsafe impl NSWindowController {
        #[method_id(initWithWindow:)]
        pub unsafe fn initWithWindow(&self, window: Option<&NSWindow>) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(initWithWindowNibName:)]
        pub unsafe fn initWithWindowNibName(&self, windowNibName: &NSNibName) -> Id<Self, Shared>;
        #[method_id(initWithWindowNibName:owner:)]
        pub unsafe fn initWithWindowNibName_owner(
            &self,
            windowNibName: &NSNibName,
            owner: &Object,
        ) -> Id<Self, Shared>;
        #[method_id(initWithWindowNibPath:owner:)]
        pub unsafe fn initWithWindowNibPath_owner(
            &self,
            windowNibPath: &NSString,
            owner: &Object,
        ) -> Id<Self, Shared>;
        #[method_id(windowNibName)]
        pub unsafe fn windowNibName(&self) -> Option<Id<NSNibName, Shared>>;
        #[method_id(windowNibPath)]
        pub unsafe fn windowNibPath(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(owner)]
        pub unsafe fn owner(&self) -> Option<Id<Object, Shared>>;
        #[method_id(windowFrameAutosaveName)]
        pub unsafe fn windowFrameAutosaveName(&self) -> Id<NSWindowFrameAutosaveName, Shared>;
        #[method(setWindowFrameAutosaveName:)]
        pub unsafe fn setWindowFrameAutosaveName(
            &self,
            windowFrameAutosaveName: &NSWindowFrameAutosaveName,
        );
        #[method(shouldCascadeWindows)]
        pub unsafe fn shouldCascadeWindows(&self) -> bool;
        #[method(setShouldCascadeWindows:)]
        pub unsafe fn setShouldCascadeWindows(&self, shouldCascadeWindows: bool);
        #[method_id(document)]
        pub unsafe fn document(&self) -> Option<Id<Object, Shared>>;
        #[method(setDocument:)]
        pub unsafe fn setDocument(&self, document: Option<&Object>);
        #[method(setDocumentEdited:)]
        pub unsafe fn setDocumentEdited(&self, dirtyFlag: bool);
        #[method(shouldCloseDocument)]
        pub unsafe fn shouldCloseDocument(&self) -> bool;
        #[method(setShouldCloseDocument:)]
        pub unsafe fn setShouldCloseDocument(&self, shouldCloseDocument: bool);
        #[method(synchronizeWindowTitleWithDocumentName)]
        pub unsafe fn synchronizeWindowTitleWithDocumentName(&self);
        #[method_id(windowTitleForDocumentDisplayName:)]
        pub unsafe fn windowTitleForDocumentDisplayName(
            &self,
            displayName: &NSString,
        ) -> Id<NSString, Shared>;
        #[method_id(contentViewController)]
        pub unsafe fn contentViewController(&self) -> Option<Id<NSViewController, Shared>>;
        #[method(setContentViewController:)]
        pub unsafe fn setContentViewController(
            &self,
            contentViewController: Option<&NSViewController>,
        );
        #[method_id(window)]
        pub unsafe fn window(&self) -> Option<Id<NSWindow, Shared>>;
        #[method(setWindow:)]
        pub unsafe fn setWindow(&self, window: Option<&NSWindow>);
        #[method(isWindowLoaded)]
        pub unsafe fn isWindowLoaded(&self) -> bool;
        #[method(windowWillLoad)]
        pub unsafe fn windowWillLoad(&self);
        #[method(windowDidLoad)]
        pub unsafe fn windowDidLoad(&self);
        #[method(loadWindow)]
        pub unsafe fn loadWindow(&self);
        #[method(close)]
        pub unsafe fn close(&self);
        #[method(showWindow:)]
        pub unsafe fn showWindow(&self, sender: Option<&Object>);
    }
);
extern_methods!(
    #[doc = "NSWindowControllerStoryboardingMethods"]
    unsafe impl NSWindowController {
        #[method_id(storyboard)]
        pub unsafe fn storyboard(&self) -> Option<Id<NSStoryboard, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSWindowControllerDismissing"]
    unsafe impl NSWindowController {
        #[method(dismissController:)]
        pub unsafe fn dismissController(&self, sender: Option<&Object>);
    }
);