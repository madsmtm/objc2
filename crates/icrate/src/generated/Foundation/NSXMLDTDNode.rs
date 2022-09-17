#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSXMLDTDNode;
    unsafe impl ClassType for NSXMLDTDNode {
        type Super = NSXMLNode;
    }
);
impl NSXMLDTDNode {
    pub unsafe fn initWithXMLString(&self, string: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithXMLString: string]
    }
    pub unsafe fn initWithKind_options(
        &self,
        kind: NSXMLNodeKind,
        options: NSXMLNodeOptions,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithKind: kind, options: options]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn DTDKind(&self) -> NSXMLDTDNodeKind {
        msg_send![self, DTDKind]
    }
    pub unsafe fn setDTDKind(&self, DTDKind: NSXMLDTDNodeKind) {
        msg_send![self, setDTDKind: DTDKind]
    }
    pub unsafe fn isExternal(&self) -> bool {
        msg_send![self, isExternal]
    }
    pub unsafe fn publicID(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, publicID]
    }
    pub unsafe fn setPublicID(&self, publicID: Option<&NSString>) {
        msg_send![self, setPublicID: publicID]
    }
    pub unsafe fn systemID(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, systemID]
    }
    pub unsafe fn setSystemID(&self, systemID: Option<&NSString>) {
        msg_send![self, setSystemID: systemID]
    }
    pub unsafe fn notationName(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, notationName]
    }
    pub unsafe fn setNotationName(&self, notationName: Option<&NSString>) {
        msg_send![self, setNotationName: notationName]
    }
}
