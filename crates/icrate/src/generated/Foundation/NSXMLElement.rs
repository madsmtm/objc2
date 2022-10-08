use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSEnumerator;
use super::__exported::NSMutableArray;
use super::__exported::NSString;
use crate::Foundation::generated::NSXMLNode::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSXMLElement;
    unsafe impl ClassType for NSXMLElement {
        type Super = NSXMLNode;
    }
);
extern_methods!(
    unsafe impl NSXMLElement {
        # [method_id (initWithName :)]
        pub unsafe fn initWithName(&self, name: &NSString) -> Id<Self, Shared>;
        # [method_id (initWithName : URI :)]
        pub unsafe fn initWithName_URI(
            &self,
            name: &NSString,
            URI: Option<&NSString>,
        ) -> Id<Self, Shared>;
        # [method_id (initWithName : stringValue :)]
        pub unsafe fn initWithName_stringValue(
            &self,
            name: &NSString,
            string: Option<&NSString>,
        ) -> Id<Self, Shared>;
        # [method_id (initWithXMLString : error :)]
        pub unsafe fn initWithXMLString_error(
            &self,
            string: &NSString,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        # [method_id (initWithKind : options :)]
        pub unsafe fn initWithKind_options(
            &self,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self, Shared>;
        # [method_id (elementsForName :)]
        pub unsafe fn elementsForName(&self, name: &NSString) -> Id<NSArray<NSXMLElement>, Shared>;
        # [method_id (elementsForLocalName : URI :)]
        pub unsafe fn elementsForLocalName_URI(
            &self,
            localName: &NSString,
            URI: Option<&NSString>,
        ) -> Id<NSArray<NSXMLElement>, Shared>;
        # [method (addAttribute :)]
        pub unsafe fn addAttribute(&self, attribute: &NSXMLNode);
        # [method (removeAttributeForName :)]
        pub unsafe fn removeAttributeForName(&self, name: &NSString);
        #[method_id(attributes)]
        pub unsafe fn attributes(&self) -> Option<Id<NSArray<NSXMLNode>, Shared>>;
        # [method (setAttributes :)]
        pub unsafe fn setAttributes(&self, attributes: Option<&NSArray<NSXMLNode>>);
        # [method (setAttributesWithDictionary :)]
        pub unsafe fn setAttributesWithDictionary(
            &self,
            attributes: &NSDictionary<NSString, NSString>,
        );
        # [method_id (attributeForName :)]
        pub unsafe fn attributeForName(&self, name: &NSString) -> Option<Id<NSXMLNode, Shared>>;
        # [method_id (attributeForLocalName : URI :)]
        pub unsafe fn attributeForLocalName_URI(
            &self,
            localName: &NSString,
            URI: Option<&NSString>,
        ) -> Option<Id<NSXMLNode, Shared>>;
        # [method (addNamespace :)]
        pub unsafe fn addNamespace(&self, aNamespace: &NSXMLNode);
        # [method (removeNamespaceForPrefix :)]
        pub unsafe fn removeNamespaceForPrefix(&self, name: &NSString);
        #[method_id(namespaces)]
        pub unsafe fn namespaces(&self) -> Option<Id<NSArray<NSXMLNode>, Shared>>;
        # [method (setNamespaces :)]
        pub unsafe fn setNamespaces(&self, namespaces: Option<&NSArray<NSXMLNode>>);
        # [method_id (namespaceForPrefix :)]
        pub unsafe fn namespaceForPrefix(&self, name: &NSString) -> Option<Id<NSXMLNode, Shared>>;
        # [method_id (resolveNamespaceForName :)]
        pub unsafe fn resolveNamespaceForName(
            &self,
            name: &NSString,
        ) -> Option<Id<NSXMLNode, Shared>>;
        # [method_id (resolvePrefixForNamespaceURI :)]
        pub unsafe fn resolvePrefixForNamespaceURI(
            &self,
            namespaceURI: &NSString,
        ) -> Option<Id<NSString, Shared>>;
        # [method (insertChild : atIndex :)]
        pub unsafe fn insertChild_atIndex(&self, child: &NSXMLNode, index: NSUInteger);
        # [method (insertChildren : atIndex :)]
        pub unsafe fn insertChildren_atIndex(
            &self,
            children: &NSArray<NSXMLNode>,
            index: NSUInteger,
        );
        # [method (removeChildAtIndex :)]
        pub unsafe fn removeChildAtIndex(&self, index: NSUInteger);
        # [method (setChildren :)]
        pub unsafe fn setChildren(&self, children: Option<&NSArray<NSXMLNode>>);
        # [method (addChild :)]
        pub unsafe fn addChild(&self, child: &NSXMLNode);
        # [method (replaceChildAtIndex : withNode :)]
        pub unsafe fn replaceChildAtIndex_withNode(&self, index: NSUInteger, node: &NSXMLNode);
        # [method (normalizeAdjacentTextNodesPreservingCDATA :)]
        pub unsafe fn normalizeAdjacentTextNodesPreservingCDATA(&self, preserve: bool);
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSXMLElement {
        # [method (setAttributesAsDictionary :)]
        pub unsafe fn setAttributesAsDictionary(&self, attributes: &NSDictionary);
    }
);
