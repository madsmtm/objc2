#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSet;
    unsafe impl ClassType for NSSet {
        type Super = NSObject;
    }
);
impl NSSet {
    pub unsafe fn member(&self, object: ObjectType) -> ObjectType {
        msg_send![self, member: object]
    }
    pub unsafe fn objectEnumerator(&self) -> TodoGenerics {
        msg_send![self, objectEnumerator]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithObjects_count(
        &self,
        objects: TodoArray,
        cnt: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithObjects: objects, count: cnt]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
    pub unsafe fn count(&self) -> NSUInteger {
        msg_send![self, count]
    }
}
#[doc = "NSExtendedSet"]
impl NSSet {
    pub unsafe fn anyObject(&self) -> ObjectType {
        msg_send![self, anyObject]
    }
    pub unsafe fn containsObject(&self, anObject: ObjectType) -> bool {
        msg_send![self, containsObject: anObject]
    }
    pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>) -> Id<NSString, Shared> {
        msg_send_id![self, descriptionWithLocale: locale]
    }
    pub unsafe fn intersectsSet(&self, otherSet: TodoGenerics) -> bool {
        msg_send![self, intersectsSet: otherSet]
    }
    pub unsafe fn isEqualToSet(&self, otherSet: TodoGenerics) -> bool {
        msg_send![self, isEqualToSet: otherSet]
    }
    pub unsafe fn isSubsetOfSet(&self, otherSet: TodoGenerics) -> bool {
        msg_send![self, isSubsetOfSet: otherSet]
    }
    pub unsafe fn makeObjectsPerformSelector(&self, aSelector: Sel) {
        msg_send![self, makeObjectsPerformSelector: aSelector]
    }
    pub unsafe fn makeObjectsPerformSelector_withObject(
        &self,
        aSelector: Sel,
        argument: Option<&Object>,
    ) {
        msg_send![
            self,
            makeObjectsPerformSelector: aSelector,
            withObject: argument
        ]
    }
    pub unsafe fn setByAddingObject(&self, anObject: ObjectType) -> TodoGenerics {
        msg_send![self, setByAddingObject: anObject]
    }
    pub unsafe fn setByAddingObjectsFromSet(&self, other: TodoGenerics) -> TodoGenerics {
        msg_send![self, setByAddingObjectsFromSet: other]
    }
    pub unsafe fn setByAddingObjectsFromArray(&self, other: TodoGenerics) -> TodoGenerics {
        msg_send![self, setByAddingObjectsFromArray: other]
    }
    pub unsafe fn enumerateObjectsUsingBlock(&self, block: TodoBlock) {
        msg_send![self, enumerateObjectsUsingBlock: block]
    }
    pub unsafe fn enumerateObjectsWithOptions_usingBlock(
        &self,
        opts: NSEnumerationOptions,
        block: TodoBlock,
    ) {
        msg_send![self, enumerateObjectsWithOptions: opts, usingBlock: block]
    }
    pub unsafe fn objectsPassingTest(&self, predicate: TodoBlock) -> TodoGenerics {
        msg_send![self, objectsPassingTest: predicate]
    }
    pub unsafe fn objectsWithOptions_passingTest(
        &self,
        opts: NSEnumerationOptions,
        predicate: TodoBlock,
    ) -> TodoGenerics {
        msg_send![self, objectsWithOptions: opts, passingTest: predicate]
    }
    pub unsafe fn allObjects(&self) -> TodoGenerics {
        msg_send![self, allObjects]
    }
    pub unsafe fn description(&self) -> Id<NSString, Shared> {
        msg_send_id![self, description]
    }
}
#[doc = "NSSetCreation"]
impl NSSet {
    pub unsafe fn set() -> Id<Self, Shared> {
        msg_send_id![Self::class(), set]
    }
    pub unsafe fn setWithObject(object: ObjectType) -> Id<Self, Shared> {
        msg_send_id![Self::class(), setWithObject: object]
    }
    pub unsafe fn setWithObjects_count(objects: TodoArray, cnt: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![Self::class(), setWithObjects: objects, count: cnt]
    }
    pub unsafe fn setWithSet(set: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![Self::class(), setWithSet: set]
    }
    pub unsafe fn setWithArray(array: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![Self::class(), setWithArray: array]
    }
    pub unsafe fn initWithSet(&self, set: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![self, initWithSet: set]
    }
    pub unsafe fn initWithSet_copyItems(&self, set: TodoGenerics, flag: bool) -> Id<Self, Shared> {
        msg_send_id![self, initWithSet: set, copyItems: flag]
    }
    pub unsafe fn initWithArray(&self, array: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![self, initWithArray: array]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSMutableSet;
    unsafe impl ClassType for NSMutableSet {
        type Super = NSSet;
    }
);
impl NSMutableSet {
    pub unsafe fn addObject(&self, object: ObjectType) {
        msg_send![self, addObject: object]
    }
    pub unsafe fn removeObject(&self, object: ObjectType) {
        msg_send![self, removeObject: object]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithCapacity(&self, numItems: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![self, initWithCapacity: numItems]
    }
}
#[doc = "NSExtendedMutableSet"]
impl NSMutableSet {
    pub unsafe fn addObjectsFromArray(&self, array: TodoGenerics) {
        msg_send![self, addObjectsFromArray: array]
    }
    pub unsafe fn intersectSet(&self, otherSet: TodoGenerics) {
        msg_send![self, intersectSet: otherSet]
    }
    pub unsafe fn minusSet(&self, otherSet: TodoGenerics) {
        msg_send![self, minusSet: otherSet]
    }
    pub unsafe fn removeAllObjects(&self) {
        msg_send![self, removeAllObjects]
    }
    pub unsafe fn unionSet(&self, otherSet: TodoGenerics) {
        msg_send![self, unionSet: otherSet]
    }
    pub unsafe fn setSet(&self, otherSet: TodoGenerics) {
        msg_send![self, setSet: otherSet]
    }
}
#[doc = "NSMutableSetCreation"]
impl NSMutableSet {
    pub unsafe fn setWithCapacity(numItems: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![Self::class(), setWithCapacity: numItems]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSCountedSet;
    unsafe impl ClassType for NSCountedSet {
        type Super = NSMutableSet;
    }
);
impl NSCountedSet {
    pub unsafe fn initWithCapacity(&self, numItems: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![self, initWithCapacity: numItems]
    }
    pub unsafe fn initWithArray(&self, array: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![self, initWithArray: array]
    }
    pub unsafe fn initWithSet(&self, set: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![self, initWithSet: set]
    }
    pub unsafe fn countForObject(&self, object: ObjectType) -> NSUInteger {
        msg_send![self, countForObject: object]
    }
    pub unsafe fn objectEnumerator(&self) -> TodoGenerics {
        msg_send![self, objectEnumerator]
    }
    pub unsafe fn addObject(&self, object: ObjectType) {
        msg_send![self, addObject: object]
    }
    pub unsafe fn removeObject(&self, object: ObjectType) {
        msg_send![self, removeObject: object]
    }
}
