use super::NSArray;
use super::NSIndexSet;
use super::NSSet;
use super::NSString;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSEnumerator::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSOrderedCollectionDifference::*;
use crate::Foundation::generated::NSRange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSOrderedSet;
    unsafe impl ClassType for NSOrderedSet {
        type Super = NSObject;
    }
);
impl NSOrderedSet {
    pub unsafe fn objectAtIndex(&self, idx: NSUInteger) -> ObjectType {
        msg_send![self, objectAtIndex: idx]
    }
    pub unsafe fn indexOfObject(&self, object: ObjectType) -> NSUInteger {
        msg_send![self, indexOfObject: object]
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
#[doc = "NSExtendedOrderedSet"]
impl NSOrderedSet {
    pub unsafe fn getObjects_range(&self, objects: TodoArray, range: NSRange) {
        msg_send![self, getObjects: objects, range: range]
    }
    pub unsafe fn objectsAtIndexes(&self, indexes: &NSIndexSet) -> TodoGenerics {
        msg_send![self, objectsAtIndexes: indexes]
    }
    pub unsafe fn isEqualToOrderedSet(&self, other: TodoGenerics) -> bool {
        msg_send![self, isEqualToOrderedSet: other]
    }
    pub unsafe fn containsObject(&self, object: ObjectType) -> bool {
        msg_send![self, containsObject: object]
    }
    pub unsafe fn intersectsOrderedSet(&self, other: TodoGenerics) -> bool {
        msg_send![self, intersectsOrderedSet: other]
    }
    pub unsafe fn intersectsSet(&self, set: TodoGenerics) -> bool {
        msg_send![self, intersectsSet: set]
    }
    pub unsafe fn isSubsetOfOrderedSet(&self, other: TodoGenerics) -> bool {
        msg_send![self, isSubsetOfOrderedSet: other]
    }
    pub unsafe fn isSubsetOfSet(&self, set: TodoGenerics) -> bool {
        msg_send![self, isSubsetOfSet: set]
    }
    pub unsafe fn objectAtIndexedSubscript(&self, idx: NSUInteger) -> ObjectType {
        msg_send![self, objectAtIndexedSubscript: idx]
    }
    pub unsafe fn objectEnumerator(&self) -> TodoGenerics {
        msg_send![self, objectEnumerator]
    }
    pub unsafe fn reverseObjectEnumerator(&self) -> TodoGenerics {
        msg_send![self, reverseObjectEnumerator]
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
    pub unsafe fn enumerateObjectsAtIndexes_options_usingBlock(
        &self,
        s: &NSIndexSet,
        opts: NSEnumerationOptions,
        block: TodoBlock,
    ) {
        msg_send![
            self,
            enumerateObjectsAtIndexes: s,
            options: opts,
            usingBlock: block
        ]
    }
    pub unsafe fn indexOfObjectPassingTest(&self, predicate: TodoBlock) -> NSUInteger {
        msg_send![self, indexOfObjectPassingTest: predicate]
    }
    pub unsafe fn indexOfObjectWithOptions_passingTest(
        &self,
        opts: NSEnumerationOptions,
        predicate: TodoBlock,
    ) -> NSUInteger {
        msg_send![self, indexOfObjectWithOptions: opts, passingTest: predicate]
    }
    pub unsafe fn indexOfObjectAtIndexes_options_passingTest(
        &self,
        s: &NSIndexSet,
        opts: NSEnumerationOptions,
        predicate: TodoBlock,
    ) -> NSUInteger {
        msg_send![
            self,
            indexOfObjectAtIndexes: s,
            options: opts,
            passingTest: predicate
        ]
    }
    pub unsafe fn indexesOfObjectsPassingTest(
        &self,
        predicate: TodoBlock,
    ) -> Id<NSIndexSet, Shared> {
        msg_send_id![self, indexesOfObjectsPassingTest: predicate]
    }
    pub unsafe fn indexesOfObjectsWithOptions_passingTest(
        &self,
        opts: NSEnumerationOptions,
        predicate: TodoBlock,
    ) -> Id<NSIndexSet, Shared> {
        msg_send_id![
            self,
            indexesOfObjectsWithOptions: opts,
            passingTest: predicate
        ]
    }
    pub unsafe fn indexesOfObjectsAtIndexes_options_passingTest(
        &self,
        s: &NSIndexSet,
        opts: NSEnumerationOptions,
        predicate: TodoBlock,
    ) -> Id<NSIndexSet, Shared> {
        msg_send_id![
            self,
            indexesOfObjectsAtIndexes: s,
            options: opts,
            passingTest: predicate
        ]
    }
    pub unsafe fn indexOfObject_inSortedRange_options_usingComparator(
        &self,
        object: ObjectType,
        range: NSRange,
        opts: NSBinarySearchingOptions,
        cmp: NSComparator,
    ) -> NSUInteger {
        msg_send![
            self,
            indexOfObject: object,
            inSortedRange: range,
            options: opts,
            usingComparator: cmp
        ]
    }
    pub unsafe fn sortedArrayUsingComparator(&self, cmptr: NSComparator) -> TodoGenerics {
        msg_send![self, sortedArrayUsingComparator: cmptr]
    }
    pub unsafe fn sortedArrayWithOptions_usingComparator(
        &self,
        opts: NSSortOptions,
        cmptr: NSComparator,
    ) -> TodoGenerics {
        msg_send![self, sortedArrayWithOptions: opts, usingComparator: cmptr]
    }
    pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>) -> Id<NSString, Shared> {
        msg_send_id![self, descriptionWithLocale: locale]
    }
    pub unsafe fn descriptionWithLocale_indent(
        &self,
        locale: Option<&Object>,
        level: NSUInteger,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, descriptionWithLocale: locale, indent: level]
    }
    pub unsafe fn firstObject(&self) -> ObjectType {
        msg_send![self, firstObject]
    }
    pub unsafe fn lastObject(&self) -> ObjectType {
        msg_send![self, lastObject]
    }
    pub unsafe fn reversedOrderedSet(&self) -> TodoGenerics {
        msg_send![self, reversedOrderedSet]
    }
    pub unsafe fn array(&self) -> TodoGenerics {
        msg_send![self, array]
    }
    pub unsafe fn set(&self) -> TodoGenerics {
        msg_send![self, set]
    }
    pub unsafe fn description(&self) -> Id<NSString, Shared> {
        msg_send_id![self, description]
    }
}
#[doc = "NSOrderedSetCreation"]
impl NSOrderedSet {
    pub unsafe fn orderedSet() -> Id<Self, Shared> {
        msg_send_id![Self::class(), orderedSet]
    }
    pub unsafe fn orderedSetWithObject(object: ObjectType) -> Id<Self, Shared> {
        msg_send_id![Self::class(), orderedSetWithObject: object]
    }
    pub unsafe fn orderedSetWithObjects_count(
        objects: TodoArray,
        cnt: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![Self::class(), orderedSetWithObjects: objects, count: cnt]
    }
    pub unsafe fn orderedSetWithOrderedSet(set: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![Self::class(), orderedSetWithOrderedSet: set]
    }
    pub unsafe fn orderedSetWithOrderedSet_range_copyItems(
        set: TodoGenerics,
        range: NSRange,
        flag: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            orderedSetWithOrderedSet: set,
            range: range,
            copyItems: flag
        ]
    }
    pub unsafe fn orderedSetWithArray(array: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![Self::class(), orderedSetWithArray: array]
    }
    pub unsafe fn orderedSetWithArray_range_copyItems(
        array: TodoGenerics,
        range: NSRange,
        flag: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            orderedSetWithArray: array,
            range: range,
            copyItems: flag
        ]
    }
    pub unsafe fn orderedSetWithSet(set: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![Self::class(), orderedSetWithSet: set]
    }
    pub unsafe fn orderedSetWithSet_copyItems(set: TodoGenerics, flag: bool) -> Id<Self, Shared> {
        msg_send_id![Self::class(), orderedSetWithSet: set, copyItems: flag]
    }
    pub unsafe fn initWithObject(&self, object: ObjectType) -> Id<Self, Shared> {
        msg_send_id![self, initWithObject: object]
    }
    pub unsafe fn initWithOrderedSet(&self, set: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![self, initWithOrderedSet: set]
    }
    pub unsafe fn initWithOrderedSet_copyItems(
        &self,
        set: TodoGenerics,
        flag: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithOrderedSet: set, copyItems: flag]
    }
    pub unsafe fn initWithOrderedSet_range_copyItems(
        &self,
        set: TodoGenerics,
        range: NSRange,
        flag: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithOrderedSet: set, range: range, copyItems: flag]
    }
    pub unsafe fn initWithArray(&self, array: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![self, initWithArray: array]
    }
    pub unsafe fn initWithArray_copyItems(
        &self,
        set: TodoGenerics,
        flag: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithArray: set, copyItems: flag]
    }
    pub unsafe fn initWithArray_range_copyItems(
        &self,
        set: TodoGenerics,
        range: NSRange,
        flag: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithArray: set, range: range, copyItems: flag]
    }
    pub unsafe fn initWithSet(&self, set: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![self, initWithSet: set]
    }
    pub unsafe fn initWithSet_copyItems(&self, set: TodoGenerics, flag: bool) -> Id<Self, Shared> {
        msg_send_id![self, initWithSet: set, copyItems: flag]
    }
}
#[doc = "NSOrderedSetDiffing"]
impl NSOrderedSet {
    pub unsafe fn differenceFromOrderedSet_withOptions_usingEquivalenceTest(
        &self,
        other: TodoGenerics,
        options: NSOrderedCollectionDifferenceCalculationOptions,
        block: TodoBlock,
    ) -> TodoGenerics {
        msg_send![
            self,
            differenceFromOrderedSet: other,
            withOptions: options,
            usingEquivalenceTest: block
        ]
    }
    pub unsafe fn differenceFromOrderedSet_withOptions(
        &self,
        other: TodoGenerics,
        options: NSOrderedCollectionDifferenceCalculationOptions,
    ) -> TodoGenerics {
        msg_send![self, differenceFromOrderedSet: other, withOptions: options]
    }
    pub unsafe fn differenceFromOrderedSet(&self, other: TodoGenerics) -> TodoGenerics {
        msg_send![self, differenceFromOrderedSet: other]
    }
    pub unsafe fn orderedSetByApplyingDifference(&self, difference: TodoGenerics) -> TodoGenerics {
        msg_send![self, orderedSetByApplyingDifference: difference]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSMutableOrderedSet;
    unsafe impl ClassType for NSMutableOrderedSet {
        type Super = NSOrderedSet;
    }
);
impl NSMutableOrderedSet {
    pub unsafe fn insertObject_atIndex(&self, object: ObjectType, idx: NSUInteger) {
        msg_send![self, insertObject: object, atIndex: idx]
    }
    pub unsafe fn removeObjectAtIndex(&self, idx: NSUInteger) {
        msg_send![self, removeObjectAtIndex: idx]
    }
    pub unsafe fn replaceObjectAtIndex_withObject(&self, idx: NSUInteger, object: ObjectType) {
        msg_send![self, replaceObjectAtIndex: idx, withObject: object]
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
#[doc = "NSExtendedMutableOrderedSet"]
impl NSMutableOrderedSet {
    pub unsafe fn addObject(&self, object: ObjectType) {
        msg_send![self, addObject: object]
    }
    pub unsafe fn addObjects_count(&self, objects: TodoArray, count: NSUInteger) {
        msg_send![self, addObjects: objects, count: count]
    }
    pub unsafe fn addObjectsFromArray(&self, array: TodoGenerics) {
        msg_send![self, addObjectsFromArray: array]
    }
    pub unsafe fn exchangeObjectAtIndex_withObjectAtIndex(
        &self,
        idx1: NSUInteger,
        idx2: NSUInteger,
    ) {
        msg_send![self, exchangeObjectAtIndex: idx1, withObjectAtIndex: idx2]
    }
    pub unsafe fn moveObjectsAtIndexes_toIndex(&self, indexes: &NSIndexSet, idx: NSUInteger) {
        msg_send![self, moveObjectsAtIndexes: indexes, toIndex: idx]
    }
    pub unsafe fn insertObjects_atIndexes(&self, objects: TodoGenerics, indexes: &NSIndexSet) {
        msg_send![self, insertObjects: objects, atIndexes: indexes]
    }
    pub unsafe fn setObject_atIndex(&self, obj: ObjectType, idx: NSUInteger) {
        msg_send![self, setObject: obj, atIndex: idx]
    }
    pub unsafe fn setObject_atIndexedSubscript(&self, obj: ObjectType, idx: NSUInteger) {
        msg_send![self, setObject: obj, atIndexedSubscript: idx]
    }
    pub unsafe fn replaceObjectsInRange_withObjects_count(
        &self,
        range: NSRange,
        objects: TodoArray,
        count: NSUInteger,
    ) {
        msg_send![
            self,
            replaceObjectsInRange: range,
            withObjects: objects,
            count: count
        ]
    }
    pub unsafe fn replaceObjectsAtIndexes_withObjects(
        &self,
        indexes: &NSIndexSet,
        objects: TodoGenerics,
    ) {
        msg_send![self, replaceObjectsAtIndexes: indexes, withObjects: objects]
    }
    pub unsafe fn removeObjectsInRange(&self, range: NSRange) {
        msg_send![self, removeObjectsInRange: range]
    }
    pub unsafe fn removeObjectsAtIndexes(&self, indexes: &NSIndexSet) {
        msg_send![self, removeObjectsAtIndexes: indexes]
    }
    pub unsafe fn removeAllObjects(&self) {
        msg_send![self, removeAllObjects]
    }
    pub unsafe fn removeObject(&self, object: ObjectType) {
        msg_send![self, removeObject: object]
    }
    pub unsafe fn removeObjectsInArray(&self, array: TodoGenerics) {
        msg_send![self, removeObjectsInArray: array]
    }
    pub unsafe fn intersectOrderedSet(&self, other: TodoGenerics) {
        msg_send![self, intersectOrderedSet: other]
    }
    pub unsafe fn minusOrderedSet(&self, other: TodoGenerics) {
        msg_send![self, minusOrderedSet: other]
    }
    pub unsafe fn unionOrderedSet(&self, other: TodoGenerics) {
        msg_send![self, unionOrderedSet: other]
    }
    pub unsafe fn intersectSet(&self, other: TodoGenerics) {
        msg_send![self, intersectSet: other]
    }
    pub unsafe fn minusSet(&self, other: TodoGenerics) {
        msg_send![self, minusSet: other]
    }
    pub unsafe fn unionSet(&self, other: TodoGenerics) {
        msg_send![self, unionSet: other]
    }
    pub unsafe fn sortUsingComparator(&self, cmptr: NSComparator) {
        msg_send![self, sortUsingComparator: cmptr]
    }
    pub unsafe fn sortWithOptions_usingComparator(&self, opts: NSSortOptions, cmptr: NSComparator) {
        msg_send![self, sortWithOptions: opts, usingComparator: cmptr]
    }
    pub unsafe fn sortRange_options_usingComparator(
        &self,
        range: NSRange,
        opts: NSSortOptions,
        cmptr: NSComparator,
    ) {
        msg_send![
            self,
            sortRange: range,
            options: opts,
            usingComparator: cmptr
        ]
    }
}
#[doc = "NSMutableOrderedSetCreation"]
impl NSMutableOrderedSet {
    pub unsafe fn orderedSetWithCapacity(numItems: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![Self::class(), orderedSetWithCapacity: numItems]
    }
}
#[doc = "NSMutableOrderedSetDiffing"]
impl NSMutableOrderedSet {
    pub unsafe fn applyDifference(&self, difference: TodoGenerics) {
        msg_send![self, applyDifference: difference]
    }
}
