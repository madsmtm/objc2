use super::NSArray;
use super::NSConnection;
use super::NSPort;
use crate::Foundation::generated::NSCoder::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPortCoder;
    unsafe impl ClassType for NSPortCoder {
        type Super = NSCoder;
    }
);
impl NSPortCoder {
    pub unsafe fn isBycopy(&self) -> bool {
        msg_send![self, isBycopy]
    }
    pub unsafe fn isByref(&self) -> bool {
        msg_send![self, isByref]
    }
    pub unsafe fn encodePortObject(&self, aport: &NSPort) {
        msg_send![self, encodePortObject: aport]
    }
    pub unsafe fn decodePortObject(&self) -> Option<Id<NSPort, Shared>> {
        msg_send_id![self, decodePortObject]
    }
    pub unsafe fn connection(&self) -> Option<Id<NSConnection, Shared>> {
        msg_send_id![self, connection]
    }
    pub unsafe fn portCoderWithReceivePort_sendPort_components(
        rcvPort: Option<&NSPort>,
        sndPort: Option<&NSPort>,
        comps: Option<&NSArray>,
    ) -> Id<Object, Shared> {
        msg_send_id![
            Self::class(),
            portCoderWithReceivePort: rcvPort,
            sendPort: sndPort,
            components: comps
        ]
    }
    pub unsafe fn initWithReceivePort_sendPort_components(
        &self,
        rcvPort: Option<&NSPort>,
        sndPort: Option<&NSPort>,
        comps: Option<&NSArray>,
    ) -> Id<Object, Shared> {
        msg_send_id![
            self,
            initWithReceivePort: rcvPort,
            sendPort: sndPort,
            components: comps
        ]
    }
    pub unsafe fn dispatch(&self) {
        msg_send![self, dispatch]
    }
}
#[doc = "NSDistributedObjects"]
impl NSObject {
    pub unsafe fn replacementObjectForPortCoder(
        &self,
        coder: &NSPortCoder,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, replacementObjectForPortCoder: coder]
    }
    pub unsafe fn classForPortCoder(&self) -> &Class {
        msg_send![self, classForPortCoder]
    }
}
