use crate::Foundation::generated::NSXMLNode::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSXMLDTDNode;
    unsafe impl ClassType for NSXMLDTDNode {
        type Super = NSXMLNode;
    }
);
extern_methods!(
    unsafe impl NSXMLDTDNode {
        # [method_id (initWithXMLString :)]
        pub unsafe fn initWithXMLString(&self, string: &NSString) -> Option<Id<Self, Shared>>;
        # [method_id (initWithKind : options :)]
        pub unsafe fn initWithKind_options(
            &self,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method(DTDKind)]
        pub unsafe fn DTDKind(&self) -> NSXMLDTDNodeKind;
        # [method (setDTDKind :)]
        pub unsafe fn setDTDKind(&self, DTDKind: NSXMLDTDNodeKind);
        #[method(isExternal)]
        pub unsafe fn isExternal(&self) -> bool;
        #[method_id(publicID)]
        pub unsafe fn publicID(&self) -> Option<Id<NSString, Shared>>;
        # [method (setPublicID :)]
        pub unsafe fn setPublicID(&self, publicID: Option<&NSString>);
        #[method_id(systemID)]
        pub unsafe fn systemID(&self) -> Option<Id<NSString, Shared>>;
        # [method (setSystemID :)]
        pub unsafe fn setSystemID(&self, systemID: Option<&NSString>);
        #[method_id(notationName)]
        pub unsafe fn notationName(&self) -> Option<Id<NSString, Shared>>;
        # [method (setNotationName :)]
        pub unsafe fn setNotationName(&self, notationName: Option<&NSString>);
    }
);
