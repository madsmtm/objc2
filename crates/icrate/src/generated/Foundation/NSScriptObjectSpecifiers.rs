use super::__exported::NSAppleEventDescriptor;
use super::__exported::NSArray;
use super::__exported::NSNumber;
use super::__exported::NSScriptClassDescription;
use super::__exported::NSScriptWhoseTest;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScriptObjectSpecifier;
    unsafe impl ClassType for NSScriptObjectSpecifier {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSScriptObjectSpecifier {
        #[method_id(objectSpecifierWithDescriptor:)]
        pub unsafe fn objectSpecifierWithDescriptor(
            descriptor: &NSAppleEventDescriptor,
        ) -> Option<Id<NSScriptObjectSpecifier, Shared>>;
        #[method_id(initWithContainerSpecifier:key:)]
        pub unsafe fn initWithContainerSpecifier_key(
            &self,
            container: &NSScriptObjectSpecifier,
            property: &NSString,
        ) -> Id<Self, Shared>;
        #[method_id(initWithContainerClassDescription:containerSpecifier:key:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key(
            &self,
            classDesc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(childSpecifier)]
        pub unsafe fn childSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>>;
        #[method(setChildSpecifier:)]
        pub unsafe fn setChildSpecifier(&self, childSpecifier: Option<&NSScriptObjectSpecifier>);
        #[method_id(containerSpecifier)]
        pub unsafe fn containerSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>>;
        #[method(setContainerSpecifier:)]
        pub unsafe fn setContainerSpecifier(
            &self,
            containerSpecifier: Option<&NSScriptObjectSpecifier>,
        );
        #[method(containerIsObjectBeingTested)]
        pub unsafe fn containerIsObjectBeingTested(&self) -> bool;
        #[method(setContainerIsObjectBeingTested:)]
        pub unsafe fn setContainerIsObjectBeingTested(&self, containerIsObjectBeingTested: bool);
        #[method(containerIsRangeContainerObject)]
        pub unsafe fn containerIsRangeContainerObject(&self) -> bool;
        #[method(setContainerIsRangeContainerObject:)]
        pub unsafe fn setContainerIsRangeContainerObject(
            &self,
            containerIsRangeContainerObject: bool,
        );
        #[method_id(key)]
        pub unsafe fn key(&self) -> Id<NSString, Shared>;
        #[method(setKey:)]
        pub unsafe fn setKey(&self, key: &NSString);
        #[method_id(containerClassDescription)]
        pub unsafe fn containerClassDescription(
            &self,
        ) -> Option<Id<NSScriptClassDescription, Shared>>;
        #[method(setContainerClassDescription:)]
        pub unsafe fn setContainerClassDescription(
            &self,
            containerClassDescription: Option<&NSScriptClassDescription>,
        );
        #[method_id(keyClassDescription)]
        pub unsafe fn keyClassDescription(&self) -> Option<Id<NSScriptClassDescription, Shared>>;
        #[method(indicesOfObjectsByEvaluatingWithContainer:count:)]
        pub unsafe fn indicesOfObjectsByEvaluatingWithContainer_count(
            &self,
            container: &Object,
            count: NonNull<NSInteger>,
        ) -> *mut NSInteger;
        #[method_id(objectsByEvaluatingWithContainers:)]
        pub unsafe fn objectsByEvaluatingWithContainers(
            &self,
            containers: &Object,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(objectsByEvaluatingSpecifier)]
        pub unsafe fn objectsByEvaluatingSpecifier(&self) -> Option<Id<Object, Shared>>;
        #[method(evaluationErrorNumber)]
        pub unsafe fn evaluationErrorNumber(&self) -> NSInteger;
        #[method(setEvaluationErrorNumber:)]
        pub unsafe fn setEvaluationErrorNumber(&self, evaluationErrorNumber: NSInteger);
        #[method_id(evaluationErrorSpecifier)]
        pub unsafe fn evaluationErrorSpecifier(
            &self,
        ) -> Option<Id<NSScriptObjectSpecifier, Shared>>;
        #[method_id(descriptor)]
        pub unsafe fn descriptor(&self) -> Option<Id<NSAppleEventDescriptor, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSScriptObjectSpecifiers"]
    unsafe impl NSObject {
        #[method_id(objectSpecifier)]
        pub unsafe fn objectSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>>;
        #[method_id(indicesOfObjectsByEvaluatingObjectSpecifier:)]
        pub unsafe fn indicesOfObjectsByEvaluatingObjectSpecifier(
            &self,
            specifier: &NSScriptObjectSpecifier,
        ) -> Option<Id<NSArray<NSNumber>, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSIndexSpecifier;
    unsafe impl ClassType for NSIndexSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
extern_methods!(
    unsafe impl NSIndexSpecifier {
        #[method_id(initWithContainerClassDescription:containerSpecifier:key:index:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_index(
            &self,
            classDesc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
            index: NSInteger,
        ) -> Id<Self, Shared>;
        #[method(index)]
        pub unsafe fn index(&self) -> NSInteger;
        #[method(setIndex:)]
        pub unsafe fn setIndex(&self, index: NSInteger);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSMiddleSpecifier;
    unsafe impl ClassType for NSMiddleSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
extern_methods!(
    unsafe impl NSMiddleSpecifier {}
);
extern_class!(
    #[derive(Debug)]
    pub struct NSNameSpecifier;
    unsafe impl ClassType for NSNameSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
extern_methods!(
    unsafe impl NSNameSpecifier {
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(initWithContainerClassDescription:containerSpecifier:key:name:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_name(
            &self,
            classDesc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
            name: &NSString,
        ) -> Id<Self, Shared>;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSPositionalSpecifier;
    unsafe impl ClassType for NSPositionalSpecifier {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPositionalSpecifier {
        #[method_id(initWithPosition:objectSpecifier:)]
        pub unsafe fn initWithPosition_objectSpecifier(
            &self,
            position: NSInsertionPosition,
            specifier: &NSScriptObjectSpecifier,
        ) -> Id<Self, Shared>;
        #[method(position)]
        pub unsafe fn position(&self) -> NSInsertionPosition;
        #[method_id(objectSpecifier)]
        pub unsafe fn objectSpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared>;
        #[method(setInsertionClassDescription:)]
        pub unsafe fn setInsertionClassDescription(
            &self,
            classDescription: &NSScriptClassDescription,
        );
        #[method(evaluate)]
        pub unsafe fn evaluate(&self);
        #[method_id(insertionContainer)]
        pub unsafe fn insertionContainer(&self) -> Option<Id<Object, Shared>>;
        #[method_id(insertionKey)]
        pub unsafe fn insertionKey(&self) -> Option<Id<NSString, Shared>>;
        #[method(insertionIndex)]
        pub unsafe fn insertionIndex(&self) -> NSInteger;
        #[method(insertionReplaces)]
        pub unsafe fn insertionReplaces(&self) -> bool;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSPropertySpecifier;
    unsafe impl ClassType for NSPropertySpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
extern_methods!(
    unsafe impl NSPropertySpecifier {}
);
extern_class!(
    #[derive(Debug)]
    pub struct NSRandomSpecifier;
    unsafe impl ClassType for NSRandomSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
extern_methods!(
    unsafe impl NSRandomSpecifier {}
);
extern_class!(
    #[derive(Debug)]
    pub struct NSRangeSpecifier;
    unsafe impl ClassType for NSRangeSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
extern_methods!(
    unsafe impl NSRangeSpecifier {
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(initWithContainerClassDescription:containerSpecifier:key:startSpecifier:endSpecifier:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_startSpecifier_endSpecifier(
            &self,
            classDesc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
            startSpec: Option<&NSScriptObjectSpecifier>,
            endSpec: Option<&NSScriptObjectSpecifier>,
        ) -> Id<Self, Shared>;
        #[method_id(startSpecifier)]
        pub unsafe fn startSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>>;
        #[method(setStartSpecifier:)]
        pub unsafe fn setStartSpecifier(&self, startSpecifier: Option<&NSScriptObjectSpecifier>);
        #[method_id(endSpecifier)]
        pub unsafe fn endSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>>;
        #[method(setEndSpecifier:)]
        pub unsafe fn setEndSpecifier(&self, endSpecifier: Option<&NSScriptObjectSpecifier>);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSRelativeSpecifier;
    unsafe impl ClassType for NSRelativeSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
extern_methods!(
    unsafe impl NSRelativeSpecifier {
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(initWithContainerClassDescription:containerSpecifier:key:relativePosition:baseSpecifier:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_relativePosition_baseSpecifier(
            &self,
            classDesc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
            relPos: NSRelativePosition,
            baseSpecifier: Option<&NSScriptObjectSpecifier>,
        ) -> Id<Self, Shared>;
        #[method(relativePosition)]
        pub unsafe fn relativePosition(&self) -> NSRelativePosition;
        #[method(setRelativePosition:)]
        pub unsafe fn setRelativePosition(&self, relativePosition: NSRelativePosition);
        #[method_id(baseSpecifier)]
        pub unsafe fn baseSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>>;
        #[method(setBaseSpecifier:)]
        pub unsafe fn setBaseSpecifier(&self, baseSpecifier: Option<&NSScriptObjectSpecifier>);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUniqueIDSpecifier;
    unsafe impl ClassType for NSUniqueIDSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
extern_methods!(
    unsafe impl NSUniqueIDSpecifier {
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(initWithContainerClassDescription:containerSpecifier:key:uniqueID:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_uniqueID(
            &self,
            classDesc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
            uniqueID: &Object,
        ) -> Id<Self, Shared>;
        #[method_id(uniqueID)]
        pub unsafe fn uniqueID(&self) -> Id<Object, Shared>;
        #[method(setUniqueID:)]
        pub unsafe fn setUniqueID(&self, uniqueID: &Object);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSWhoseSpecifier;
    unsafe impl ClassType for NSWhoseSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
extern_methods!(
    unsafe impl NSWhoseSpecifier {
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(initWithContainerClassDescription:containerSpecifier:key:test:)]
        pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_test(
            &self,
            classDesc: &NSScriptClassDescription,
            container: Option<&NSScriptObjectSpecifier>,
            property: &NSString,
            test: &NSScriptWhoseTest,
        ) -> Id<Self, Shared>;
        #[method_id(test)]
        pub unsafe fn test(&self) -> Id<NSScriptWhoseTest, Shared>;
        #[method(setTest:)]
        pub unsafe fn setTest(&self, test: &NSScriptWhoseTest);
        #[method(startSubelementIdentifier)]
        pub unsafe fn startSubelementIdentifier(&self) -> NSWhoseSubelementIdentifier;
        #[method(setStartSubelementIdentifier:)]
        pub unsafe fn setStartSubelementIdentifier(
            &self,
            startSubelementIdentifier: NSWhoseSubelementIdentifier,
        );
        #[method(startSubelementIndex)]
        pub unsafe fn startSubelementIndex(&self) -> NSInteger;
        #[method(setStartSubelementIndex:)]
        pub unsafe fn setStartSubelementIndex(&self, startSubelementIndex: NSInteger);
        #[method(endSubelementIdentifier)]
        pub unsafe fn endSubelementIdentifier(&self) -> NSWhoseSubelementIdentifier;
        #[method(setEndSubelementIdentifier:)]
        pub unsafe fn setEndSubelementIdentifier(
            &self,
            endSubelementIdentifier: NSWhoseSubelementIdentifier,
        );
        #[method(endSubelementIndex)]
        pub unsafe fn endSubelementIndex(&self) -> NSInteger;
        #[method(setEndSubelementIndex:)]
        pub unsafe fn setEndSubelementIndex(&self, endSubelementIndex: NSInteger);
    }
);
