use super::__exported::NSArray;
use super::__exported::NSData;
use super::__exported::NSDictionary;
use super::__exported::NSXMLDTD;
use crate::Foundation::generated::NSXMLNode::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSXMLDocument;
    unsafe impl ClassType for NSXMLDocument {
        type Super = NSXMLNode;
    }
);
impl NSXMLDocument {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithXMLString_options_error(
        &self,
        string: &NSString,
        mask: NSXMLNodeOptions,
    ) -> Result<Id<Self, Shared>, Id<NSError, Shared>> {
        msg_send_id![self, initWithXMLString: string, options: mask, error: _]
    }
    pub unsafe fn initWithContentsOfURL_options_error(
        &self,
        url: &NSURL,
        mask: NSXMLNodeOptions,
    ) -> Result<Id<Self, Shared>, Id<NSError, Shared>> {
        msg_send_id![self, initWithContentsOfURL: url, options: mask, error: _]
    }
    pub unsafe fn initWithData_options_error(
        &self,
        data: &NSData,
        mask: NSXMLNodeOptions,
    ) -> Result<Id<Self, Shared>, Id<NSError, Shared>> {
        msg_send_id![self, initWithData: data, options: mask, error: _]
    }
    pub unsafe fn initWithRootElement(&self, element: Option<&NSXMLElement>) -> Id<Self, Shared> {
        msg_send_id![self, initWithRootElement: element]
    }
    pub unsafe fn replacementClassForClass(cls: &Class) -> &Class {
        msg_send![Self::class(), replacementClassForClass: cls]
    }
    pub unsafe fn characterEncoding(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, characterEncoding]
    }
    pub unsafe fn setCharacterEncoding(&self, characterEncoding: Option<&NSString>) {
        msg_send![self, setCharacterEncoding: characterEncoding]
    }
    pub unsafe fn version(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, version]
    }
    pub unsafe fn setVersion(&self, version: Option<&NSString>) {
        msg_send![self, setVersion: version]
    }
    pub unsafe fn isStandalone(&self) -> bool {
        msg_send![self, isStandalone]
    }
    pub unsafe fn setStandalone(&self, standalone: bool) {
        msg_send![self, setStandalone: standalone]
    }
    pub unsafe fn documentContentKind(&self) -> NSXMLDocumentContentKind {
        msg_send![self, documentContentKind]
    }
    pub unsafe fn setDocumentContentKind(&self, documentContentKind: NSXMLDocumentContentKind) {
        msg_send![self, setDocumentContentKind: documentContentKind]
    }
    pub unsafe fn MIMEType(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, MIMEType]
    }
    pub unsafe fn setMIMEType(&self, MIMEType: Option<&NSString>) {
        msg_send![self, setMIMEType: MIMEType]
    }
    pub unsafe fn DTD(&self) -> Option<Id<NSXMLDTD, Shared>> {
        msg_send_id![self, DTD]
    }
    pub unsafe fn setDTD(&self, DTD: Option<&NSXMLDTD>) {
        msg_send![self, setDTD: DTD]
    }
    pub unsafe fn setRootElement(&self, root: &NSXMLElement) {
        msg_send![self, setRootElement: root]
    }
    pub unsafe fn rootElement(&self) -> Option<Id<NSXMLElement, Shared>> {
        msg_send_id![self, rootElement]
    }
    pub unsafe fn insertChild_atIndex(&self, child: &NSXMLNode, index: NSUInteger) {
        msg_send![self, insertChild: child, atIndex: index]
    }
    pub unsafe fn insertChildren_atIndex(&self, children: &NSArray<NSXMLNode>, index: NSUInteger) {
        msg_send![self, insertChildren: children, atIndex: index]
    }
    pub unsafe fn removeChildAtIndex(&self, index: NSUInteger) {
        msg_send![self, removeChildAtIndex: index]
    }
    pub unsafe fn setChildren(&self, children: Option<&NSArray<NSXMLNode>>) {
        msg_send![self, setChildren: children]
    }
    pub unsafe fn addChild(&self, child: &NSXMLNode) {
        msg_send![self, addChild: child]
    }
    pub unsafe fn replaceChildAtIndex_withNode(&self, index: NSUInteger, node: &NSXMLNode) {
        msg_send![self, replaceChildAtIndex: index, withNode: node]
    }
    pub unsafe fn XMLData(&self) -> Id<NSData, Shared> {
        msg_send_id![self, XMLData]
    }
    pub unsafe fn XMLDataWithOptions(&self, options: NSXMLNodeOptions) -> Id<NSData, Shared> {
        msg_send_id![self, XMLDataWithOptions: options]
    }
    pub unsafe fn objectByApplyingXSLT_arguments_error(
        &self,
        xslt: &NSData,
        arguments: Option<&NSDictionary<NSString, NSString>>,
    ) -> Result<Id<Object, Shared>, Id<NSError, Shared>> {
        msg_send_id![
            self,
            objectByApplyingXSLT: xslt,
            arguments: arguments,
            error: _
        ]
    }
    pub unsafe fn objectByApplyingXSLTString_arguments_error(
        &self,
        xslt: &NSString,
        arguments: Option<&NSDictionary<NSString, NSString>>,
    ) -> Result<Id<Object, Shared>, Id<NSError, Shared>> {
        msg_send_id![
            self,
            objectByApplyingXSLTString: xslt,
            arguments: arguments,
            error: _
        ]
    }
    pub unsafe fn objectByApplyingXSLTAtURL_arguments_error(
        &self,
        xsltURL: &NSURL,
        argument: Option<&NSDictionary<NSString, NSString>>,
    ) -> Result<Id<Object, Shared>, Id<NSError, Shared>> {
        msg_send_id![
            self,
            objectByApplyingXSLTAtURL: xsltURL,
            arguments: argument,
            error: _
        ]
    }
    pub unsafe fn validateAndReturnError(&self) -> Result<(), Id<NSError, Shared>> {
        msg_send![self, validateAndReturnError: _]
    }
}
