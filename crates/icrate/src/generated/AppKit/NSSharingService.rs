use super::__exported::CKContainer;
use super::__exported::CKShare;
use super::__exported::NSError;
use super::__exported::NSImage;
use super::__exported::NSString;
use super::__exported::NSView;
use super::__exported::NSWindow;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSPasteboard::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSItemProvider::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSSharingServiceName = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSSharingService;
    unsafe impl ClassType for NSSharingService {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSSharingService {
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSSharingServiceDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSSharingServiceDelegate>);
        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;
        #[method_id(image)]
        pub unsafe fn image(&self) -> Id<NSImage, Shared>;
        #[method_id(alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Id<NSImage, Shared>>;
        #[method_id(menuItemTitle)]
        pub unsafe fn menuItemTitle(&self) -> Id<NSString, Shared>;
        #[method(setMenuItemTitle:)]
        pub unsafe fn setMenuItemTitle(&self, menuItemTitle: &NSString);
        #[method_id(recipients)]
        pub unsafe fn recipients(&self) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method(setRecipients:)]
        pub unsafe fn setRecipients(&self, recipients: Option<&NSArray<NSString>>);
        #[method_id(subject)]
        pub unsafe fn subject(&self) -> Option<Id<NSString, Shared>>;
        #[method(setSubject:)]
        pub unsafe fn setSubject(&self, subject: Option<&NSString>);
        #[method_id(messageBody)]
        pub unsafe fn messageBody(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(permanentLink)]
        pub unsafe fn permanentLink(&self) -> Option<Id<NSURL, Shared>>;
        #[method_id(accountName)]
        pub unsafe fn accountName(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(attachmentFileURLs)]
        pub unsafe fn attachmentFileURLs(&self) -> Option<Id<NSArray<NSURL>, Shared>>;
        #[method_id(sharingServicesForItems:)]
        pub unsafe fn sharingServicesForItems(
            items: &NSArray,
        ) -> Id<NSArray<NSSharingService>, Shared>;
        #[method_id(sharingServiceNamed:)]
        pub unsafe fn sharingServiceNamed(
            serviceName: &NSSharingServiceName,
        ) -> Option<Id<NSSharingService, Shared>>;
        #[method_id(initWithTitle:image:alternateImage:handler:)]
        pub unsafe fn initWithTitle_image_alternateImage_handler(
            &self,
            title: &NSString,
            image: &NSImage,
            alternateImage: Option<&NSImage>,
            block: TodoBlock,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method(canPerformWithItems:)]
        pub unsafe fn canPerformWithItems(&self, items: Option<&NSArray>) -> bool;
        #[method(performWithItems:)]
        pub unsafe fn performWithItems(&self, items: &NSArray);
    }
);
pub type NSSharingServiceDelegate = NSObject;
pub type NSCloudSharingServiceDelegate = NSObject;
extern_methods!(
    #[doc = "NSCloudKitSharing"]
    unsafe impl NSItemProvider {
        #[method(registerCloudKitShareWithPreparationHandler:)]
        pub unsafe fn registerCloudKitShareWithPreparationHandler(
            &self,
            preparationHandler: TodoBlock,
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
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSSharingServicePickerDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSSharingServicePickerDelegate>);
        #[method_id(initWithItems:)]
        pub unsafe fn initWithItems(&self, items: &NSArray) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
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
