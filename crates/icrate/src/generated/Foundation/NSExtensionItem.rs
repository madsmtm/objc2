#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSExtensionItem;
    unsafe impl ClassType for NSExtensionItem {
        type Super = NSObject;
    }
);
impl NSExtensionItem {
    pub unsafe fn attributedTitle(&self) -> Option<Id<NSAttributedString, Shared>> {
        msg_send_id![self, attributedTitle]
    }
    pub unsafe fn setAttributedTitle(&self, attributedTitle: Option<&NSAttributedString>) {
        msg_send![self, setAttributedTitle: attributedTitle]
    }
    pub unsafe fn attributedContentText(&self) -> Option<Id<NSAttributedString, Shared>> {
        msg_send_id![self, attributedContentText]
    }
    pub unsafe fn setAttributedContentText(
        &self,
        attributedContentText: Option<&NSAttributedString>,
    ) {
        msg_send![self, setAttributedContentText: attributedContentText]
    }
    pub unsafe fn attachments(&self) -> TodoGenerics {
        msg_send![self, attachments]
    }
    pub unsafe fn setAttachments(&self, attachments: TodoGenerics) {
        msg_send![self, setAttachments: attachments]
    }
    pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>> {
        msg_send_id![self, userInfo]
    }
    pub unsafe fn setUserInfo(&self, userInfo: Option<&NSDictionary>) {
        msg_send![self, setUserInfo: userInfo]
    }
}
