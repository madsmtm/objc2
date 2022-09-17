use super::__exported::NSArray;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSFastEnumeration = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSEnumerator;
    unsafe impl ClassType for NSEnumerator {
        type Super = NSObject;
    }
);
impl NSEnumerator {
    pub unsafe fn nextObject(&self) -> ObjectType {
        msg_send![self, nextObject]
    }
}
#[doc = "NSExtendedEnumerator"]
impl NSEnumerator {
    pub unsafe fn allObjects(&self) -> TodoGenerics {
        msg_send![self, allObjects]
    }
}
