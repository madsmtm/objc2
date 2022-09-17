#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSItemProvider;
    unsafe impl ClassType for NSItemProvider {
        type Super = NSObject;
    }
);
impl NSItemProvider {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn registerDataRepresentationForTypeIdentifier_visibility_loadHandler(
        &self,
        typeIdentifier: &NSString,
        visibility: NSItemProviderRepresentationVisibility,
        loadHandler: TodoBlock,
    ) {
        msg_send![
            self,
            registerDataRepresentationForTypeIdentifier: typeIdentifier,
            visibility: visibility,
            loadHandler: loadHandler
        ]
    }
    pub unsafe fn registerFileRepresentationForTypeIdentifier_fileOptions_visibility_loadHandler(
        &self,
        typeIdentifier: &NSString,
        fileOptions: NSItemProviderFileOptions,
        visibility: NSItemProviderRepresentationVisibility,
        loadHandler: TodoBlock,
    ) {
        msg_send![
            self,
            registerFileRepresentationForTypeIdentifier: typeIdentifier,
            fileOptions: fileOptions,
            visibility: visibility,
            loadHandler: loadHandler
        ]
    }
    pub unsafe fn registeredTypeIdentifiersWithFileOptions(
        &self,
        fileOptions: NSItemProviderFileOptions,
    ) -> TodoGenerics {
        msg_send![self, registeredTypeIdentifiersWithFileOptions: fileOptions]
    }
    pub unsafe fn hasItemConformingToTypeIdentifier(&self, typeIdentifier: &NSString) -> bool {
        msg_send![self, hasItemConformingToTypeIdentifier: typeIdentifier]
    }
    pub unsafe fn hasRepresentationConformingToTypeIdentifier_fileOptions(
        &self,
        typeIdentifier: &NSString,
        fileOptions: NSItemProviderFileOptions,
    ) -> bool {
        msg_send![
            self,
            hasRepresentationConformingToTypeIdentifier: typeIdentifier,
            fileOptions: fileOptions
        ]
    }
    pub unsafe fn loadDataRepresentationForTypeIdentifier_completionHandler(
        &self,
        typeIdentifier: &NSString,
        completionHandler: TodoBlock,
    ) -> Id<NSProgress, Shared> {
        msg_send_id![
            self,
            loadDataRepresentationForTypeIdentifier: typeIdentifier,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn loadFileRepresentationForTypeIdentifier_completionHandler(
        &self,
        typeIdentifier: &NSString,
        completionHandler: TodoBlock,
    ) -> Id<NSProgress, Shared> {
        msg_send_id![
            self,
            loadFileRepresentationForTypeIdentifier: typeIdentifier,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn loadInPlaceFileRepresentationForTypeIdentifier_completionHandler(
        &self,
        typeIdentifier: &NSString,
        completionHandler: TodoBlock,
    ) -> Id<NSProgress, Shared> {
        msg_send_id![
            self,
            loadInPlaceFileRepresentationForTypeIdentifier: typeIdentifier,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn initWithObject(&self, object: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![self, initWithObject: object]
    }
    pub unsafe fn registerObject_visibility(
        &self,
        object: TodoGenerics,
        visibility: NSItemProviderRepresentationVisibility,
    ) {
        msg_send![self, registerObject: object, visibility: visibility]
    }
    pub unsafe fn registerObjectOfClass_visibility_loadHandler(
        &self,
        aClass: TodoGenerics,
        visibility: NSItemProviderRepresentationVisibility,
        loadHandler: TodoBlock,
    ) {
        msg_send![
            self,
            registerObjectOfClass: aClass,
            visibility: visibility,
            loadHandler: loadHandler
        ]
    }
    pub unsafe fn canLoadObjectOfClass(&self, aClass: TodoGenerics) -> bool {
        msg_send![self, canLoadObjectOfClass: aClass]
    }
    pub unsafe fn loadObjectOfClass_completionHandler(
        &self,
        aClass: TodoGenerics,
        completionHandler: TodoBlock,
    ) -> Id<NSProgress, Shared> {
        msg_send_id![
            self,
            loadObjectOfClass: aClass,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn initWithItem_typeIdentifier(
        &self,
        item: TodoGenerics,
        typeIdentifier: Option<&NSString>,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithItem: item, typeIdentifier: typeIdentifier]
    }
    pub unsafe fn initWithContentsOfURL(
        &self,
        fileURL: Option<&NSURL>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithContentsOfURL: fileURL]
    }
    pub unsafe fn registerItemForTypeIdentifier_loadHandler(
        &self,
        typeIdentifier: &NSString,
        loadHandler: NSItemProviderLoadHandler,
    ) {
        msg_send![
            self,
            registerItemForTypeIdentifier: typeIdentifier,
            loadHandler: loadHandler
        ]
    }
    pub unsafe fn loadItemForTypeIdentifier_options_completionHandler(
        &self,
        typeIdentifier: &NSString,
        options: Option<&NSDictionary>,
        completionHandler: NSItemProviderCompletionHandler,
    ) {
        msg_send![
            self,
            loadItemForTypeIdentifier: typeIdentifier,
            options: options,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn registeredTypeIdentifiers(&self) -> TodoGenerics {
        msg_send![self, registeredTypeIdentifiers]
    }
    pub unsafe fn suggestedName(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, suggestedName]
    }
    pub unsafe fn setSuggestedName(&self, suggestedName: Option<&NSString>) {
        msg_send![self, setSuggestedName: suggestedName]
    }
}
#[doc = "NSPreviewSupport"]
impl NSItemProvider {
    pub unsafe fn loadPreviewImageWithOptions_completionHandler(
        &self,
        options: Option<&NSDictionary>,
        completionHandler: NSItemProviderCompletionHandler,
    ) {
        msg_send![
            self,
            loadPreviewImageWithOptions: options,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn previewImageHandler(&self) -> NSItemProviderLoadHandler {
        msg_send![self, previewImageHandler]
    }
    pub unsafe fn setPreviewImageHandler(&self, previewImageHandler: NSItemProviderLoadHandler) {
        msg_send![self, setPreviewImageHandler: previewImageHandler]
    }
}
