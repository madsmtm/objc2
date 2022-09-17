use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSGarbageCollector;
    unsafe impl ClassType for NSGarbageCollector {
        type Super = NSObject;
    }
);
impl NSGarbageCollector {
    pub unsafe fn defaultCollector() -> Id<Object, Shared> {
        msg_send_id![Self::class(), defaultCollector]
    }
    pub unsafe fn isCollecting(&self) -> bool {
        msg_send![self, isCollecting]
    }
    pub unsafe fn disable(&self) {
        msg_send![self, disable]
    }
    pub unsafe fn enable(&self) {
        msg_send![self, enable]
    }
    pub unsafe fn isEnabled(&self) -> bool {
        msg_send![self, isEnabled]
    }
    pub unsafe fn collectIfNeeded(&self) {
        msg_send![self, collectIfNeeded]
    }
    pub unsafe fn collectExhaustively(&self) {
        msg_send![self, collectExhaustively]
    }
    pub unsafe fn disableCollectorForPointer(&self, ptr: NonNull<c_void>) {
        msg_send![self, disableCollectorForPointer: ptr]
    }
    pub unsafe fn enableCollectorForPointer(&self, ptr: NonNull<c_void>) {
        msg_send![self, enableCollectorForPointer: ptr]
    }
    pub unsafe fn zone(&self) -> NonNull<NSZone> {
        msg_send![self, zone]
    }
}
