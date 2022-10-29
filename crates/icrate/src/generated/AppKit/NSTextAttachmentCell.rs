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
