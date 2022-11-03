//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSSharingServiceName = NSString;

extern_static!(NSSharingServiceNameComposeEmail: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameComposeMessage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameSendViaAirDrop: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameAddToSafariReadingList: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameAddToIPhoto: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameAddToAperture: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsDesktopPicture: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnFacebook: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnTwitter: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnSinaWeibo: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnTencentWeibo: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnLinkedIn: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsTwitterProfileImage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsFacebookProfileImage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsLinkedInProfileImage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostImageOnFlickr: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostVideoOnVimeo: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostVideoOnYouku: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostVideoOnTudou: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameCloudSharing: &'static NSSharingServiceName);

extern_class!(
    #[derive(Debug)]
    pub struct NSSharingService;

    unsafe impl ClassType for NSSharingService {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSSharingService {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSSharingServiceDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSSharingServiceDelegate>);

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Id<NSImage, Shared>;

        #[method_id(@__retain_semantics Other alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Id<NSImage, Shared>>;

        #[method_id(@__retain_semantics Other menuItemTitle)]
        pub unsafe fn menuItemTitle(&self) -> Id<NSString, Shared>;

        #[method(setMenuItemTitle:)]
        pub unsafe fn setMenuItemTitle(&self, menuItemTitle: &NSString);

        #[method_id(@__retain_semantics Other recipients)]
        pub unsafe fn recipients(&self) -> Option<Id<NSArray<NSString>, Shared>>;

        #[method(setRecipients:)]
        pub unsafe fn setRecipients(&self, recipients: Option<&NSArray<NSString>>);

        #[method_id(@__retain_semantics Other subject)]
        pub unsafe fn subject(&self) -> Option<Id<NSString, Shared>>;

        #[method(setSubject:)]
        pub unsafe fn setSubject(&self, subject: Option<&NSString>);

        #[method_id(@__retain_semantics Other messageBody)]
        pub unsafe fn messageBody(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other permanentLink)]
        pub unsafe fn permanentLink(&self) -> Option<Id<NSURL, Shared>>;

        #[method_id(@__retain_semantics Other accountName)]
        pub unsafe fn accountName(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other attachmentFileURLs)]
        pub unsafe fn attachmentFileURLs(&self) -> Option<Id<NSArray<NSURL>, Shared>>;

        #[method_id(@__retain_semantics Other sharingServicesForItems:)]
        pub unsafe fn sharingServicesForItems(
            items: &NSArray,
        ) -> Id<NSArray<NSSharingService>, Shared>;

        #[method_id(@__retain_semantics Other sharingServiceNamed:)]
        pub unsafe fn sharingServiceNamed(
            serviceName: &NSSharingServiceName,
        ) -> Option<Id<NSSharingService, Shared>>;

        #[method_id(@__retain_semantics Init initWithTitle:image:alternateImage:handler:)]
        pub unsafe fn initWithTitle_image_alternateImage_handler(
            this: Option<Allocated<Self>>,
            title: &NSString,
            image: &NSImage,
            alternateImage: Option<&NSImage>,
            block: &Block<(), ()>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(canPerformWithItems:)]
        pub unsafe fn canPerformWithItems(&self, items: Option<&NSArray>) -> bool;

        #[method(performWithItems:)]
        pub unsafe fn performWithItems(&self, items: &NSArray);
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSSharingContentScope {
        NSSharingContentScopeItem = 0,
        NSSharingContentScopePartial = 1,
        NSSharingContentScopeFull = 2,
    }
);

pub type NSSharingServiceDelegate = NSObject;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSCloudKitSharingServiceOptions {
        NSCloudKitSharingServiceStandard = 0,
        NSCloudKitSharingServiceAllowPublic = 1 << 0,
        NSCloudKitSharingServiceAllowPrivate = 1 << 1,
        NSCloudKitSharingServiceAllowReadOnly = 1 << 4,
        NSCloudKitSharingServiceAllowReadWrite = 1 << 5,
    }
);

pub type NSCloudSharingServiceDelegate = NSObject;

extern_methods!(
    /// NSCloudKitSharing
    unsafe impl NSItemProvider {
        #[method(registerCloudKitShareWithPreparationHandler:)]
        pub unsafe fn registerCloudKitShareWithPreparationHandler(
            &self,
            preparationHandler: &Block<
                (NonNull<Block<(*mut CKShare, *mut CKContainer, *mut NSError), ()>>,),
                (),
            >,
        );

        #[method(registerCloudKitShare:container:)]
        pub unsafe fn registerCloudKitShare_container(
            &self,
            share: &CKShare,
            container: &CKContainer,
        );
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSSharingServicePicker;

    unsafe impl ClassType for NSSharingServicePicker {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSSharingServicePicker {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSSharingServicePickerDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSSharingServicePickerDelegate>);

        #[method_id(@__retain_semantics Init initWithItems:)]
        pub unsafe fn initWithItems(
            this: Option<Allocated<Self>>,
            items: &NSArray,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method(showRelativeToRect:ofView:preferredEdge:)]
        pub unsafe fn showRelativeToRect_ofView_preferredEdge(
            &self,
            rect: NSRect,
            view: &NSView,
            preferredEdge: NSRectEdge,
        );
    }
);

pub type NSSharingServicePickerDelegate = NSObject;
