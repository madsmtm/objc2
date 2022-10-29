#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDocumentController;
    unsafe impl ClassType for NSDocumentController {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSDocumentController {
        #[method_id(sharedDocumentController)]
        pub unsafe fn sharedDocumentController() -> Id<NSDocumentController, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(documents)]
        pub unsafe fn documents(&self) -> Id<NSArray<NSDocument>, Shared>;
        #[method_id(currentDocument)]
        pub unsafe fn currentDocument(&self) -> Option<Id<NSDocument, Shared>>;
        #[method_id(currentDirectory)]
        pub unsafe fn currentDirectory(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(documentForURL:)]
        pub unsafe fn documentForURL(&self, url: &NSURL) -> Option<Id<NSDocument, Shared>>;
        #[method_id(documentForWindow:)]
        pub unsafe fn documentForWindow(&self, window: &NSWindow)
            -> Option<Id<NSDocument, Shared>>;
        #[method(addDocument:)]
        pub unsafe fn addDocument(&self, document: &NSDocument);
        #[method(removeDocument:)]
        pub unsafe fn removeDocument(&self, document: &NSDocument);
        #[method(newDocument:)]
        pub unsafe fn newDocument(&self, sender: Option<&Object>);
        #[method_id(openUntitledDocumentAndDisplay:error:)]
        pub unsafe fn openUntitledDocumentAndDisplay_error(
            &self,
            displayDocument: bool,
        ) -> Result<Id<NSDocument, Shared>, Id<NSError, Shared>>;
        #[method_id(makeUntitledDocumentOfType:error:)]
        pub unsafe fn makeUntitledDocumentOfType_error(
            &self,
            typeName: &NSString,
        ) -> Result<Id<NSDocument, Shared>, Id<NSError, Shared>>;
        #[method(openDocument:)]
        pub unsafe fn openDocument(&self, sender: Option<&Object>);
        #[method_id(URLsFromRunningOpenPanel)]
        pub unsafe fn URLsFromRunningOpenPanel(&self) -> Option<Id<NSArray<NSURL>, Shared>>;
        #[method(runModalOpenPanel:forTypes:)]
        pub unsafe fn runModalOpenPanel_forTypes(
            &self,
            openPanel: &NSOpenPanel,
            types: Option<&NSArray<NSString>>,
        ) -> NSInteger;
        #[method(beginOpenPanelWithCompletionHandler:)]
        pub unsafe fn beginOpenPanelWithCompletionHandler(&self, completionHandler: TodoBlock);
        #[method(beginOpenPanel:forTypes:completionHandler:)]
        pub unsafe fn beginOpenPanel_forTypes_completionHandler(
            &self,
            openPanel: &NSOpenPanel,
            inTypes: Option<&NSArray<NSString>>,
            completionHandler: TodoBlock,
        );
        #[method(openDocumentWithContentsOfURL:display:completionHandler:)]
        pub unsafe fn openDocumentWithContentsOfURL_display_completionHandler(
            &self,
            url: &NSURL,
            displayDocument: bool,
            completionHandler: TodoBlock,
        );
        #[method_id(makeDocumentWithContentsOfURL:ofType:error:)]
        pub unsafe fn makeDocumentWithContentsOfURL_ofType_error(
            &self,
            url: &NSURL,
            typeName: &NSString,
        ) -> Result<Id<NSDocument, Shared>, Id<NSError, Shared>>;
        #[method(reopenDocumentForURL:withContentsOfURL:display:completionHandler:)]
        pub unsafe fn reopenDocumentForURL_withContentsOfURL_display_completionHandler(
            &self,
            urlOrNil: Option<&NSURL>,
            contentsURL: &NSURL,
            displayDocument: bool,
            completionHandler: TodoBlock,
        );
        #[method_id(makeDocumentForURL:withContentsOfURL:ofType:error:)]
        pub unsafe fn makeDocumentForURL_withContentsOfURL_ofType_error(
            &self,
            urlOrNil: Option<&NSURL>,
            contentsURL: &NSURL,
            typeName: &NSString,
        ) -> Result<Id<NSDocument, Shared>, Id<NSError, Shared>>;
        #[method(autosavingDelay)]
        pub unsafe fn autosavingDelay(&self) -> NSTimeInterval;
        #[method(setAutosavingDelay:)]
        pub unsafe fn setAutosavingDelay(&self, autosavingDelay: NSTimeInterval);
        #[method(saveAllDocuments:)]
        pub unsafe fn saveAllDocuments(&self, sender: Option<&Object>);
        #[method(hasEditedDocuments)]
        pub unsafe fn hasEditedDocuments(&self) -> bool;
        #[method(reviewUnsavedDocumentsWithAlertTitle:cancellable:delegate:didReviewAllSelector:contextInfo:)]
        pub unsafe fn reviewUnsavedDocumentsWithAlertTitle_cancellable_delegate_didReviewAllSelector_contextInfo(
            &self,
            title: Option<&NSString>,
            cancellable: bool,
            delegate: Option<&Object>,
            didReviewAllSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );
        #[method(closeAllDocumentsWithDelegate:didCloseAllSelector:contextInfo:)]
        pub unsafe fn closeAllDocumentsWithDelegate_didCloseAllSelector_contextInfo(
            &self,
            delegate: Option<&Object>,
            didCloseAllSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );
        #[method_id(duplicateDocumentWithContentsOfURL:copying:displayName:error:)]
        pub unsafe fn duplicateDocumentWithContentsOfURL_copying_displayName_error(
            &self,
            url: &NSURL,
            duplicateByCopying: bool,
            displayNameOrNil: Option<&NSString>,
        ) -> Result<Id<NSDocument, Shared>, Id<NSError, Shared>>;
        #[method(allowsAutomaticShareMenu)]
        pub unsafe fn allowsAutomaticShareMenu(&self) -> bool;
        #[method_id(standardShareMenuItem)]
        pub unsafe fn standardShareMenuItem(&self) -> Id<NSMenuItem, Shared>;
        #[method(presentError:modalForWindow:delegate:didPresentSelector:contextInfo:)]
        pub unsafe fn presentError_modalForWindow_delegate_didPresentSelector_contextInfo(
            &self,
            error: &NSError,
            window: &NSWindow,
            delegate: Option<&Object>,
            didPresentSelector: Option<Sel>,
            contextInfo: *mut c_void,
        );
        #[method(presentError:)]
        pub unsafe fn presentError(&self, error: &NSError) -> bool;
        #[method_id(willPresentError:)]
        pub unsafe fn willPresentError(&self, error: &NSError) -> Id<NSError, Shared>;
        #[method(maximumRecentDocumentCount)]
        pub unsafe fn maximumRecentDocumentCount(&self) -> NSUInteger;
        #[method(clearRecentDocuments:)]
        pub unsafe fn clearRecentDocuments(&self, sender: Option<&Object>);
        #[method(noteNewRecentDocument:)]
        pub unsafe fn noteNewRecentDocument(&self, document: &NSDocument);
        #[method(noteNewRecentDocumentURL:)]
        pub unsafe fn noteNewRecentDocumentURL(&self, url: &NSURL);
        #[method_id(recentDocumentURLs)]
        pub unsafe fn recentDocumentURLs(&self) -> Id<NSArray<NSURL>, Shared>;
        #[method_id(defaultType)]
        pub unsafe fn defaultType(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(typeForContentsOfURL:error:)]
        pub unsafe fn typeForContentsOfURL_error(
            &self,
            url: &NSURL,
        ) -> Result<Id<NSString, Shared>, Id<NSError, Shared>>;
        #[method_id(documentClassNames)]
        pub unsafe fn documentClassNames(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(documentClassForType:)]
        pub unsafe fn documentClassForType(&self, typeName: &NSString) -> Option<&Class>;
        #[method_id(displayNameForType:)]
        pub unsafe fn displayNameForType(
            &self,
            typeName: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method(validateUserInterfaceItem:)]
        pub unsafe fn validateUserInterfaceItem(&self, item: &NSValidatedUserInterfaceItem)
            -> bool;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSDocumentController {
        #[method_id(openDocumentWithContentsOfURL:display:error:)]
        pub unsafe fn openDocumentWithContentsOfURL_display_error(
            &self,
            url: &NSURL,
            displayDocument: bool,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;
        #[method(reopenDocumentForURL:withContentsOfURL:error:)]
        pub unsafe fn reopenDocumentForURL_withContentsOfURL_error(
            &self,
            url: Option<&NSURL>,
            contentsURL: &NSURL,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method_id(fileExtensionsFromType:)]
        pub unsafe fn fileExtensionsFromType(
            &self,
            typeName: &NSString,
        ) -> Option<Id<NSArray, Shared>>;
        #[method_id(typeFromFileExtension:)]
        pub unsafe fn typeFromFileExtension(
            &self,
            fileNameExtensionOrHFSFileType: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(documentForFileName:)]
        pub unsafe fn documentForFileName(&self, fileName: &NSString)
            -> Option<Id<Object, Shared>>;
        #[method_id(fileNamesFromRunningOpenPanel)]
        pub unsafe fn fileNamesFromRunningOpenPanel(&self) -> Option<Id<NSArray, Shared>>;
        #[method_id(makeDocumentWithContentsOfFile:ofType:)]
        pub unsafe fn makeDocumentWithContentsOfFile_ofType(
            &self,
            fileName: &NSString,
            type_: &NSString,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(makeDocumentWithContentsOfURL:ofType:)]
        pub unsafe fn makeDocumentWithContentsOfURL_ofType(
            &self,
            url: &NSURL,
            type_: Option<&NSString>,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(makeUntitledDocumentOfType:)]
        pub unsafe fn makeUntitledDocumentOfType(
            &self,
            type_: &NSString,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(openDocumentWithContentsOfFile:display:)]
        pub unsafe fn openDocumentWithContentsOfFile_display(
            &self,
            fileName: &NSString,
            display: bool,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(openDocumentWithContentsOfURL:display:)]
        pub unsafe fn openDocumentWithContentsOfURL_display(
            &self,
            url: &NSURL,
            display: bool,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(openUntitledDocumentOfType:display:)]
        pub unsafe fn openUntitledDocumentOfType_display(
            &self,
            type_: &NSString,
            display: bool,
        ) -> Option<Id<Object, Shared>>;
        #[method(setShouldCreateUI:)]
        pub unsafe fn setShouldCreateUI(&self, flag: bool);
        #[method(shouldCreateUI)]
        pub unsafe fn shouldCreateUI(&self) -> bool;
    }
);
