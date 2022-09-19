use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSEnumerator;
use super::__exported::NSMutableArray;
use super::__exported::NSString;
use crate::Foundation::generated::NSXMLNode::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSXMLElement;
    unsafe impl ClassType for NSXMLElement {
        type Super = NSXMLNode;
    }
);
impl NSXMLElement {
    pub unsafe fn initWithName(&self, name: &NSString) -> Id<Self, Shared> {
        msg_send_id![self, initWithName: name]
    }
    pub unsafe fn initWithName_URI(
        &self,
        name: &NSString,
        URI: Option<&NSString>,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithName: name, URI: URI]
    }
    pub unsafe fn initWithName_stringValue(
        &self,
        name: &NSString,
        string: Option<&NSString>,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithName: name, stringValue: string]
    }
    pub unsafe fn initWithXMLString_error(
        &self,
        string: &NSString,
        error: *mut *mut NSError,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithXMLString: string, error: error]
    }
    pub unsafe fn initWithKind_options(
        &self,
        kind: NSXMLNodeKind,
        options: NSXMLNodeOptions,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithKind: kind, options: options]
    }
    pub unsafe fn elementsForName(&self, name: &NSString) -> TodoGenerics {
        msg_send![self, elementsForName: name]
    }
    pub unsafe fn elementsForLocalName_URI(
        &self,
        localName: &NSString,
        URI: Option<&NSString>,
    ) -> TodoGenerics {
        msg_send![self, elementsForLocalName: localName, URI: URI]
    }
    pub unsafe fn addAttribute(&self, attribute: &NSXMLNode) {
        msg_send![self, addAttribute: attribute]
    }
    pub unsafe fn removeAttributeForName(&self, name: &NSString) {
        msg_send![self, removeAttributeForName: name]
    }
    pub unsafe fn setAttributesWithDictionary(&self, attributes: TodoGenerics) {
        msg_send![self, setAttributesWithDictionary: attributes]
    }
    pub unsafe fn attributeForName(&self, name: &NSString) -> Option<Id<NSXMLNode, Shared>> {
        msg_send_id![self, attributeForName: name]
    }
    pub unsafe fn attributeForLocalName_URI(
        &self,
        localName: &NSString,
        URI: Option<&NSString>,
    ) -> Option<Id<NSXMLNode, Shared>> {
        msg_send_id![self, attributeForLocalName: localName, URI: URI]
    }
    pub unsafe fn addNamespace(&self, aNamespace: &NSXMLNode) {
        msg_send![self, addNamespace: aNamespace]
    }
    pub unsafe fn removeNamespaceForPrefix(&self, name: &NSString) {
        msg_send![self, removeNamespaceForPrefix: name]
    }
    pub unsafe fn namespaceForPrefix(&self, name: &NSString) -> Option<Id<NSXMLNode, Shared>> {
        msg_send_id![self, namespaceForPrefix: name]
    }
    pub unsafe fn resolveNamespaceForName(&self, name: &NSString) -> Option<Id<NSXMLNode, Shared>> {
        msg_send_id![self, resolveNamespaceForName: name]
    }
    pub unsafe fn resolvePrefixForNamespaceURI(
        &self,
        namespaceURI: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, resolvePrefixForNamespaceURI: namespaceURI]
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
    pub unsafe fn normalizeAdjacentTextNodesPreservingCDATA(&self, preserve: bool) {
        msg_send![self, normalizeAdjacentTextNodesPreservingCDATA: preserve]
    }
    pub unsafe fn attributes(&self) -> TodoGenerics {
        msg_send![self, attributes]
    }
    pub unsafe fn setAttributes(&self, attributes: TodoGenerics) {
        msg_send![self, setAttributes: attributes]
    }
    pub unsafe fn namespaces(&self) -> TodoGenerics {
        msg_send![self, namespaces]
    }
    pub unsafe fn setNamespaces(&self, namespaces: TodoGenerics) {
        msg_send![self, setNamespaces: namespaces]
    }
}
#[doc = "NSDeprecated"]
impl NSXMLElement {
    pub unsafe fn setAttributesAsDictionary(&self, attributes: &NSDictionary) {
        msg_send![self, setAttributesAsDictionary: attributes]
    }
}
