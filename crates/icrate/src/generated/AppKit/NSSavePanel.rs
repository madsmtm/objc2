#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSavePanel;
    unsafe impl ClassType for NSSavePanel {
        type Super = NSPanel;
    }
);
extern_methods!(
    unsafe impl NSSavePanel {
        #[method_id(savePanel)]
        pub unsafe fn savePanel() -> Id<NSSavePanel, Shared>;
        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(directoryURL)]
        pub unsafe fn directoryURL(&self) -> Option<Id<NSURL, Shared>>;
        #[method(setDirectoryURL:)]
        pub unsafe fn setDirectoryURL(&self, directoryURL: Option<&NSURL>);
        #[method_id(allowedContentTypes)]
        pub unsafe fn allowedContentTypes(&self) -> Id<NSArray<UTType>, Shared>;
        #[method(setAllowedContentTypes:)]
        pub unsafe fn setAllowedContentTypes(&self, allowedContentTypes: &NSArray<UTType>);
        #[method(allowsOtherFileTypes)]
        pub unsafe fn allowsOtherFileTypes(&self) -> bool;
        #[method(setAllowsOtherFileTypes:)]
        pub unsafe fn setAllowsOtherFileTypes(&self, allowsOtherFileTypes: bool);
        #[method_id(accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessoryView: Option<&NSView>);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSOpenSavePanelDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSOpenSavePanelDelegate>);
        #[method(isExpanded)]
        pub unsafe fn isExpanded(&self) -> bool;
        #[method(canCreateDirectories)]
        pub unsafe fn canCreateDirectories(&self) -> bool;
        #[method(setCanCreateDirectories:)]
        pub unsafe fn setCanCreateDirectories(&self, canCreateDirectories: bool);
        #[method(canSelectHiddenExtension)]
        pub unsafe fn canSelectHiddenExtension(&self) -> bool;
        #[method(setCanSelectHiddenExtension:)]
        pub unsafe fn setCanSelectHiddenExtension(&self, canSelectHiddenExtension: bool);
        #[method(isExtensionHidden)]
        pub unsafe fn isExtensionHidden(&self) -> bool;
        #[method(setExtensionHidden:)]
        pub unsafe fn setExtensionHidden(&self, extensionHidden: bool);
        #[method(treatsFilePackagesAsDirectories)]
        pub unsafe fn treatsFilePackagesAsDirectories(&self) -> bool;
        #[method(setTreatsFilePackagesAsDirectories:)]
        pub unsafe fn setTreatsFilePackagesAsDirectories(
            &self,
            treatsFilePackagesAsDirectories: bool,
        );
        #[method_id(prompt)]
        pub unsafe fn prompt(&self) -> Id<NSString, Shared>;
        #[method(setPrompt:)]
        pub unsafe fn setPrompt(&self, prompt: Option<&NSString>);
        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);
        #[method_id(nameFieldLabel)]
        pub unsafe fn nameFieldLabel(&self) -> Id<NSString, Shared>;
        #[method(setNameFieldLabel:)]
        pub unsafe fn setNameFieldLabel(&self, nameFieldLabel: Option<&NSString>);
        #[method_id(nameFieldStringValue)]
        pub unsafe fn nameFieldStringValue(&self) -> Id<NSString, Shared>;
        #[method(setNameFieldStringValue:)]
        pub unsafe fn setNameFieldStringValue(&self, nameFieldStringValue: &NSString);
        #[method_id(message)]
        pub unsafe fn message(&self) -> Id<NSString, Shared>;
        #[method(setMessage:)]
        pub unsafe fn setMessage(&self, message: Option<&NSString>);
        #[method(validateVisibleColumns)]
        pub unsafe fn validateVisibleColumns(&self);
        #[method(showsHiddenFiles)]
        pub unsafe fn showsHiddenFiles(&self) -> bool;
        #[method(setShowsHiddenFiles:)]
        pub unsafe fn setShowsHiddenFiles(&self, showsHiddenFiles: bool);
        #[method(showsTagField)]
        pub unsafe fn showsTagField(&self) -> bool;
        #[method(setShowsTagField:)]
        pub unsafe fn setShowsTagField(&self, showsTagField: bool);
        #[method_id(tagNames)]
        pub unsafe fn tagNames(&self) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method(setTagNames:)]
        pub unsafe fn setTagNames(&self, tagNames: Option<&NSArray<NSString>>);
        #[method(ok:)]
        pub unsafe fn ok(&self, sender: Option<&Object>);
        #[method(cancel:)]
        pub unsafe fn cancel(&self, sender: Option<&Object>);
        #[method(beginSheetModalForWindow:completionHandler:)]
        pub unsafe fn beginSheetModalForWindow_completionHandler(
            &self,
            window: &NSWindow,
            handler: TodoBlock,
        );
        #[method(beginWithCompletionHandler:)]
        pub unsafe fn beginWithCompletionHandler(&self, handler: TodoBlock);
        #[method(runModal)]
        pub unsafe fn runModal(&self) -> NSModalResponse;
    }
);
pub type NSOpenSavePanelDelegate = NSObject;
extern_methods!(
    #[doc = "NSSavePanelDelegateDeprecated"]
    unsafe impl NSObject {
        #[method(panel:isValidFilename:)]
        pub unsafe fn panel_isValidFilename(&self, sender: &Object, filename: &NSString) -> bool;
        #[method(panel:directoryDidChange:)]
        pub unsafe fn panel_directoryDidChange(&self, sender: &Object, path: &NSString);
        #[method(panel:compareFilename:with:caseSensitive:)]
        pub unsafe fn panel_compareFilename_with_caseSensitive(
            &self,
            sender: &Object,
            name1: &NSString,
            name2: &NSString,
            caseSensitive: bool,
        ) -> NSComparisonResult;
        #[method(panel:shouldShowFilename:)]
        pub unsafe fn panel_shouldShowFilename(&self, sender: &Object, filename: &NSString)
            -> bool;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSSavePanel {
        #[method_id(filename)]
        pub unsafe fn filename(&self) -> Id<NSString, Shared>;
        #[method_id(directory)]
        pub unsafe fn directory(&self) -> Id<NSString, Shared>;
        #[method(setDirectory:)]
        pub unsafe fn setDirectory(&self, path: Option<&NSString>);
        #[method_id(requiredFileType)]
        pub unsafe fn requiredFileType(&self) -> Option<Id<NSString, Shared>>;
        #[method(setRequiredFileType:)]
        pub unsafe fn setRequiredFileType(&self, type_: Option<&NSString>);
        #[method(beginSheetForDirectory:file:modalForWindow:modalDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetForDirectory_file_modalForWindow_modalDelegate_didEndSelector_contextInfo(
            &self,
            path: &NSString,
            name: Option<&NSString>,
            docWindow: Option<&NSWindow>,
            delegate: Option<&Object>,
            didEndSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );
        #[method(runModalForDirectory:file:)]
        pub unsafe fn runModalForDirectory_file(
            &self,
            path: Option<&NSString>,
            name: Option<&NSString>,
        ) -> NSInteger;
        #[method(selectText:)]
        pub unsafe fn selectText(&self, sender: Option<&Object>);
        #[method_id(allowedFileTypes)]
        pub unsafe fn allowedFileTypes(&self) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method(setAllowedFileTypes:)]
        pub unsafe fn setAllowedFileTypes(&self, allowedFileTypes: Option<&NSArray<NSString>>);
    }
);