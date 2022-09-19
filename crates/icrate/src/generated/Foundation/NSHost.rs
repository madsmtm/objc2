use super::__exported::NSArray;
use super::__exported::NSMutableArray;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSHost;
    unsafe impl ClassType for NSHost {
        type Super = NSObject;
    }
);
impl NSHost {
    pub unsafe fn currentHost() -> Id<Self, Shared> {
        msg_send_id![Self::class(), currentHost]
    }
    pub unsafe fn hostWithName(name: Option<&NSString>) -> Id<Self, Shared> {
        msg_send_id![Self::class(), hostWithName: name]
    }
    pub unsafe fn hostWithAddress(address: &NSString) -> Id<Self, Shared> {
        msg_send_id![Self::class(), hostWithAddress: address]
    }
    pub unsafe fn isEqualToHost(&self, aHost: &NSHost) -> bool {
        msg_send![self, isEqualToHost: aHost]
    }
    pub unsafe fn setHostCacheEnabled(flag: bool) {
        msg_send![Self::class(), setHostCacheEnabled: flag]
    }
    pub unsafe fn isHostCacheEnabled() -> bool {
        msg_send![Self::class(), isHostCacheEnabled]
    }
    pub unsafe fn flushHostCache() {
        msg_send![Self::class(), flushHostCache]
    }
    pub unsafe fn name(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, name]
    }
    pub unsafe fn names(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, names]
    }
    pub unsafe fn address(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, address]
    }
    pub unsafe fn addresses(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, addresses]
    }
    pub unsafe fn localizedName(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, localizedName]
    }
}
