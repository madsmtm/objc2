#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSXMLNode;
    unsafe impl ClassType for NSXMLNode {
        type Super = NSObject;
    }
);
impl NSXMLNode {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithKind(&self, kind: NSXMLNodeKind) -> Id<Self, Shared> {
        msg_send_id![self, initWithKind: kind]
    }
    pub unsafe fn initWithKind_options(
        &self,
        kind: NSXMLNodeKind,
        options: NSXMLNodeOptions,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithKind: kind, options: options]
    }
    pub unsafe fn document() -> Id<Object, Shared> {
        msg_send_id![Self::class(), document]
    }
    pub unsafe fn documentWithRootElement(element: &NSXMLElement) -> Id<Object, Shared> {
        msg_send_id![Self::class(), documentWithRootElement: element]
    }
    pub unsafe fn elementWithName(name: &NSString) -> Id<Object, Shared> {
        msg_send_id![Self::class(), elementWithName: name]
    }
    pub unsafe fn elementWithName_URI(name: &NSString, URI: &NSString) -> Id<Object, Shared> {
        msg_send_id![Self::class(), elementWithName: name, URI: URI]
    }
    pub unsafe fn elementWithName_stringValue(
        name: &NSString,
        string: &NSString,
    ) -> Id<Object, Shared> {
        msg_send_id![Self::class(), elementWithName: name, stringValue: string]
    }
    pub unsafe fn elementWithName_children_attributes(
        name: &NSString,
        children: TodoGenerics,
        attributes: TodoGenerics,
    ) -> Id<Object, Shared> {
        msg_send_id![
            Self::class(),
            elementWithName: name,
            children: children,
            attributes: attributes
        ]
    }
    pub unsafe fn attributeWithName_stringValue(
        name: &NSString,
        stringValue: &NSString,
    ) -> Id<Object, Shared> {
        msg_send_id![
            Self::class(),
            attributeWithName: name,
            stringValue: stringValue
        ]
    }
    pub unsafe fn attributeWithName_URI_stringValue(
        name: &NSString,
        URI: &NSString,
        stringValue: &NSString,
    ) -> Id<Object, Shared> {
        msg_send_id![
            Self::class(),
            attributeWithName: name,
            URI: URI,
            stringValue: stringValue
        ]
    }
    pub unsafe fn namespaceWithName_stringValue(
        name: &NSString,
        stringValue: &NSString,
    ) -> Id<Object, Shared> {
        msg_send_id![
            Self::class(),
            namespaceWithName: name,
            stringValue: stringValue
        ]
    }
    pub unsafe fn processingInstructionWithName_stringValue(
        name: &NSString,
        stringValue: &NSString,
    ) -> Id<Object, Shared> {
        msg_send_id![
            Self::class(),
            processingInstructionWithName: name,
            stringValue: stringValue
        ]
    }
    pub unsafe fn commentWithStringValue(stringValue: &NSString) -> Id<Object, Shared> {
        msg_send_id![Self::class(), commentWithStringValue: stringValue]
    }
    pub unsafe fn textWithStringValue(stringValue: &NSString) -> Id<Object, Shared> {
        msg_send_id![Self::class(), textWithStringValue: stringValue]
    }
    pub unsafe fn DTDNodeWithXMLString(string: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![Self::class(), DTDNodeWithXMLString: string]
    }
    pub unsafe fn setStringValue_resolvingEntities(&self, string: &NSString, resolve: bool) {
        msg_send![self, setStringValue: string, resolvingEntities: resolve]
    }
    pub unsafe fn childAtIndex(&self, index: NSUInteger) -> Option<Id<NSXMLNode, Shared>> {
        msg_send_id![self, childAtIndex: index]
    }
    pub unsafe fn detach(&self) {
        msg_send![self, detach]
    }
    pub unsafe fn localNameForName(name: &NSString) -> Id<NSString, Shared> {
        msg_send_id![Self::class(), localNameForName: name]
    }
    pub unsafe fn prefixForName(name: &NSString) -> Option<Id<NSString, Shared>> {
        msg_send_id![Self::class(), prefixForName: name]
    }
    pub unsafe fn predefinedNamespaceForPrefix(name: &NSString) -> Option<Id<NSXMLNode, Shared>> {
        msg_send_id![Self::class(), predefinedNamespaceForPrefix: name]
    }
    pub unsafe fn XMLStringWithOptions(&self, options: NSXMLNodeOptions) -> Id<NSString, Shared> {
        msg_send_id![self, XMLStringWithOptions: options]
    }
    pub unsafe fn canonicalXMLStringPreservingComments(
        &self,
        comments: bool,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, canonicalXMLStringPreservingComments: comments]
    }
    pub unsafe fn nodesForXPath_error(
        &self,
        xpath: &NSString,
        error: *mut Option<&NSError>,
    ) -> TodoGenerics {
        msg_send![self, nodesForXPath: xpath, error: error]
    }
    pub unsafe fn objectsForXQuery_constants_error(
        &self,
        xquery: &NSString,
        constants: TodoGenerics,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSArray, Shared>> {
        msg_send_id![
            self,
            objectsForXQuery: xquery,
            constants: constants,
            error: error
        ]
    }
    pub unsafe fn objectsForXQuery_error(
        &self,
        xquery: &NSString,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSArray, Shared>> {
        msg_send_id![self, objectsForXQuery: xquery, error: error]
    }
    pub unsafe fn kind(&self) -> NSXMLNodeKind {
        msg_send![self, kind]
    }
    pub unsafe fn name(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, name]
    }
    pub unsafe fn setName(&self, name: Option<&NSString>) {
        msg_send![self, setName: name]
    }
    pub unsafe fn objectValue(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, objectValue]
    }
    pub unsafe fn setObjectValue(&self, objectValue: Option<&Object>) {
        msg_send![self, setObjectValue: objectValue]
    }
    pub unsafe fn stringValue(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringValue]
    }
    pub unsafe fn setStringValue(&self, stringValue: Option<&NSString>) {
        msg_send![self, setStringValue: stringValue]
    }
    pub unsafe fn index(&self) -> NSUInteger {
        msg_send![self, index]
    }
    pub unsafe fn level(&self) -> NSUInteger {
        msg_send![self, level]
    }
    pub unsafe fn rootDocument(&self) -> Option<Id<NSXMLDocument, Shared>> {
        msg_send_id![self, rootDocument]
    }
    pub unsafe fn parent(&self) -> Option<Id<NSXMLNode, Shared>> {
        msg_send_id![self, parent]
    }
    pub unsafe fn childCount(&self) -> NSUInteger {
        msg_send![self, childCount]
    }
    pub unsafe fn children(&self) -> TodoGenerics {
        msg_send![self, children]
    }
    pub unsafe fn previousSibling(&self) -> Option<Id<NSXMLNode, Shared>> {
        msg_send_id![self, previousSibling]
    }
    pub unsafe fn nextSibling(&self) -> Option<Id<NSXMLNode, Shared>> {
        msg_send_id![self, nextSibling]
    }
    pub unsafe fn previousNode(&self) -> Option<Id<NSXMLNode, Shared>> {
        msg_send_id![self, previousNode]
    }
    pub unsafe fn nextNode(&self) -> Option<Id<NSXMLNode, Shared>> {
        msg_send_id![self, nextNode]
    }
    pub unsafe fn XPath(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, XPath]
    }
    pub unsafe fn localName(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, localName]
    }
    pub unsafe fn prefix(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, prefix]
    }
    pub unsafe fn URI(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, URI]
    }
    pub unsafe fn setURI(&self, URI: Option<&NSString>) {
        msg_send![self, setURI: URI]
    }
    pub unsafe fn description(&self) -> Id<NSString, Shared> {
        msg_send_id![self, description]
    }
    pub unsafe fn XMLString(&self) -> Id<NSString, Shared> {
        msg_send_id![self, XMLString]
    }
}
