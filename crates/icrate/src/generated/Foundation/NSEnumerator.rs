use super::__exported::NSArray;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSFastEnumeration = NSObject;
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSEnumerator<ObjectType: Message>;
    unsafe impl<ObjectType: Message> ClassType for NSEnumerator<ObjectType> {
        type Super = NSObject;
    }
);
impl<ObjectType: Message> NSEnumerator<ObjectType> {
    pub unsafe fn nextObject(&self) -> Option<Id<ObjectType, Shared>> {
        msg_send_id![self, nextObject]
    }
}
#[doc = "NSExtendedEnumerator"]
impl<ObjectType: Message> NSEnumerator<ObjectType> {
    pub unsafe fn allObjects(&self) -> Id<NSArray<ObjectType>, Shared> {
        msg_send_id![self, allObjects]
    }
}
