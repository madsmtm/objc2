//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSAlertStyle = NSUInteger;
pub const NSAlertStyleWarning: NSAlertStyle = 0;
pub const NSAlertStyleInformational: NSAlertStyle = 1;
pub const NSAlertStyleCritical: NSAlertStyle = 2;

pub static NSAlertFirstButtonReturn: NSModalResponse = 1000;

pub static NSAlertSecondButtonReturn: NSModalResponse = 1001;

pub static NSAlertThirdButtonReturn: NSModalResponse = 1002;

extern_class!(
    #[derive(Debug)]
    pub struct NSAlert;

    unsafe impl ClassType for NSAlert {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSAlert {
        #[method_id(@__retain_semantics Other alertWithError:)]
        pub unsafe fn alertWithError(error: &NSError) -> Id<NSAlert, Shared>;

        #[method_id(@__retain_semantics Other messageText)]
        pub unsafe fn messageText(&self) -> Id<NSString, Shared>;

        #[method(setMessageText:)]
        pub unsafe fn setMessageText(&self, messageText: &NSString);

        #[method_id(@__retain_semantics Other informativeText)]
        pub unsafe fn informativeText(&self) -> Id<NSString, Shared>;

        #[method(setInformativeText:)]
        pub unsafe fn setInformativeText(&self, informativeText: &NSString);

        #[method_id(@__retain_semantics Other icon)]
        pub unsafe fn icon(&self) -> Option<Id<NSImage, Shared>>;

        #[method(setIcon:)]
        pub unsafe fn setIcon(&self, icon: Option<&NSImage>);

        #[method_id(@__retain_semantics Other addButtonWithTitle:)]
        pub unsafe fn addButtonWithTitle(&self, title: &NSString) -> Id<NSButton, Shared>;

        #[method_id(@__retain_semantics Other buttons)]
        pub unsafe fn buttons(&self) -> Id<NSArray<NSButton>, Shared>;

        #[method(showsHelp)]
        pub unsafe fn showsHelp(&self) -> bool;

        #[method(setShowsHelp:)]
        pub unsafe fn setShowsHelp(&self, showsHelp: bool);

        #[method_id(@__retain_semantics Other helpAnchor)]
        pub unsafe fn helpAnchor(&self) -> Option<Id<NSHelpAnchorName, Shared>>;

        #[method(setHelpAnchor:)]
        pub unsafe fn setHelpAnchor(&self, helpAnchor: Option<&NSHelpAnchorName>);

        #[method(alertStyle)]
        pub unsafe fn alertStyle(&self) -> NSAlertStyle;

        #[method(setAlertStyle:)]
        pub unsafe fn setAlertStyle(&self, alertStyle: NSAlertStyle);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSAlertDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSAlertDelegate>);

        #[method(showsSuppressionButton)]
        pub unsafe fn showsSuppressionButton(&self) -> bool;

        #[method(setShowsSuppressionButton:)]
        pub unsafe fn setShowsSuppressionButton(&self, showsSuppressionButton: bool);

        #[method_id(@__retain_semantics Other suppressionButton)]
        pub unsafe fn suppressionButton(&self) -> Option<Id<NSButton, Shared>>;

        #[method_id(@__retain_semantics Other accessoryView)]
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

        #[method_id(@__retain_semantics Other window)]
        pub unsafe fn window(&self) -> Id<NSWindow, Shared>;
    }
);

pub type NSAlertDelegate = NSObject;

extern_methods!(
    /// NSAlertDeprecated
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

pub static NSWarningAlertStyle: NSAlertStyle = NSAlertStyleWarning;

pub static NSInformationalAlertStyle: NSAlertStyle = NSAlertStyleInformational;

pub static NSCriticalAlertStyle: NSAlertStyle = NSAlertStyleCritical;
