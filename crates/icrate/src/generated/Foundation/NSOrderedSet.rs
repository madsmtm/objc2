use super::__exported::NSArray;
use super::__exported::NSIndexSet;
use super::__exported::NSSet;
use super::__exported::NSString;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSEnumerator::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSOrderedCollectionDifference::*;
use crate::Foundation::generated::NSRange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSOrderedSet<ObjectType: Message>;
    unsafe impl<ObjectType: Message> ClassType for NSOrderedSet<ObjectType> {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;
        #[method_id(objectAtIndex:)]
        pub unsafe fn objectAtIndex(&self, idx: NSUInteger) -> Id<ObjectType, Shared>;
        #[method(indexOfObject:)]
        pub unsafe fn indexOfObject(&self, object: &ObjectType) -> NSUInteger;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithObjects:count:)]
        pub unsafe fn initWithObjects_count(
            &self,
            objects: TodoArray,
            cnt: NSUInteger,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSExtendedOrderedSet"]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[method(getObjects:range:)]
        pub unsafe fn getObjects_range(&self, objects: TodoArray, range: NSRange);
        #[method_id(objectsAtIndexes:)]
        pub unsafe fn objectsAtIndexes(
            &self,
            indexes: &NSIndexSet,
        ) -> Id<NSArray<ObjectType>, Shared>;
        #[method_id(firstObject)]
        pub unsafe fn firstObject(&self) -> Option<Id<ObjectType, Shared>>;
        #[method_id(lastObject)]
        pub unsafe fn lastObject(&self) -> Option<Id<ObjectType, Shared>>;
        #[method(isEqualToOrderedSet:)]
        pub unsafe fn isEqualToOrderedSet(&self, other: &NSOrderedSet<ObjectType>) -> bool;
        #[method(containsObject:)]
        pub unsafe fn containsObject(&self, object: &ObjectType) -> bool;
        #[method(intersectsOrderedSet:)]
        pub unsafe fn intersectsOrderedSet(&self, other: &NSOrderedSet<ObjectType>) -> bool;
        #[method(intersectsSet:)]
        pub unsafe fn intersectsSet(&self, set: &NSSet<ObjectType>) -> bool;
        #[method(isSubsetOfOrderedSet:)]
        pub unsafe fn isSubsetOfOrderedSet(&self, other: &NSOrderedSet<ObjectType>) -> bool;
        #[method(isSubsetOfSet:)]
        pub unsafe fn isSubsetOfSet(&self, set: &NSSet<ObjectType>) -> bool;
        #[method_id(objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(&self, idx: NSUInteger) -> Id<ObjectType, Shared>;
        #[method_id(objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Id<NSEnumerator<ObjectType>, Shared>;
        #[method_id(reverseObjectEnumerator)]
        pub unsafe fn reverseObjectEnumerator(&self) -> Id<NSEnumerator<ObjectType>, Shared>;
        #[method_id(reversedOrderedSet)]
        pub unsafe fn reversedOrderedSet(&self) -> Id<NSOrderedSet<ObjectType>, Shared>;
        #[method_id(array)]
        pub unsafe fn array(&self) -> Id<NSArray<ObjectType>, Shared>;
        #[method_id(set)]
        pub unsafe fn set(&self) -> Id<NSSet<ObjectType>, Shared>;
        #[method(enumerateObjectsUsingBlock:)]
        pub unsafe fn enumerateObjectsUsingBlock(&self, block: TodoBlock);
        #[method(enumerateObjectsWithOptions:usingBlock:)]
        pub unsafe fn enumerateObjectsWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: TodoBlock,
        );
        #[method(enumerateObjectsAtIndexes:options:usingBlock:)]
        pub unsafe fn enumerateObjectsAtIndexes_options_usingBlock(
            &self,
            s: &NSIndexSet,
            opts: NSEnumerationOptions,
            block: TodoBlock,
        );
        #[method(indexOfObjectPassingTest:)]
        pub unsafe fn indexOfObjectPassingTest(&self, predicate: TodoBlock) -> NSUInteger;
        #[method(indexOfObjectWithOptions:passingTest:)]
        pub unsafe fn indexOfObjectWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: TodoBlock,
        ) -> NSUInteger;
        #[method(indexOfObjectAtIndexes:options:passingTest:)]
        pub unsafe fn indexOfObjectAtIndexes_options_passingTest(
            &self,
            s: &NSIndexSet,
            opts: NSEnumerationOptions,
            predicate: TodoBlock,
        ) -> NSUInteger;
        #[method_id(indexesOfObjectsPassingTest:)]
        pub unsafe fn indexesOfObjectsPassingTest(
            &self,
            predicate: TodoBlock,
        ) -> Id<NSIndexSet, Shared>;
        #[method_id(indexesOfObjectsWithOptions:passingTest:)]
        pub unsafe fn indexesOfObjectsWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: TodoBlock,
        ) -> Id<NSIndexSet, Shared>;
        #[method_id(indexesOfObjectsAtIndexes:options:passingTest:)]
        pub unsafe fn indexesOfObjectsAtIndexes_options_passingTest(
            &self,
            s: &NSIndexSet,
            opts: NSEnumerationOptions,
            predicate: TodoBlock,
        ) -> Id<NSIndexSet, Shared>;
        #[method(indexOfObject:inSortedRange:options:usingComparator:)]
        pub unsafe fn indexOfObject_inSortedRange_options_usingComparator(
            &self,
            object: &ObjectType,
            range: NSRange,
            opts: NSBinarySearchingOptions,
            cmp: NSComparator,
        ) -> NSUInteger;
        #[method_id(sortedArrayUsingComparator:)]
        pub unsafe fn sortedArrayUsingComparator(
            &self,
            cmptr: NSComparator,
        ) -> Id<NSArray<ObjectType>, Shared>;
        #[method_id(sortedArrayWithOptions:usingComparator:)]
        pub unsafe fn sortedArrayWithOptions_usingComparator(
            &self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        ) -> Id<NSArray<ObjectType>, Shared>;
        #[method_id(description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;
        #[method_id(descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>)
            -> Id<NSString, Shared>;
        #[method_id(descriptionWithLocale:indent:)]
        pub unsafe fn descriptionWithLocale_indent(
            &self,
            locale: Option<&Object>,
            level: NSUInteger,
        ) -> Id<NSString, Shared>;
    }
);
extern_methods!(
    #[doc = "NSOrderedSetCreation"]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[method_id(orderedSet)]
        pub unsafe fn orderedSet() -> Id<Self, Shared>;
        #[method_id(orderedSetWithObject:)]
        pub unsafe fn orderedSetWithObject(object: &ObjectType) -> Id<Self, Shared>;
        #[method_id(orderedSetWithObjects:count:)]
        pub unsafe fn orderedSetWithObjects_count(
            objects: TodoArray,
            cnt: NSUInteger,
        ) -> Id<Self, Shared>;
        #[method_id(orderedSetWithOrderedSet:)]
        pub unsafe fn orderedSetWithOrderedSet(set: &NSOrderedSet<ObjectType>) -> Id<Self, Shared>;
        #[method_id(orderedSetWithOrderedSet:range:copyItems:)]
        pub unsafe fn orderedSetWithOrderedSet_range_copyItems(
            set: &NSOrderedSet<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self, Shared>;
        #[method_id(orderedSetWithArray:)]
        pub unsafe fn orderedSetWithArray(array: &NSArray<ObjectType>) -> Id<Self, Shared>;
        #[method_id(orderedSetWithArray:range:copyItems:)]
        pub unsafe fn orderedSetWithArray_range_copyItems(
            array: &NSArray<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self, Shared>;
        #[method_id(orderedSetWithSet:)]
        pub unsafe fn orderedSetWithSet(set: &NSSet<ObjectType>) -> Id<Self, Shared>;
        #[method_id(orderedSetWithSet:copyItems:)]
        pub unsafe fn orderedSetWithSet_copyItems(
            set: &NSSet<ObjectType>,
            flag: bool,
        ) -> Id<Self, Shared>;
        #[method_id(initWithObject:)]
        pub unsafe fn initWithObject(&self, object: &ObjectType) -> Id<Self, Shared>;
        #[method_id(initWithOrderedSet:)]
        pub unsafe fn initWithOrderedSet(&self, set: &NSOrderedSet<ObjectType>)
            -> Id<Self, Shared>;
        #[method_id(initWithOrderedSet:copyItems:)]
        pub unsafe fn initWithOrderedSet_copyItems(
            &self,
            set: &NSOrderedSet<ObjectType>,
            flag: bool,
        ) -> Id<Self, Shared>;
        #[method_id(initWithOrderedSet:range:copyItems:)]
        pub unsafe fn initWithOrderedSet_range_copyItems(
            &self,
            set: &NSOrderedSet<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self, Shared>;
        #[method_id(initWithArray:)]
        pub unsafe fn initWithArray(&self, array: &NSArray<ObjectType>) -> Id<Self, Shared>;
        #[method_id(initWithArray:copyItems:)]
        pub unsafe fn initWithArray_copyItems(
            &self,
            set: &NSArray<ObjectType>,
            flag: bool,
        ) -> Id<Self, Shared>;
        #[method_id(initWithArray:range:copyItems:)]
        pub unsafe fn initWithArray_range_copyItems(
            &self,
            set: &NSArray<ObjectType>,
            range: NSRange,
            flag: bool,
        ) -> Id<Self, Shared>;
        #[method_id(initWithSet:)]
        pub unsafe fn initWithSet(&self, set: &NSSet<ObjectType>) -> Id<Self, Shared>;
        #[method_id(initWithSet:copyItems:)]
        pub unsafe fn initWithSet_copyItems(
            &self,
            set: &NSSet<ObjectType>,
            flag: bool,
        ) -> Id<Self, Shared>;
    }
);
extern_methods!(
    #[doc = "NSOrderedSetDiffing"]
    unsafe impl<ObjectType: Message> NSOrderedSet<ObjectType> {
        #[method_id(differenceFromOrderedSet:withOptions:usingEquivalenceTest:)]
        pub unsafe fn differenceFromOrderedSet_withOptions_usingEquivalenceTest(
            &self,
            other: &NSOrderedSet<ObjectType>,
            options: NSOrderedCollectionDifferenceCalculationOptions,
            block: TodoBlock,
        ) -> Id<NSOrderedCollectionDifference<ObjectType>, Shared>;
        #[method_id(differenceFromOrderedSet:withOptions:)]
        pub unsafe fn differenceFromOrderedSet_withOptions(
            &self,
            other: &NSOrderedSet<ObjectType>,
            options: NSOrderedCollectionDifferenceCalculationOptions,
        ) -> Id<NSOrderedCollectionDifference<ObjectType>, Shared>;
        #[method_id(differenceFromOrderedSet:)]
        pub unsafe fn differenceFromOrderedSet(
            &self,
            other: &NSOrderedSet<ObjectType>,
        ) -> Id<NSOrderedCollectionDifference<ObjectType>, Shared>;
        #[method_id(orderedSetByApplyingDifference:)]
        pub unsafe fn orderedSetByApplyingDifference(
            &self,
            difference: &NSOrderedCollectionDifference<ObjectType>,
        ) -> Option<Id<NSOrderedSet<ObjectType>, Shared>>;
    }
);
__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSMutableOrderedSet<ObjectType: Message>;
    unsafe impl<ObjectType: Message> ClassType for NSMutableOrderedSet<ObjectType> {
        type Super = NSOrderedSet;
    }
);
extern_methods!(
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method(insertObject:atIndex:)]
        pub unsafe fn insertObject_atIndex(&self, object: &ObjectType, idx: NSUInteger);
        #[method(removeObjectAtIndex:)]
        pub unsafe fn removeObjectAtIndex(&self, idx: NSUInteger);
        #[method(replaceObjectAtIndex:withObject:)]
        pub unsafe fn replaceObjectAtIndex_withObject(&self, idx: NSUInteger, object: &ObjectType);
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithCapacity:)]
        pub unsafe fn initWithCapacity(&self, numItems: NSUInteger) -> Id<Self, Shared>;
    }
);
extern_methods!(
    #[doc = "NSExtendedMutableOrderedSet"]
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method(addObject:)]
        pub unsafe fn addObject(&self, object: &ObjectType);
        #[method(addObjects:count:)]
        pub unsafe fn addObjects_count(&self, objects: TodoArray, count: NSUInteger);
        #[method(addObjectsFromArray:)]
        pub unsafe fn addObjectsFromArray(&self, array: &NSArray<ObjectType>);
        #[method(exchangeObjectAtIndex:withObjectAtIndex:)]
        pub unsafe fn exchangeObjectAtIndex_withObjectAtIndex(
            &self,
            idx1: NSUInteger,
            idx2: NSUInteger,
        );
        #[method(moveObjectsAtIndexes:toIndex:)]
        pub unsafe fn moveObjectsAtIndexes_toIndex(&self, indexes: &NSIndexSet, idx: NSUInteger);
        #[method(insertObjects:atIndexes:)]
        pub unsafe fn insertObjects_atIndexes(
            &self,
            objects: &NSArray<ObjectType>,
            indexes: &NSIndexSet,
        );
        #[method(setObject:atIndex:)]
        pub unsafe fn setObject_atIndex(&self, obj: &ObjectType, idx: NSUInteger);
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(&self, obj: &ObjectType, idx: NSUInteger);
        #[method(replaceObjectsInRange:withObjects:count:)]
        pub unsafe fn replaceObjectsInRange_withObjects_count(
            &self,
            range: NSRange,
            objects: TodoArray,
            count: NSUInteger,
        );
        #[method(replaceObjectsAtIndexes:withObjects:)]
        pub unsafe fn replaceObjectsAtIndexes_withObjects(
            &self,
            indexes: &NSIndexSet,
            objects: &NSArray<ObjectType>,
        );
        #[method(removeObjectsInRange:)]
        pub unsafe fn removeObjectsInRange(&self, range: NSRange);
        #[method(removeObjectsAtIndexes:)]
        pub unsafe fn removeObjectsAtIndexes(&self, indexes: &NSIndexSet);
        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);
        #[method(removeObject:)]
        pub unsafe fn removeObject(&self, object: &ObjectType);
        #[method(removeObjectsInArray:)]
        pub unsafe fn removeObjectsInArray(&self, array: &NSArray<ObjectType>);
        #[method(intersectOrderedSet:)]
        pub unsafe fn intersectOrderedSet(&self, other: &NSOrderedSet<ObjectType>);
        #[method(minusOrderedSet:)]
        pub unsafe fn minusOrderedSet(&self, other: &NSOrderedSet<ObjectType>);
        #[method(unionOrderedSet:)]
        pub unsafe fn unionOrderedSet(&self, other: &NSOrderedSet<ObjectType>);
        #[method(intersectSet:)]
        pub unsafe fn intersectSet(&self, other: &NSSet<ObjectType>);
        #[method(minusSet:)]
        pub unsafe fn minusSet(&self, other: &NSSet<ObjectType>);
        #[method(unionSet:)]
        pub unsafe fn unionSet(&self, other: &NSSet<ObjectType>);
        #[method(sortUsingComparator:)]
        pub unsafe fn sortUsingComparator(&self, cmptr: NSComparator);
        #[method(sortWithOptions:usingComparator:)]
        pub unsafe fn sortWithOptions_usingComparator(
            &self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        );
        #[method(sortRange:options:usingComparator:)]
        pub unsafe fn sortRange_options_usingComparator(
            &self,
            range: NSRange,
            opts: NSSortOptions,
            cmptr: NSComparator,
        );
    }
);
extern_methods!(
    #[doc = "NSMutableOrderedSetCreation"]
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method_id(orderedSetWithCapacity:)]
        pub unsafe fn orderedSetWithCapacity(numItems: NSUInteger) -> Id<Self, Shared>;
    }
);
extern_methods!(
    #[doc = "NSMutableOrderedSetDiffing"]
    unsafe impl<ObjectType: Message> NSMutableOrderedSet<ObjectType> {
        #[method(applyDifference:)]
        pub unsafe fn applyDifference(
            &self,
            difference: &NSOrderedCollectionDifference<ObjectType>,
        );
    }
);
