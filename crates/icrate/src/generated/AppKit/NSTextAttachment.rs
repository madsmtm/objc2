#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSTextAttachmentContainer = NSObject;
pub type NSTextAttachmentLayout = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSTextAttachment;
    unsafe impl ClassType for NSTextAttachment {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextAttachment {
        #[method_id(initWithData:ofType:)]
        pub unsafe fn initWithData_ofType(
            &self,
            contentData: Option<&NSData>,
            uti: Option<&NSString>,
        ) -> Id<Self, Shared>;
        #[method_id(initWithFileWrapper:)]
        pub unsafe fn initWithFileWrapper(
            &self,
            fileWrapper: Option<&NSFileWrapper>,
        ) -> Id<Self, Shared>;
        #[method_id(contents)]
        pub unsafe fn contents(&self) -> Option<Id<NSData, Shared>>;
        #[method(setContents:)]
        pub unsafe fn setContents(&self, contents: Option<&NSData>);
        #[method_id(fileType)]
        pub unsafe fn fileType(&self) -> Option<Id<NSString, Shared>>;
        #[method(setFileType:)]
        pub unsafe fn setFileType(&self, fileType: Option<&NSString>);
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
        #[method(bounds)]
        pub unsafe fn bounds(&self) -> CGRect;
        #[method(setBounds:)]
        pub unsafe fn setBounds(&self, bounds: CGRect);
        #[method_id(fileWrapper)]
        pub unsafe fn fileWrapper(&self) -> Option<Id<NSFileWrapper, Shared>>;
        #[method(setFileWrapper:)]
        pub unsafe fn setFileWrapper(&self, fileWrapper: Option<&NSFileWrapper>);
        #[method_id(attachmentCell)]
        pub unsafe fn attachmentCell(&self) -> Option<Id<NSTextAttachmentCell, Shared>>;
        #[method(setAttachmentCell:)]
        pub unsafe fn setAttachmentCell(&self, attachmentCell: Option<&NSTextAttachmentCell>);
        #[method(lineLayoutPadding)]
        pub unsafe fn lineLayoutPadding(&self) -> CGFloat;
        #[method(setLineLayoutPadding:)]
        pub unsafe fn setLineLayoutPadding(&self, lineLayoutPadding: CGFloat);
        #[method(textAttachmentViewProviderClassForFileType:)]
        pub unsafe fn textAttachmentViewProviderClassForFileType(
            fileType: &NSString,
        ) -> Option<&Class>;
        #[method(registerTextAttachmentViewProviderClass:forFileType:)]
        pub unsafe fn registerTextAttachmentViewProviderClass_forFileType(
            textAttachmentViewProviderClass: &Class,
            fileType: &NSString,
        );
        #[method(allowsTextAttachmentView)]
        pub unsafe fn allowsTextAttachmentView(&self) -> bool;
        #[method(setAllowsTextAttachmentView:)]
        pub unsafe fn setAllowsTextAttachmentView(&self, allowsTextAttachmentView: bool);
        #[method(usesTextAttachmentView)]
        pub unsafe fn usesTextAttachmentView(&self) -> bool;
    }
);
extern_methods!(
    #[doc = "NSAttributedStringAttachmentConveniences"]
    unsafe impl NSAttributedString {
        #[method_id(attributedStringWithAttachment:)]
        pub unsafe fn attributedStringWithAttachment(
            attachment: &NSTextAttachment,
        ) -> Id<NSAttributedString, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSTextAttachmentViewProvider;
    unsafe impl ClassType for NSTextAttachmentViewProvider {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextAttachmentViewProvider {
        #[method_id(initWithTextAttachment:parentView:textLayoutManager:location:)]
        pub unsafe fn initWithTextAttachment_parentView_textLayoutManager_location(
            &self,
            textAttachment: &NSTextAttachment,
            parentView: Option<&NSView>,
            textLayoutManager: Option<&NSTextLayoutManager>,
            location: &NSTextLocation,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(new)]
        pub unsafe fn new() -> Id<Self, Shared>;
        #[method_id(textAttachment)]
        pub unsafe fn textAttachment(&self) -> Option<Id<NSTextAttachment, Shared>>;
        #[method_id(textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager, Shared>>;
        #[method_id(location)]
        pub unsafe fn location(&self) -> Id<NSTextLocation, Shared>;
        #[method_id(view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);
        #[method(loadView)]
        pub unsafe fn loadView(&self);
        #[method(tracksTextAttachmentViewBounds)]
        pub unsafe fn tracksTextAttachmentViewBounds(&self) -> bool;
        #[method(setTracksTextAttachmentViewBounds:)]
        pub unsafe fn setTracksTextAttachmentViewBounds(
            &self,
            tracksTextAttachmentViewBounds: bool,
        );
        #[method(attachmentBoundsForAttributes:location:textContainer:proposedLineFragment:position:)]
        pub unsafe fn attachmentBoundsForAttributes_location_textContainer_proposedLineFragment_position(
            &self,
            attributes: &NSDictionary<NSAttributedStringKey, Object>,
            location: &NSTextLocation,
            textContainer: Option<&NSTextContainer>,
            proposedLineFragment: CGRect,
            position: CGPoint,
        ) -> CGRect;
    }
);
extern_methods!(
    #[doc = "NSMutableAttributedStringAttachmentConveniences"]
    unsafe impl NSMutableAttributedString {
        #[method(updateAttachmentsFromPath:)]
        pub unsafe fn updateAttachmentsFromPath(&self, path: &NSString);
    }
);