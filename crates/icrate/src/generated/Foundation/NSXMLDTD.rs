#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSXMLDTD;
    unsafe impl ClassType for NSXMLDTD {
        type Super = NSXMLNode;
    }
);
extern_methods!(
    unsafe impl NSXMLDTD {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            &self,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self, Shared>;
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
        #[method_id(publicID)]
        pub unsafe fn publicID(&self) -> Option<Id<NSString, Shared>>;
        #[method(setPublicID:)]
        pub unsafe fn setPublicID(&self, publicID: Option<&NSString>);
        #[method_id(systemID)]
        pub unsafe fn systemID(&self) -> Option<Id<NSString, Shared>>;
        #[method(setSystemID:)]
        pub unsafe fn setSystemID(&self, systemID: Option<&NSString>);
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
        #[method_id(entityDeclarationForName:)]
        pub unsafe fn entityDeclarationForName(
            &self,
            name: &NSString,
        ) -> Option<Id<NSXMLDTDNode, Shared>>;
        #[method_id(notationDeclarationForName:)]
        pub unsafe fn notationDeclarationForName(
            &self,
            name: &NSString,
        ) -> Option<Id<NSXMLDTDNode, Shared>>;
        #[method_id(elementDeclarationForName:)]
        pub unsafe fn elementDeclarationForName(
            &self,
            name: &NSString,
        ) -> Option<Id<NSXMLDTDNode, Shared>>;
        #[method_id(attributeDeclarationForName:elementName:)]
        pub unsafe fn attributeDeclarationForName_elementName(
            &self,
            name: &NSString,
            elementName: &NSString,
        ) -> Option<Id<NSXMLDTDNode, Shared>>;
        #[method_id(predefinedEntityDeclarationForName:)]
        pub unsafe fn predefinedEntityDeclarationForName(
            name: &NSString,
        ) -> Option<Id<NSXMLDTDNode, Shared>>;
    }
);
