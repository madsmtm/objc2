use crate::uuid::generated::uuid::*;
use crate::CoreFoundation::generated::CFUUID::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUUID;
    unsafe impl ClassType for NSUUID {
        type Super = NSObject;
    }
);
impl NSUUID {
    pub unsafe fn UUID() -> Id<Self, Shared> {
        msg_send_id![Self::class(), UUID]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithUUIDString(&self, string: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithUUIDString: string]
    }
    pub unsafe fn initWithUUIDBytes(&self, bytes: uuid_t) -> Id<Self, Shared> {
        msg_send_id![self, initWithUUIDBytes: bytes]
    }
    pub unsafe fn getUUIDBytes(&self, uuid: uuid_t) {
        msg_send![self, getUUIDBytes: uuid]
    }
    pub unsafe fn compare(&self, otherUUID: &NSUUID) -> NSComparisonResult {
        msg_send![self, compare: otherUUID]
    }
    pub unsafe fn UUIDString(&self) -> Id<NSString, Shared> {
        msg_send_id![self, UUIDString]
    }
}
