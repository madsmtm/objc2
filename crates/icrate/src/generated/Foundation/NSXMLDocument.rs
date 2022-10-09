use super::__exported::NSArray;
use super::__exported::NSData;
use super::__exported::NSDictionary;
use super::__exported::NSXMLDTD;
use crate::Foundation::generated::NSXMLNode::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSXMLDocument;
    unsafe impl ClassType for NSXMLDocument {
        type Super = NSXMLNode;
    }
);
extern_methods!(
    unsafe impl NSXMLDocument {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithXMLString:options:error:)]
        pub unsafe fn initWithXMLString_options_error(
            &self,
            string: &NSString,
            mask: NSXMLNodeOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        #[method_id(initWithContentsOfURL:options:error:)]
        pub unsafe fn initWithContentsOfURL_options_error(
            &self,
            url: &NSURL,
            mask: NSXMLNodeOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        #[method_id(initWithData:options:error:)]
        pub unsafe fn initWithData_options_error(
            &self,
            data: &NSData,
            mask: NSXMLNodeOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        #[method_id(initWithRootElement:)]
        pub unsafe fn initWithRootElement(
            &self,
            element: Option<&NSXMLElement>,
        ) -> Id<Self, Shared>;
        #[method(replacementClassForClass:)]
        pub unsafe fn replacementClassForClass(cls: &Class) -> &Class;
        #[method_id(characterEncoding)]
        pub unsafe fn characterEncoding(&self) -> Option<Id<NSString, Shared>>;
        #[method(setCharacterEncoding:)]
        pub unsafe fn setCharacterEncoding(&self, characterEncoding: Option<&NSString>);
        #[method_id(version)]
        pub unsafe fn version(&self) -> Option<Id<NSString, Shared>>;
        #[method(setVersion:)]
        pub unsafe fn setVersion(&self, version: Option<&NSString>);
        #[method(isStandalone)]
        pub unsafe fn isStandalone(&self) -> bool;
        #[method(setStandalone:)]
        pub unsafe fn setStandalone(&self, standalone: bool);
        #[method(documentContentKind)]
        pub unsafe fn documentContentKind(&self) -> NSXMLDocumentContentKind;
        #[method(setDocumentContentKind:)]
        pub unsafe fn setDocumentContentKind(&self, documentContentKind: NSXMLDocumentContentKind);
        #[method_id(MIMEType)]
        pub unsafe fn MIMEType(&self) -> Option<Id<NSString, Shared>>;
        #[method(setMIMEType:)]
        pub unsafe fn setMIMEType(&self, MIMEType: Option<&NSString>);
        #[method_id(DTD)]
        pub unsafe fn DTD(&self) -> Option<Id<NSXMLDTD, Shared>>;
        #[method(setDTD:)]
        pub unsafe fn setDTD(&self, DTD: Option<&NSXMLDTD>);
        #[method(setRootElement:)]
        pub unsafe fn setRootElement(&self, root: &NSXMLElement);
        #[method_id(rootElement)]
        pub unsafe fn rootElement(&self) -> Option<Id<NSXMLElement, Shared>>;
        #[method(insertChild:atIndex:)]
        pub unsafe fn insertChild_atIndex(&self, child: &NSXMLNode, index: NSUInteger);
        #[method(insertChildren:atIndex:)]
        pub unsafe fn insertChildren_atIndex(
            &self,
            children: &NSArray<NSXMLNode>,
            index: NSUInteger,
        );
        #[method(removeChildAtIndex:)]
        pub unsafe fn removeChildAtIndex(&self, index: NSUInteger);
        #[method(setChildren:)]
        pub unsafe fn setChildren(&self, children: Option<&NSArray<NSXMLNode>>);
        #[method(addChild:)]
        pub unsafe fn addChild(&self, child: &NSXMLNode);
        #[method(replaceChildAtIndex:withNode:)]
        pub unsafe fn replaceChildAtIndex_withNode(&self, index: NSUInteger, node: &NSXMLNode);
        #[method_id(XMLData)]
        pub unsafe fn XMLData(&self) -> Id<NSData, Shared>;
        #[method_id(XMLDataWithOptions:)]
        pub unsafe fn XMLDataWithOptions(&self, options: NSXMLNodeOptions) -> Id<NSData, Shared>;
        #[method_id(objectByApplyingXSLT:arguments:error:)]
        pub unsafe fn objectByApplyingXSLT_arguments_error(
            &self,
            xslt: &NSData,
            arguments: Option<&NSDictionary<NSString, NSString>>,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;
        #[method_id(objectByApplyingXSLTString:arguments:error:)]
        pub unsafe fn objectByApplyingXSLTString_arguments_error(
            &self,
            xslt: &NSString,
            arguments: Option<&NSDictionary<NSString, NSString>>,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;
        #[method_id(objectByApplyingXSLTAtURL:arguments:error:)]
        pub unsafe fn objectByApplyingXSLTAtURL_arguments_error(
            &self,
            xsltURL: &NSURL,
            argument: Option<&NSDictionary<NSString, NSString>>,
        ) -> Result<Id<Object, Shared>, Id<NSError, Shared>>;
        #[method(validateAndReturnError:)]
        pub unsafe fn validateAndReturnError(&self) -> Result<(), Id<NSError, Shared>>;
    }
);
