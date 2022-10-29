#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAlert;
    unsafe impl ClassType for NSAlert {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAlert {
        #[method_id(alertWithError:)]
        pub unsafe fn alertWithError(error: &NSError) -> Id<NSAlert, Shared>;
        #[method_id(messageText)]
        pub unsafe fn messageText(&self) -> Id<NSString, Shared>;
        #[method(setMessageText:)]
        pub unsafe fn setMessageText(&self, messageText: &NSString);
        #[method_id(informativeText)]
        pub unsafe fn informativeText(&self) -> Id<NSString, Shared>;
        #[method(setInformativeText:)]
        pub unsafe fn setInformativeText(&self, informativeText: &NSString);
        #[method_id(icon)]
        pub unsafe fn icon(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setIcon:)]
        pub unsafe fn setIcon(&self, icon: Option<&NSImage>);
        #[method_id(addButtonWithTitle:)]
        pub unsafe fn addButtonWithTitle(&self, title: &NSString) -> Id<NSButton, Shared>;
        #[method_id(buttons)]
        pub unsafe fn buttons(&self) -> Id<NSArray<NSButton>, Shared>;
        #[method(showsHelp)]
        pub unsafe fn showsHelp(&self) -> bool;
        #[method(setShowsHelp:)]
        pub unsafe fn setShowsHelp(&self, showsHelp: bool);
        #[method_id(helpAnchor)]
        pub unsafe fn helpAnchor(&self) -> Option<Id<NSHelpAnchorName, Shared>>;
        #[method(setHelpAnchor:)]
        pub unsafe fn setHelpAnchor(&self, helpAnchor: Option<&NSHelpAnchorName>);
        #[method(alertStyle)]
        pub unsafe fn alertStyle(&self) -> NSAlertStyle;
        #[method(setAlertStyle:)]
        pub unsafe fn setAlertStyle(&self, alertStyle: NSAlertStyle);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSAlertDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSAlertDelegate>);
        #[method(showsSuppressionButton)]
        pub unsafe fn showsSuppressionButton(&self) -> bool;
        #[method(setShowsSuppressionButton:)]
        pub unsafe fn setShowsSuppressionButton(&self, showsSuppressionButton: bool);
        #[method_id(suppressionButton)]
        pub unsafe fn suppressionButton(&self) -> Option<Id<NSButton, Shared>>;
        #[method_id(accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessoryView: Option<&NSView>);
        #[method(layout)]
        pub unsafe fn layout(&self);
        #[method(runModal)]
        pub unsafe fn runModal(&self) -> NSModalResponse;
        #[method(beginSheetModalForWindow:completionHandler:)]
        pub unsafe fn beginSheetModalForWindow_completionHandler(
            &self,
            sheetWindow: &NSWindow,
            handler: TodoBlock,
        );
        #[method_id(window)]
        pub unsafe fn window(&self) -> Id<NSWindow, Shared>;
    }
);
pub type NSAlertDelegate = NSObject;
extern_methods!(
    #[doc = "NSAlertDeprecated"]
    unsafe impl NSAlert {
        #[method(beginSheetModalForWindow:modalDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetModalForWindow_modalDelegate_didEndSelector_contextInfo(
            &self,
            window: &NSWindow,
            delegate: Option<&Object>,
            didEndSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );
    }
);