//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSItemProviderRepresentationVisibility = NSInteger;
pub const NSItemProviderRepresentationVisibilityAll: NSItemProviderRepresentationVisibility = 0;
pub const NSItemProviderRepresentationVisibilityTeam: NSItemProviderRepresentationVisibility = 1;
pub const NSItemProviderRepresentationVisibilityGroup: NSItemProviderRepresentationVisibility = 2;
pub const NSItemProviderRepresentationVisibilityOwnProcess: NSItemProviderRepresentationVisibility =
    3;

pub type NSItemProviderFileOptions = NSInteger;
pub const NSItemProviderFileOptionOpenInPlace: NSItemProviderFileOptions = 1;

pub type NSItemProviderWriting = NSObject;

pub type NSItemProviderReading = NSObject;

extern_class!(
    #[derive(Debug)]
    pub struct NSItemProvider;

    unsafe impl ClassType for NSItemProvider {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSItemProvider {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;

        #[method(registerDataRepresentationForTypeIdentifier:visibility:loadHandler:)]
        pub unsafe fn registerDataRepresentationForTypeIdentifier_visibility_loadHandler(
            &self,
            typeIdentifier: &NSString,
            visibility: NSItemProviderRepresentationVisibility,
            loadHandler: TodoBlock,
        );

        #[method(registerFileRepresentationForTypeIdentifier:fileOptions:visibility:loadHandler:)]
        pub unsafe fn registerFileRepresentationForTypeIdentifier_fileOptions_visibility_loadHandler(
            &self,
            typeIdentifier: &NSString,
            fileOptions: NSItemProviderFileOptions,
            visibility: NSItemProviderRepresentationVisibility,
            loadHandler: TodoBlock,
        );

        #[method_id(registeredTypeIdentifiers)]
        pub unsafe fn registeredTypeIdentifiers(&self) -> Id<NSArray<NSString>, Shared>;

        #[method_id(registeredTypeIdentifiersWithFileOptions:)]
        pub unsafe fn registeredTypeIdentifiersWithFileOptions(
            &self,
            fileOptions: NSItemProviderFileOptions,
        ) -> Id<NSArray<NSString>, Shared>;

        #[method(hasItemConformingToTypeIdentifier:)]
        pub unsafe fn hasItemConformingToTypeIdentifier(&self, typeIdentifier: &NSString) -> bool;

        #[method(hasRepresentationConformingToTypeIdentifier:fileOptions:)]
        pub unsafe fn hasRepresentationConformingToTypeIdentifier_fileOptions(
            &self,
            typeIdentifier: &NSString,
            fileOptions: NSItemProviderFileOptions,
        ) -> bool;

        #[method_id(loadDataRepresentationForTypeIdentifier:completionHandler:)]
        pub unsafe fn loadDataRepresentationForTypeIdentifier_completionHandler(
            &self,
            typeIdentifier: &NSString,
            completionHandler: TodoBlock,
        ) -> Id<NSProgress, Shared>;

        #[method_id(loadFileRepresentationForTypeIdentifier:completionHandler:)]
        pub unsafe fn loadFileRepresentationForTypeIdentifier_completionHandler(
            &self,
            typeIdentifier: &NSString,
            completionHandler: TodoBlock,
        ) -> Id<NSProgress, Shared>;

        #[method_id(loadInPlaceFileRepresentationForTypeIdentifier:completionHandler:)]
        pub unsafe fn loadInPlaceFileRepresentationForTypeIdentifier_completionHandler(
            &self,
            typeIdentifier: &NSString,
            completionHandler: TodoBlock,
        ) -> Id<NSProgress, Shared>;

        #[method_id(suggestedName)]
        pub unsafe fn suggestedName(&self) -> Option<Id<NSString, Shared>>;

        #[method(setSuggestedName:)]
        pub unsafe fn setSuggestedName(&self, suggestedName: Option<&NSString>);

        #[method_id(initWithObject:)]
        pub unsafe fn initWithObject(&self, object: &NSItemProviderWriting) -> Id<Self, Shared>;

        #[method(registerObject:visibility:)]
        pub unsafe fn registerObject_visibility(
            &self,
            object: &NSItemProviderWriting,
            visibility: NSItemProviderRepresentationVisibility,
        );

        #[method_id(initWithItem:typeIdentifier:)]
        pub unsafe fn initWithItem_typeIdentifier(
            &self,
            item: Option<&NSSecureCoding>,
            typeIdentifier: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            &self,
            fileURL: Option<&NSURL>,
        ) -> Option<Id<Self, Shared>>;

        #[method(registerItemForTypeIdentifier:loadHandler:)]
        pub unsafe fn registerItemForTypeIdentifier_loadHandler(
            &self,
            typeIdentifier: &NSString,
            loadHandler: NSItemProviderLoadHandler,
        );

        #[method(loadItemForTypeIdentifier:options:completionHandler:)]
        pub unsafe fn loadItemForTypeIdentifier_options_completionHandler(
            &self,
            typeIdentifier: &NSString,
            options: Option<&NSDictionary>,
            completionHandler: NSItemProviderCompletionHandler,
        );
    }
);

extern "C" {
    pub static NSItemProviderPreferredImageSizeKey: &'static NSString;
}

extern_methods!(
    /// NSPreviewSupport
    unsafe impl NSItemProvider {
        #[method(previewImageHandler)]
        pub unsafe fn previewImageHandler(&self) -> NSItemProviderLoadHandler;

        #[method(setPreviewImageHandler:)]
        pub unsafe fn setPreviewImageHandler(&self, previewImageHandler: NSItemProviderLoadHandler);

        #[method(loadPreviewImageWithOptions:completionHandler:)]
        pub unsafe fn loadPreviewImageWithOptions_completionHandler(
            &self,
            options: Option<&NSDictionary>,
            completionHandler: NSItemProviderCompletionHandler,
        );
    }
);

extern "C" {
    pub static NSExtensionJavaScriptPreprocessingResultsKey: Option<&'static NSString>;
}

extern "C" {
    pub static NSExtensionJavaScriptFinalizeArgumentKey: Option<&'static NSString>;
}

extern "C" {
    pub static NSItemProviderErrorDomain: &'static NSString;
}

pub type NSItemProviderErrorCode = NSInteger;
pub const NSItemProviderUnknownError: NSItemProviderErrorCode = -1;
pub const NSItemProviderItemUnavailableError: NSItemProviderErrorCode = -1000;
pub const NSItemProviderUnexpectedValueClassError: NSItemProviderErrorCode = -1100;
pub const NSItemProviderUnavailableCoercionError: NSItemProviderErrorCode = -1200;
