use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSEnumerator::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSSet<ObjectType: Message>;
    unsafe impl<ObjectType: Message> ClassType for NSSet<ObjectType> {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;
        # [method_id (member :)]
        pub unsafe fn member(&self, object: &ObjectType) -> Option<Id<ObjectType, Shared>>;
        #[method_id(objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Id<NSEnumerator<ObjectType>, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        # [method_id (initWithObjects : count :)]
        pub unsafe fn initWithObjects_count(
            &self,
            objects: TodoArray,
            cnt: NSUInteger,
        ) -> Id<Self, Shared>;
        # [method_id (initWithCoder :)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSExtendedSet"]
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[method_id(allObjects)]
        pub unsafe fn allObjects(&self) -> Id<NSArray<ObjectType>, Shared>;
        #[method_id(anyObject)]
        pub unsafe fn anyObject(&self) -> Option<Id<ObjectType, Shared>>;
        # [method (containsObject :)]
        pub unsafe fn containsObject(&self, anObject: &ObjectType) -> bool;
        #[method_id(description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;
        # [method_id (descriptionWithLocale :)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>)
            -> Id<NSString, Shared>;
        # [method (intersectsSet :)]
        pub unsafe fn intersectsSet(&self, otherSet: &NSSet<ObjectType>) -> bool;
        # [method (isEqualToSet :)]
        pub unsafe fn isEqualToSet(&self, otherSet: &NSSet<ObjectType>) -> bool;
        # [method (isSubsetOfSet :)]
        pub unsafe fn isSubsetOfSet(&self, otherSet: &NSSet<ObjectType>) -> bool;
        # [method (makeObjectsPerformSelector :)]
        pub unsafe fn makeObjectsPerformSelector(&self, aSelector: Sel);
        # [method (makeObjectsPerformSelector : withObject :)]
        pub unsafe fn makeObjectsPerformSelector_withObject(
            &self,
            aSelector: Sel,
            argument: Option<&Object>,
        );
        # [method_id (setByAddingObject :)]
        pub unsafe fn setByAddingObject(
            &self,
            anObject: &ObjectType,
        ) -> Id<NSSet<ObjectType>, Shared>;
        # [method_id (setByAddingObjectsFromSet :)]
        pub unsafe fn setByAddingObjectsFromSet(
            &self,
            other: &NSSet<ObjectType>,
        ) -> Id<NSSet<ObjectType>, Shared>;
        # [method_id (setByAddingObjectsFromArray :)]
        pub unsafe fn setByAddingObjectsFromArray(
            &self,
            other: &NSArray<ObjectType>,
        ) -> Id<NSSet<ObjectType>, Shared>;
        # [method (enumerateObjectsUsingBlock :)]
        pub unsafe fn enumerateObjectsUsingBlock(&self, block: TodoBlock);
        # [method (enumerateObjectsWithOptions : usingBlock :)]
        pub unsafe fn enumerateObjectsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: TodoBlock,
        );
        # [method_id (objectsPassingTest :)]
        pub unsafe fn objectsPassingTest(
            &self,
            predicate: TodoBlock,
        ) -> Id<NSSet<ObjectType>, Shared>;
        # [method_id (objectsWithOptions : passingTest :)]
        pub unsafe fn objectsWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: TodoBlock,
        ) -> Id<NSSet<ObjectType>, Shared>;
    }
);
extern_methods!(
    #[doc = "NSSetCreation"]
    unsafe impl<ObjectType: Message> NSSet<ObjectType> {
        #[method_id(set)]
        pub unsafe fn set() -> Id<Self, Shared>;
        # [method_id (setWithObject :)]
        pub unsafe fn setWithObject(object: &ObjectType) -> Id<Self, Shared>;
        # [method_id (setWithObjects : count :)]
        pub unsafe fn setWithObjects_count(objects: TodoArray, cnt: NSUInteger)
            -> Id<Self, Shared>;
        # [method_id (setWithSet :)]
        pub unsafe fn setWithSet(set: &NSSet<ObjectType>) -> Id<Self, Shared>;
        # [method_id (setWithArray :)]
        pub unsafe fn setWithArray(array: &NSArray<ObjectType>) -> Id<Self, Shared>;
        # [method_id (initWithSet :)]
        pub unsafe fn initWithSet(&self, set: &NSSet<ObjectType>) -> Id<Self, Shared>;
        # [method_id (initWithSet : copyItems :)]
        pub unsafe fn initWithSet_copyItems(
            &self,
            set: &NSSet<ObjectType>,
            flag: bool,
        ) -> Id<Self, Shared>;
        # [method_id (initWithArray :)]
        pub unsafe fn initWithArray(&self, array: &NSArray<ObjectType>) -> Id<Self, Shared>;
    }
);
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSMutableSet<ObjectType: Message>;
    unsafe impl<ObjectType: Message> ClassType for NSMutableSet<ObjectType> {
        type Super = NSSet;
    }
);
extern_methods!(
    unsafe impl<ObjectType: Message> NSMutableSet<ObjectType> {
        # [method (addObject :)]
        pub unsafe fn addObject(&self, object: &ObjectType);
        # [method (removeObject :)]
        pub unsafe fn removeObject(&self, object: &ObjectType);
        # [method_id (initWithCoder :)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        # [method_id (initWithCapacity :)]
        pub unsafe fn initWithCapacity(&self, numItems: NSUInteger) -> Id<Self, Shared>;
    }
);
extern_methods!(
    #[doc = "NSExtendedMutableSet"]
    unsafe impl<ObjectType: Message> NSMutableSet<ObjectType> {
        # [method (addObjectsFromArray :)]
        pub unsafe fn addObjectsFromArray(&self, array: &NSArray<ObjectType>);
        # [method (intersectSet :)]
        pub unsafe fn intersectSet(&self, otherSet: &NSSet<ObjectType>);
        # [method (minusSet :)]
        pub unsafe fn minusSet(&self, otherSet: &NSSet<ObjectType>);
        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);
        # [method (unionSet :)]
        pub unsafe fn unionSet(&self, otherSet: &NSSet<ObjectType>);
        # [method (setSet :)]
        pub unsafe fn setSet(&self, otherSet: &NSSet<ObjectType>);
    }
);
extern_methods!(
    #[doc = "NSMutableSetCreation"]
    unsafe impl<ObjectType: Message> NSMutableSet<ObjectType> {
        # [method_id (setWithCapacity :)]
        pub unsafe fn setWithCapacity(numItems: NSUInteger) -> Id<Self, Shared>;
    }
);
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSCountedSet<ObjectType: Message>;
    unsafe impl<ObjectType: Message> ClassType for NSCountedSet<ObjectType> {
        type Super = NSMutableSet;
    }
);
extern_methods!(
    unsafe impl<ObjectType: Message> NSCountedSet<ObjectType> {
        # [method_id (initWithCapacity :)]
        pub unsafe fn initWithCapacity(&self, numItems: NSUInteger) -> Id<Self, Shared>;
        # [method_id (initWithArray :)]
        pub unsafe fn initWithArray(&self, array: &NSArray<ObjectType>) -> Id<Self, Shared>;
        # [method_id (initWithSet :)]
        pub unsafe fn initWithSet(&self, set: &NSSet<ObjectType>) -> Id<Self, Shared>;
        # [method (countForObject :)]
        pub unsafe fn countForObject(&self, object: &ObjectType) -> NSUInteger;
        #[method_id(objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Id<NSEnumerator<ObjectType>, Shared>;
        # [method (addObject :)]
        pub unsafe fn addObject(&self, object: &ObjectType);
        # [method (removeObject :)]
        pub unsafe fn removeObject(&self, object: &ObjectType);
    }
);
