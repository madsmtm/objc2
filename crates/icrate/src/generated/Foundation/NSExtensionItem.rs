use crate::Foundation::generated::Foundation::*;
use crate::Foundation::generated::NSItemProvider::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSExtensionItem;
    unsafe impl ClassType for NSExtensionItem {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSExtensionItem {
        #[method_id(attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Option<Id<NSAttributedString, Shared>>;
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributedTitle: Option<&NSAttributedString>);
        #[method_id(attributedContentText)]
        pub unsafe fn attributedContentText(&self) -> Option<Id<NSAttributedString, Shared>>;
        #[method(setAttributedContentText:)]
        pub unsafe fn setAttributedContentText(
            &self,
            attributedContentText: Option<&NSAttributedString>,
        );
        #[method_id(attachments)]
        pub unsafe fn attachments(&self) -> Option<Id<NSArray<NSItemProvider>, Shared>>;
        #[method(setAttachments:)]
        pub unsafe fn setAttachments(&self, attachments: Option<&NSArray<NSItemProvider>>);
        #[method_id(userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>>;
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, userInfo: Option<&NSDictionary>);
    }
);
