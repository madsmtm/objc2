use super::__exported::NSDate;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDistributedLock;
    unsafe impl ClassType for NSDistributedLock {
        type Super = NSObject;
    }
);
impl NSDistributedLock {
    pub unsafe fn lockWithPath(path: &NSString) -> Option<Id<NSDistributedLock, Shared>> {
        msg_send_id![Self::class(), lockWithPath: path]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithPath(&self, path: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithPath: path]
    }
    pub unsafe fn tryLock(&self) -> bool {
        msg_send![self, tryLock]
    }
    pub unsafe fn unlock(&self) {
        msg_send![self, unlock]
    }
    pub unsafe fn breakLock(&self) {
        msg_send![self, breakLock]
    }
    pub unsafe fn lockDate(&self) -> Id<NSDate, Shared> {
        msg_send_id![self, lockDate]
    }
}
