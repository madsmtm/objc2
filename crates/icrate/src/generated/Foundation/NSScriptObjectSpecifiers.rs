#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScriptObjectSpecifier;
    unsafe impl ClassType for NSScriptObjectSpecifier {
        type Super = NSObject;
    }
);
impl NSScriptObjectSpecifier {
    pub unsafe fn objectSpecifierWithDescriptor(
        descriptor: &NSAppleEventDescriptor,
    ) -> Option<Id<NSScriptObjectSpecifier, Shared>> {
        msg_send_id![Self::class(), objectSpecifierWithDescriptor: descriptor]
    }
    pub unsafe fn initWithContainerSpecifier_key(
        &self,
        container: &NSScriptObjectSpecifier,
        property: &NSString,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithContainerSpecifier: container, key: property]
    }
    pub unsafe fn initWithContainerClassDescription_containerSpecifier_key(
        &self,
        classDesc: &NSScriptClassDescription,
        container: Option<&NSScriptObjectSpecifier>,
        property: &NSString,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithContainerClassDescription: classDesc,
            containerSpecifier: container,
            key: property
        ]
    }
    pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: inCoder]
    }
    pub unsafe fn indicesOfObjectsByEvaluatingWithContainer_count(
        &self,
        container: &Object,
        count: NonNull<NSInteger>,
    ) -> *mut NSInteger {
        msg_send![
            self,
            indicesOfObjectsByEvaluatingWithContainer: container,
            count: count
        ]
    }
    pub unsafe fn objectsByEvaluatingWithContainers(
        &self,
        containers: &Object,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, objectsByEvaluatingWithContainers: containers]
    }
    pub unsafe fn childSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>> {
        msg_send_id![self, childSpecifier]
    }
    pub unsafe fn setChildSpecifier(&self, childSpecifier: Option<&NSScriptObjectSpecifier>) {
        msg_send![self, setChildSpecifier: childSpecifier]
    }
    pub unsafe fn containerSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>> {
        msg_send_id![self, containerSpecifier]
    }
    pub unsafe fn setContainerSpecifier(
        &self,
        containerSpecifier: Option<&NSScriptObjectSpecifier>,
    ) {
        msg_send![self, setContainerSpecifier: containerSpecifier]
    }
    pub unsafe fn containerIsObjectBeingTested(&self) -> bool {
        msg_send![self, containerIsObjectBeingTested]
    }
    pub unsafe fn setContainerIsObjectBeingTested(&self, containerIsObjectBeingTested: bool) {
        msg_send![
            self,
            setContainerIsObjectBeingTested: containerIsObjectBeingTested
        ]
    }
    pub unsafe fn containerIsRangeContainerObject(&self) -> bool {
        msg_send![self, containerIsRangeContainerObject]
    }
    pub unsafe fn setContainerIsRangeContainerObject(&self, containerIsRangeContainerObject: bool) {
        msg_send![
            self,
            setContainerIsRangeContainerObject: containerIsRangeContainerObject
        ]
    }
    pub unsafe fn key(&self) -> Id<NSString, Shared> {
        msg_send_id![self, key]
    }
    pub unsafe fn setKey(&self, key: &NSString) {
        msg_send![self, setKey: key]
    }
    pub unsafe fn containerClassDescription(&self) -> Option<Id<NSScriptClassDescription, Shared>> {
        msg_send_id![self, containerClassDescription]
    }
    pub unsafe fn setContainerClassDescription(
        &self,
        containerClassDescription: Option<&NSScriptClassDescription>,
    ) {
        msg_send![
            self,
            setContainerClassDescription: containerClassDescription
        ]
    }
    pub unsafe fn keyClassDescription(&self) -> Option<Id<NSScriptClassDescription, Shared>> {
        msg_send_id![self, keyClassDescription]
    }
    pub unsafe fn objectsByEvaluatingSpecifier(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, objectsByEvaluatingSpecifier]
    }
    pub unsafe fn evaluationErrorNumber(&self) -> NSInteger {
        msg_send![self, evaluationErrorNumber]
    }
    pub unsafe fn setEvaluationErrorNumber(&self, evaluationErrorNumber: NSInteger) {
        msg_send![self, setEvaluationErrorNumber: evaluationErrorNumber]
    }
    pub unsafe fn evaluationErrorSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>> {
        msg_send_id![self, evaluationErrorSpecifier]
    }
    pub unsafe fn descriptor(&self) -> Option<Id<NSAppleEventDescriptor, Shared>> {
        msg_send_id![self, descriptor]
    }
}
#[doc = "NSScriptObjectSpecifiers"]
impl NSObject {
    pub unsafe fn indicesOfObjectsByEvaluatingObjectSpecifier(
        &self,
        specifier: &NSScriptObjectSpecifier,
    ) -> TodoGenerics {
        msg_send![self, indicesOfObjectsByEvaluatingObjectSpecifier: specifier]
    }
    pub unsafe fn objectSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>> {
        msg_send_id![self, objectSpecifier]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSIndexSpecifier;
    unsafe impl ClassType for NSIndexSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
impl NSIndexSpecifier {
    pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_index(
        &self,
        classDesc: &NSScriptClassDescription,
        container: Option<&NSScriptObjectSpecifier>,
        property: &NSString,
        index: NSInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithContainerClassDescription: classDesc,
            containerSpecifier: container,
            key: property,
            index: index
        ]
    }
    pub unsafe fn index(&self) -> NSInteger {
        msg_send![self, index]
    }
    pub unsafe fn setIndex(&self, index: NSInteger) {
        msg_send![self, setIndex: index]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSMiddleSpecifier;
    unsafe impl ClassType for NSMiddleSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
impl NSMiddleSpecifier {}
extern_class!(
    #[derive(Debug)]
    pub struct NSNameSpecifier;
    unsafe impl ClassType for NSNameSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
impl NSNameSpecifier {
    pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: inCoder]
    }
    pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_name(
        &self,
        classDesc: &NSScriptClassDescription,
        container: Option<&NSScriptObjectSpecifier>,
        property: &NSString,
        name: &NSString,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithContainerClassDescription: classDesc,
            containerSpecifier: container,
            key: property,
            name: name
        ]
    }
    pub unsafe fn name(&self) -> Id<NSString, Shared> {
        msg_send_id![self, name]
    }
    pub unsafe fn setName(&self, name: &NSString) {
        msg_send![self, setName: name]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSPositionalSpecifier;
    unsafe impl ClassType for NSPositionalSpecifier {
        type Super = NSObject;
    }
);
impl NSPositionalSpecifier {
    pub unsafe fn initWithPosition_objectSpecifier(
        &self,
        position: NSInsertionPosition,
        specifier: &NSScriptObjectSpecifier,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithPosition: position, objectSpecifier: specifier]
    }
    pub unsafe fn setInsertionClassDescription(&self, classDescription: &NSScriptClassDescription) {
        msg_send![self, setInsertionClassDescription: classDescription]
    }
    pub unsafe fn evaluate(&self) {
        msg_send![self, evaluate]
    }
    pub unsafe fn position(&self) -> NSInsertionPosition {
        msg_send![self, position]
    }
    pub unsafe fn objectSpecifier(&self) -> Id<NSScriptObjectSpecifier, Shared> {
        msg_send_id![self, objectSpecifier]
    }
    pub unsafe fn insertionContainer(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, insertionContainer]
    }
    pub unsafe fn insertionKey(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, insertionKey]
    }
    pub unsafe fn insertionIndex(&self) -> NSInteger {
        msg_send![self, insertionIndex]
    }
    pub unsafe fn insertionReplaces(&self) -> bool {
        msg_send![self, insertionReplaces]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSPropertySpecifier;
    unsafe impl ClassType for NSPropertySpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
impl NSPropertySpecifier {}
extern_class!(
    #[derive(Debug)]
    pub struct NSRandomSpecifier;
    unsafe impl ClassType for NSRandomSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
impl NSRandomSpecifier {}
extern_class!(
    #[derive(Debug)]
    pub struct NSRangeSpecifier;
    unsafe impl ClassType for NSRangeSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
impl NSRangeSpecifier {
    pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: inCoder]
    }
    pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_startSpecifier_endSpecifier(
        &self,
        classDesc: &NSScriptClassDescription,
        container: Option<&NSScriptObjectSpecifier>,
        property: &NSString,
        startSpec: Option<&NSScriptObjectSpecifier>,
        endSpec: Option<&NSScriptObjectSpecifier>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithContainerClassDescription: classDesc,
            containerSpecifier: container,
            key: property,
            startSpecifier: startSpec,
            endSpecifier: endSpec
        ]
    }
    pub unsafe fn startSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>> {
        msg_send_id![self, startSpecifier]
    }
    pub unsafe fn setStartSpecifier(&self, startSpecifier: Option<&NSScriptObjectSpecifier>) {
        msg_send![self, setStartSpecifier: startSpecifier]
    }
    pub unsafe fn endSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>> {
        msg_send_id![self, endSpecifier]
    }
    pub unsafe fn setEndSpecifier(&self, endSpecifier: Option<&NSScriptObjectSpecifier>) {
        msg_send![self, setEndSpecifier: endSpecifier]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSRelativeSpecifier;
    unsafe impl ClassType for NSRelativeSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
impl NSRelativeSpecifier {
    pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: inCoder]
    }
    pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_relativePosition_baseSpecifier(
        &self,
        classDesc: &NSScriptClassDescription,
        container: Option<&NSScriptObjectSpecifier>,
        property: &NSString,
        relPos: NSRelativePosition,
        baseSpecifier: Option<&NSScriptObjectSpecifier>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithContainerClassDescription: classDesc,
            containerSpecifier: container,
            key: property,
            relativePosition: relPos,
            baseSpecifier: baseSpecifier
        ]
    }
    pub unsafe fn relativePosition(&self) -> NSRelativePosition {
        msg_send![self, relativePosition]
    }
    pub unsafe fn setRelativePosition(&self, relativePosition: NSRelativePosition) {
        msg_send![self, setRelativePosition: relativePosition]
    }
    pub unsafe fn baseSpecifier(&self) -> Option<Id<NSScriptObjectSpecifier, Shared>> {
        msg_send_id![self, baseSpecifier]
    }
    pub unsafe fn setBaseSpecifier(&self, baseSpecifier: Option<&NSScriptObjectSpecifier>) {
        msg_send![self, setBaseSpecifier: baseSpecifier]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUniqueIDSpecifier;
    unsafe impl ClassType for NSUniqueIDSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
impl NSUniqueIDSpecifier {
    pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: inCoder]
    }
    pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_uniqueID(
        &self,
        classDesc: &NSScriptClassDescription,
        container: Option<&NSScriptObjectSpecifier>,
        property: &NSString,
        uniqueID: &Object,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithContainerClassDescription: classDesc,
            containerSpecifier: container,
            key: property,
            uniqueID: uniqueID
        ]
    }
    pub unsafe fn uniqueID(&self) -> Id<Object, Shared> {
        msg_send_id![self, uniqueID]
    }
    pub unsafe fn setUniqueID(&self, uniqueID: &Object) {
        msg_send![self, setUniqueID: uniqueID]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSWhoseSpecifier;
    unsafe impl ClassType for NSWhoseSpecifier {
        type Super = NSScriptObjectSpecifier;
    }
);
impl NSWhoseSpecifier {
    pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: inCoder]
    }
    pub unsafe fn initWithContainerClassDescription_containerSpecifier_key_test(
        &self,
        classDesc: &NSScriptClassDescription,
        container: Option<&NSScriptObjectSpecifier>,
        property: &NSString,
        test: &NSScriptWhoseTest,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithContainerClassDescription: classDesc,
            containerSpecifier: container,
            key: property,
            test: test
        ]
    }
    pub unsafe fn test(&self) -> Id<NSScriptWhoseTest, Shared> {
        msg_send_id![self, test]
    }
    pub unsafe fn setTest(&self, test: &NSScriptWhoseTest) {
        msg_send![self, setTest: test]
    }
    pub unsafe fn startSubelementIdentifier(&self) -> NSWhoseSubelementIdentifier {
        msg_send![self, startSubelementIdentifier]
    }
    pub unsafe fn setStartSubelementIdentifier(
        &self,
        startSubelementIdentifier: NSWhoseSubelementIdentifier,
    ) {
        msg_send![
            self,
            setStartSubelementIdentifier: startSubelementIdentifier
        ]
    }
    pub unsafe fn startSubelementIndex(&self) -> NSInteger {
        msg_send![self, startSubelementIndex]
    }
    pub unsafe fn setStartSubelementIndex(&self, startSubelementIndex: NSInteger) {
        msg_send![self, setStartSubelementIndex: startSubelementIndex]
    }
    pub unsafe fn endSubelementIdentifier(&self) -> NSWhoseSubelementIdentifier {
        msg_send![self, endSubelementIdentifier]
    }
    pub unsafe fn setEndSubelementIdentifier(
        &self,
        endSubelementIdentifier: NSWhoseSubelementIdentifier,
    ) {
        msg_send![self, setEndSubelementIdentifier: endSubelementIdentifier]
    }
    pub unsafe fn endSubelementIndex(&self) -> NSInteger {
        msg_send![self, endSubelementIndex]
    }
    pub unsafe fn setEndSubelementIndex(&self, endSubelementIndex: NSInteger) {
        msg_send![self, setEndSubelementIndex: endSubelementIndex]
    }
}
