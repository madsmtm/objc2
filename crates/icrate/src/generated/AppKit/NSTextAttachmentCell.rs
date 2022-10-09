use super::__exported::NSLayoutManager;
use super::__exported::NSTextAttachment;
use super::__exported::NSTextContainer;
use crate::AppKit::generated::NSCell::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSTextAttachmentCell = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSTextAttachmentCell;
    unsafe impl ClassType for NSTextAttachmentCell {
        type Super = NSCell;
    }
);
extern_methods!(
    unsafe impl NSTextAttachmentCell {}
);
