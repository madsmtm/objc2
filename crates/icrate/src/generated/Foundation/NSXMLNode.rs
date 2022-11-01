//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSXMLNodeKind = NSUInteger;
pub const NSXMLInvalidKind: NSXMLNodeKind = 0;
pub const NSXMLDocumentKind: NSXMLNodeKind = 1;
pub const NSXMLElementKind: NSXMLNodeKind = 2;
pub const NSXMLAttributeKind: NSXMLNodeKind = 3;
pub const NSXMLNamespaceKind: NSXMLNodeKind = 4;
pub const NSXMLProcessingInstructionKind: NSXMLNodeKind = 5;
pub const NSXMLCommentKind: NSXMLNodeKind = 6;
pub const NSXMLTextKind: NSXMLNodeKind = 7;
pub const NSXMLDTDKind: NSXMLNodeKind = 8;
pub const NSXMLEntityDeclarationKind: NSXMLNodeKind = 9;
pub const NSXMLAttributeDeclarationKind: NSXMLNodeKind = 10;
pub const NSXMLElementDeclarationKind: NSXMLNodeKind = 11;
pub const NSXMLNotationDeclarationKind: NSXMLNodeKind = 12;

extern_class!(
    #[derive(Debug)]
    pub struct NSXMLNode;

    unsafe impl ClassType for NSXMLNode {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSXMLNode {
        #[method_id(init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(initWithKind:)]
        pub unsafe fn initWithKind(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
        ) -> Id<Self, Shared>;

        #[method_id(initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            this: Option<Allocated<Self>>,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self, Shared>;

        #[method_id(document)]
        pub unsafe fn document() -> Id<Object, Shared>;

        #[method_id(documentWithRootElement:)]
        pub unsafe fn documentWithRootElement(element: &NSXMLElement) -> Id<Object, Shared>;

        #[method_id(elementWithName:)]
        pub unsafe fn elementWithName(name: &NSString) -> Id<Object, Shared>;

        #[method_id(elementWithName:URI:)]
        pub unsafe fn elementWithName_URI(name: &NSString, URI: &NSString) -> Id<Object, Shared>;

        #[method_id(elementWithName:stringValue:)]
        pub unsafe fn elementWithName_stringValue(
            name: &NSString,
            string: &NSString,
        ) -> Id<Object, Shared>;

        #[method_id(elementWithName:children:attributes:)]
        pub unsafe fn elementWithName_children_attributes(
            name: &NSString,
            children: Option<&NSArray<NSXMLNode>>,
            attributes: Option<&NSArray<NSXMLNode>>,
        ) -> Id<Object, Shared>;

        #[method_id(attributeWithName:stringValue:)]
        pub unsafe fn attributeWithName_stringValue(
            name: &NSString,
            stringValue: &NSString,
        ) -> Id<Object, Shared>;

        #[method_id(attributeWithName:URI:stringValue:)]
        pub unsafe fn attributeWithName_URI_stringValue(
            name: &NSString,
            URI: &NSString,
            stringValue: &NSString,
        ) -> Id<Object, Shared>;

        #[method_id(namespaceWithName:stringValue:)]
        pub unsafe fn namespaceWithName_stringValue(
            name: &NSString,
            stringValue: &NSString,
        ) -> Id<Object, Shared>;

        #[method_id(processingInstructionWithName:stringValue:)]
        pub unsafe fn processingInstructionWithName_stringValue(
            name: &NSString,
            stringValue: &NSString,
        ) -> Id<Object, Shared>;

        #[method_id(commentWithStringValue:)]
        pub unsafe fn commentWithStringValue(stringValue: &NSString) -> Id<Object, Shared>;

        #[method_id(textWithStringValue:)]
        pub unsafe fn textWithStringValue(stringValue: &NSString) -> Id<Object, Shared>;

        #[method_id(DTDNodeWithXMLString:)]
        pub unsafe fn DTDNodeWithXMLString(string: &NSString) -> Option<Id<Object, Shared>>;

        #[method(kind)]
        pub unsafe fn kind(&self) -> NSXMLNodeKind;

        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[method_id(objectValue)]
        pub unsafe fn objectValue(&self) -> Option<Id<Object, Shared>>;

        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, objectValue: Option<&Object>);

        #[method_id(stringValue)]
        pub unsafe fn stringValue(&self) -> Option<Id<NSString, Shared>>;

        #[method(setStringValue:)]
        pub unsafe fn setStringValue(&self, stringValue: Option<&NSString>);

        #[method(setStringValue:resolvingEntities:)]
        pub unsafe fn setStringValue_resolvingEntities(&self, string: &NSString, resolve: bool);

        #[method(index)]
        pub unsafe fn index(&self) -> NSUInteger;

        #[method(level)]
        pub unsafe fn level(&self) -> NSUInteger;

        #[method_id(rootDocument)]
        pub unsafe fn rootDocument(&self) -> Option<Id<NSXMLDocument, Shared>>;

        #[method_id(parent)]
        pub unsafe fn parent(&self) -> Option<Id<NSXMLNode, Shared>>;

        #[method(childCount)]
        pub unsafe fn childCount(&self) -> NSUInteger;

        #[method_id(children)]
        pub unsafe fn children(&self) -> Option<Id<NSArray<NSXMLNode>, Shared>>;

        #[method_id(childAtIndex:)]
        pub unsafe fn childAtIndex(&self, index: NSUInteger) -> Option<Id<NSXMLNode, Shared>>;

        #[method_id(previousSibling)]
        pub unsafe fn previousSibling(&self) -> Option<Id<NSXMLNode, Shared>>;

        #[method_id(nextSibling)]
        pub unsafe fn nextSibling(&self) -> Option<Id<NSXMLNode, Shared>>;

        #[method_id(previousNode)]
        pub unsafe fn previousNode(&self) -> Option<Id<NSXMLNode, Shared>>;

        #[method_id(nextNode)]
        pub unsafe fn nextNode(&self) -> Option<Id<NSXMLNode, Shared>>;

        #[method(detach)]
        pub unsafe fn detach(&self);

        #[method_id(XPath)]
        pub unsafe fn XPath(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(localName)]
        pub unsafe fn localName(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(prefix)]
        pub unsafe fn prefix(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(URI)]
        pub unsafe fn URI(&self) -> Option<Id<NSString, Shared>>;

        #[method(setURI:)]
        pub unsafe fn setURI(&self, URI: Option<&NSString>);

        #[method_id(localNameForName:)]
        pub unsafe fn localNameForName(name: &NSString) -> Id<NSString, Shared>;

        #[method_id(prefixForName:)]
        pub unsafe fn prefixForName(name: &NSString) -> Option<Id<NSString, Shared>>;

        #[method_id(predefinedNamespaceForPrefix:)]
        pub unsafe fn predefinedNamespaceForPrefix(
            name: &NSString,
        ) -> Option<Id<NSXMLNode, Shared>>;

        #[method_id(description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;

        #[method_id(XMLString)]
        pub unsafe fn XMLString(&self) -> Id<NSString, Shared>;

        #[method_id(XMLStringWithOptions:)]
        pub unsafe fn XMLStringWithOptions(
            &self,
            options: NSXMLNodeOptions,
        ) -> Id<NSString, Shared>;

        #[method_id(canonicalXMLStringPreservingComments:)]
        pub unsafe fn canonicalXMLStringPreservingComments(
            &self,
            comments: bool,
        ) -> Id<NSString, Shared>;

        #[method_id(nodesForXPath:error:)]
        pub unsafe fn nodesForXPath_error(
            &self,
            xpath: &NSString,
        ) -> Result<Id<NSArray<NSXMLNode>, Shared>, Id<NSError, Shared>>;

        #[method_id(objectsForXQuery:constants:error:)]
        pub unsafe fn objectsForXQuery_constants_error(
            &self,
            xquery: &NSString,
            constants: Option<&NSDictionary<NSString, Object>>,
        ) -> Result<Id<NSArray, Shared>, Id<NSError, Shared>>;

        #[method_id(objectsForXQuery:error:)]
        pub unsafe fn objectsForXQuery_error(
            &self,
            xquery: &NSString,
        ) -> Result<Id<NSArray, Shared>, Id<NSError, Shared>>;
    }
);
