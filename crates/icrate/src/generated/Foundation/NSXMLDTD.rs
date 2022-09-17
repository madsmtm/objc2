use super::NSArray;
use super::NSData;
use super::NSMutableDictionary;
use super::NSXMLDTDNode;
use crate::Foundation::generated::NSXMLNode::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSXMLDTD;
    unsafe impl ClassType for NSXMLDTD {
        type Super = NSXMLNode;
    }
);
impl NSXMLDTD {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithKind_options(
        &self,
        kind: NSXMLNodeKind,
        options: NSXMLNodeOptions,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithKind: kind, options: options]
    }
    pub unsafe fn initWithContentsOfURL_options_error(
        &self,
        url: &NSURL,
        mask: NSXMLNodeOptions,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithContentsOfURL: url,
            options: mask,
            error: error
        ]
    }
    pub unsafe fn initWithData_options_error(
        &self,
        data: &NSData,
        mask: NSXMLNodeOptions,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithData: data, options: mask, error: error]
    }
    pub unsafe fn insertChild_atIndex(&self, child: &NSXMLNode, index: NSUInteger) {
        msg_send![self, insertChild: child, atIndex: index]
    }
    pub unsafe fn insertChildren_atIndex(&self, children: TodoGenerics, index: NSUInteger) {
        msg_send![self, insertChildren: children, atIndex: index]
    }
    pub unsafe fn removeChildAtIndex(&self, index: NSUInteger) {
        msg_send![self, removeChildAtIndex: index]
    }
    pub unsafe fn setChildren(&self, children: TodoGenerics) {
        msg_send![self, setChildren: children]
    }
    pub unsafe fn addChild(&self, child: &NSXMLNode) {
        msg_send![self, addChild: child]
    }
    pub unsafe fn replaceChildAtIndex_withNode(&self, index: NSUInteger, node: &NSXMLNode) {
        msg_send![self, replaceChildAtIndex: index, withNode: node]
    }
    pub unsafe fn entityDeclarationForName(
        &self,
        name: &NSString,
    ) -> Option<Id<NSXMLDTDNode, Shared>> {
        msg_send_id![self, entityDeclarationForName: name]
    }
    pub unsafe fn notationDeclarationForName(
        &self,
        name: &NSString,
    ) -> Option<Id<NSXMLDTDNode, Shared>> {
        msg_send_id![self, notationDeclarationForName: name]
    }
    pub unsafe fn elementDeclarationForName(
        &self,
        name: &NSString,
    ) -> Option<Id<NSXMLDTDNode, Shared>> {
        msg_send_id![self, elementDeclarationForName: name]
    }
    pub unsafe fn attributeDeclarationForName_elementName(
        &self,
        name: &NSString,
        elementName: &NSString,
    ) -> Option<Id<NSXMLDTDNode, Shared>> {
        msg_send_id![
            self,
            attributeDeclarationForName: name,
            elementName: elementName
        ]
    }
    pub unsafe fn predefinedEntityDeclarationForName(
        name: &NSString,
    ) -> Option<Id<NSXMLDTDNode, Shared>> {
        msg_send_id![Self::class(), predefinedEntityDeclarationForName: name]
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
}
