//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

pub type NSXMLDTDNodeKind = NSUInteger;
pub const NSXMLEntityGeneralKind: NSXMLDTDNodeKind = 1;
pub const NSXMLEntityParsedKind: NSXMLDTDNodeKind = 2;
pub const NSXMLEntityUnparsedKind: NSXMLDTDNodeKind = 3;
pub const NSXMLEntityParameterKind: NSXMLDTDNodeKind = 4;
pub const NSXMLEntityPredefined: NSXMLDTDNodeKind = 5;
pub const NSXMLAttributeCDATAKind: NSXMLDTDNodeKind = 6;
pub const NSXMLAttributeIDKind: NSXMLDTDNodeKind = 7;
pub const NSXMLAttributeIDRefKind: NSXMLDTDNodeKind = 8;
pub const NSXMLAttributeIDRefsKind: NSXMLDTDNodeKind = 9;
pub const NSXMLAttributeEntityKind: NSXMLDTDNodeKind = 10;
pub const NSXMLAttributeEntitiesKind: NSXMLDTDNodeKind = 11;
pub const NSXMLAttributeNMTokenKind: NSXMLDTDNodeKind = 12;
pub const NSXMLAttributeNMTokensKind: NSXMLDTDNodeKind = 13;
pub const NSXMLAttributeEnumerationKind: NSXMLDTDNodeKind = 14;
pub const NSXMLAttributeNotationKind: NSXMLDTDNodeKind = 15;
pub const NSXMLElementDeclarationUndefinedKind: NSXMLDTDNodeKind = 16;
pub const NSXMLElementDeclarationEmptyKind: NSXMLDTDNodeKind = 17;
pub const NSXMLElementDeclarationAnyKind: NSXMLDTDNodeKind = 18;
pub const NSXMLElementDeclarationMixedKind: NSXMLDTDNodeKind = 19;
pub const NSXMLElementDeclarationElementKind: NSXMLDTDNodeKind = 20;

extern_class!(
    #[derive(Debug)]
    pub struct NSXMLDTDNode;

    unsafe impl ClassType for NSXMLDTDNode {
        type Super = NSXMLNode;
    }
);

extern_methods!(
    unsafe impl NSXMLDTDNode {
        #[method_id(initWithXMLString:)]
        pub unsafe fn initWithXMLString(&self, string: &NSString) -> Option<Id<Self, Shared>>;

        #[method_id(initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            &self,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self, Shared>;

        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;

        #[method(DTDKind)]
        pub unsafe fn DTDKind(&self) -> NSXMLDTDNodeKind;

        #[method(setDTDKind:)]
        pub unsafe fn setDTDKind(&self, DTDKind: NSXMLDTDNodeKind);

        #[method(isExternal)]
        pub unsafe fn isExternal(&self) -> bool;

        #[method_id(publicID)]
        pub unsafe fn publicID(&self) -> Option<Id<NSString, Shared>>;

        #[method(setPublicID:)]
        pub unsafe fn setPublicID(&self, publicID: Option<&NSString>);

        #[method_id(systemID)]
        pub unsafe fn systemID(&self) -> Option<Id<NSString, Shared>>;

        #[method(setSystemID:)]
        pub unsafe fn setSystemID(&self, systemID: Option<&NSString>);

        #[method_id(notationName)]
        pub unsafe fn notationName(&self) -> Option<Id<NSString, Shared>>;

        #[method(setNotationName:)]
        pub unsafe fn setNotationName(&self, notationName: Option<&NSString>);
    }
);
