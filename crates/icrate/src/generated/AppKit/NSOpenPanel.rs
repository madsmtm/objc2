use super::__exported::NSString;
use super::__exported::NSWindow;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSSavePanel::*;
use crate::Foundation::generated::NSArray::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSOpenPanel;
    unsafe impl ClassType for NSOpenPanel {
        type Super = NSSavePanel;
    }
);
extern_methods!(
    unsafe impl NSOpenPanel {
        #[method_id(openPanel)]
        pub unsafe fn openPanel() -> Id<NSOpenPanel, Shared>;
        #[method_id(URLs)]
        pub unsafe fn URLs(&self) -> Id<NSArray<NSURL>, Shared>;
        #[method(resolvesAliases)]
        pub unsafe fn resolvesAliases(&self) -> bool;
        #[method(setResolvesAliases:)]
        pub unsafe fn setResolvesAliases(&self, resolvesAliases: bool);
        #[method(canChooseDirectories)]
        pub unsafe fn canChooseDirectories(&self) -> bool;
        #[method(setCanChooseDirectories:)]
        pub unsafe fn setCanChooseDirectories(&self, canChooseDirectories: bool);
        #[method(allowsMultipleSelection)]
        pub unsafe fn allowsMultipleSelection(&self) -> bool;
        #[method(setAllowsMultipleSelection:)]
        pub unsafe fn setAllowsMultipleSelection(&self, allowsMultipleSelection: bool);
        #[method(canChooseFiles)]
        pub unsafe fn canChooseFiles(&self) -> bool;
        #[method(setCanChooseFiles:)]
        pub unsafe fn setCanChooseFiles(&self, canChooseFiles: bool);
        #[method(canResolveUbiquitousConflicts)]
        pub unsafe fn canResolveUbiquitousConflicts(&self) -> bool;
        #[method(setCanResolveUbiquitousConflicts:)]
        pub unsafe fn setCanResolveUbiquitousConflicts(&self, canResolveUbiquitousConflicts: bool);
        #[method(canDownloadUbiquitousContents)]
        pub unsafe fn canDownloadUbiquitousContents(&self) -> bool;
        #[method(setCanDownloadUbiquitousContents:)]
        pub unsafe fn setCanDownloadUbiquitousContents(&self, canDownloadUbiquitousContents: bool);
        #[method(isAccessoryViewDisclosed)]
        pub unsafe fn isAccessoryViewDisclosed(&self) -> bool;
        #[method(setAccessoryViewDisclosed:)]
        pub unsafe fn setAccessoryViewDisclosed(&self, accessoryViewDisclosed: bool);
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSOpenPanel {
        #[method_id(filenames)]
        pub unsafe fn filenames(&self) -> Id<NSArray, Shared>;
        #[method(beginSheetForDirectory:file:types:modalForWindow:modalDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetForDirectory_file_types_modalForWindow_modalDelegate_didEndSelector_contextInfo(
            &self,
            path: Option<&NSString>,
            name: Option<&NSString>,
            fileTypes: Option<&NSArray>,
            docWindow: Option<&NSWindow>,
            delegate: Option<&Object>,
            didEndSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );
        #[method(beginForDirectory:file:types:modelessDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginForDirectory_file_types_modelessDelegate_didEndSelector_contextInfo(
            &self,
            path: Option<&NSString>,
            name: Option<&NSString>,
            fileTypes: Option<&NSArray>,
            delegate: Option<&Object>,
            didEndSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );
        #[method(runModalForDirectory:file:types:)]
        pub unsafe fn runModalForDirectory_file_types(
            &self,
            path: Option<&NSString>,
            name: Option<&NSString>,
            fileTypes: Option<&NSArray>,
        ) -> NSInteger;
        #[method(runModalForTypes:)]
        pub unsafe fn runModalForTypes(&self, fileTypes: Option<&NSArray>) -> NSInteger;
    }
);
