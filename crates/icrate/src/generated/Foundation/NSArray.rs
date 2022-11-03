//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSArray<ObjectType: Message = Object> {
        _inner0: PhantomData<*mut ObjectType>,
    }

    unsafe impl<ObjectType: Message> ClassType for NSArray<ObjectType> {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other objectAtIndex:)]
        pub unsafe fn objectAtIndex(&self, index: NSUInteger) -> Id<ObjectType, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithObjects:count:)]
        pub unsafe fn initWithObjects_count(
            this: Option<Allocated<Self>>,
            objects: *mut NonNull<ObjectType>,
            cnt: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSBinarySearchingOptions {
        NSBinarySearchingFirstEqual = 1 << 8,
        NSBinarySearchingLastEqual = 1 << 9,
        NSBinarySearchingInsertionIndex = 1 << 10,
    }
);

extern_methods!(
    /// NSExtendedArray
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[method_id(@__retain_semantics Other arrayByAddingObject:)]
        pub unsafe fn arrayByAddingObject(
            &self,
            anObject: &ObjectType,
        ) -> Id<NSArray<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other arrayByAddingObjectsFromArray:)]
        pub unsafe fn arrayByAddingObjectsFromArray(
            &self,
            otherArray: &NSArray<ObjectType>,
        ) -> Id<NSArray<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other componentsJoinedByString:)]
        pub unsafe fn componentsJoinedByString(&self, separator: &NSString)
            -> Id<NSString, Shared>;

        #[method(containsObject:)]
        pub unsafe fn containsObject(&self, anObject: &ObjectType) -> bool;

        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>)
            -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other descriptionWithLocale:indent:)]
        pub unsafe fn descriptionWithLocale_indent(
            &self,
            locale: Option<&Object>,
            level: NSUInteger,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other firstObjectCommonWithArray:)]
        pub unsafe fn firstObjectCommonWithArray(
            &self,
            otherArray: &NSArray<ObjectType>,
        ) -> Option<Id<ObjectType, Shared>>;

        #[method(getObjects:range:)]
        pub unsafe fn getObjects_range(
            &self,
            objects: NonNull<NonNull<ObjectType>>,
            range: NSRange,
        );

        #[method(indexOfObject:)]
        pub unsafe fn indexOfObject(&self, anObject: &ObjectType) -> NSUInteger;

        #[method(indexOfObject:inRange:)]
        pub unsafe fn indexOfObject_inRange(
            &self,
            anObject: &ObjectType,
            range: NSRange,
        ) -> NSUInteger;

        #[method(indexOfObjectIdenticalTo:)]
        pub unsafe fn indexOfObjectIdenticalTo(&self, anObject: &ObjectType) -> NSUInteger;

        #[method(indexOfObjectIdenticalTo:inRange:)]
        pub unsafe fn indexOfObjectIdenticalTo_inRange(
            &self,
            anObject: &ObjectType,
            range: NSRange,
        ) -> NSUInteger;

        #[method(isEqualToArray:)]
        pub unsafe fn isEqualToArray(&self, otherArray: &NSArray<ObjectType>) -> bool;

        #[method_id(@__retain_semantics Other firstObject)]
        pub unsafe fn firstObject(&self) -> Option<Id<ObjectType, Shared>>;

        #[method_id(@__retain_semantics Other lastObject)]
        pub unsafe fn lastObject(&self) -> Option<Id<ObjectType, Shared>>;

        #[method_id(@__retain_semantics Other objectEnumerator)]
        pub unsafe fn objectEnumerator(&self) -> Id<NSEnumerator<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other reverseObjectEnumerator)]
        pub unsafe fn reverseObjectEnumerator(&self) -> Id<NSEnumerator<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other sortedArrayHint)]
        pub unsafe fn sortedArrayHint(&self) -> Id<NSData, Shared>;

        #[method_id(@__retain_semantics Other sortedArrayUsingFunction:context:)]
        pub unsafe fn sortedArrayUsingFunction_context(
            &self,
            comparator: unsafe extern "C" fn(
                NonNull<ObjectType>,
                NonNull<ObjectType>,
                *mut c_void,
            ) -> NSInteger,
            context: *mut c_void,
        ) -> Id<NSArray<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other sortedArrayUsingFunction:context:hint:)]
        pub unsafe fn sortedArrayUsingFunction_context_hint(
            &self,
            comparator: unsafe extern "C" fn(
                NonNull<ObjectType>,
                NonNull<ObjectType>,
                *mut c_void,
            ) -> NSInteger,
            context: *mut c_void,
            hint: Option<&NSData>,
        ) -> Id<NSArray<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other sortedArrayUsingSelector:)]
        pub unsafe fn sortedArrayUsingSelector(
            &self,
            comparator: Sel,
        ) -> Id<NSArray<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other subarrayWithRange:)]
        pub unsafe fn subarrayWithRange(&self, range: NSRange) -> Id<NSArray<ObjectType>, Shared>;

        #[method(writeToURL:error:)]
        pub unsafe fn writeToURL_error(&self, url: &NSURL) -> Result<(), Id<NSError, Shared>>;

        #[method(makeObjectsPerformSelector:)]
        pub unsafe fn makeObjectsPerformSelector(&self, aSelector: Sel);

        #[method(makeObjectsPerformSelector:withObject:)]
        pub unsafe fn makeObjectsPerformSelector_withObject(
            &self,
            aSelector: Sel,
            argument: Option<&Object>,
        );

        #[method_id(@__retain_semantics Other objectsAtIndexes:)]
        pub unsafe fn objectsAtIndexes(
            &self,
            indexes: &NSIndexSet,
        ) -> Id<NSArray<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(&self, idx: NSUInteger) -> Id<ObjectType, Shared>;

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

        #[method_id(@__retain_semantics Other indexesOfObjectsPassingTest:)]
        pub unsafe fn indexesOfObjectsPassingTest(
            &self,
            predicate: TodoBlock,
        ) -> Id<NSIndexSet, Shared>;

        #[method_id(@__retain_semantics Other indexesOfObjectsWithOptions:passingTest:)]
        pub unsafe fn indexesOfObjectsWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: TodoBlock,
        ) -> Id<NSIndexSet, Shared>;

        #[method_id(@__retain_semantics Other indexesOfObjectsAtIndexes:options:passingTest:)]
        pub unsafe fn indexesOfObjectsAtIndexes_options_passingTest(
            &self,
            s: &NSIndexSet,
            opts: NSEnumerationOptions,
            predicate: TodoBlock,
        ) -> Id<NSIndexSet, Shared>;

        #[method_id(@__retain_semantics Other sortedArrayUsingComparator:)]
        pub unsafe fn sortedArrayUsingComparator(
            &self,
            cmptr: NSComparator,
        ) -> Id<NSArray<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other sortedArrayWithOptions:usingComparator:)]
        pub unsafe fn sortedArrayWithOptions_usingComparator(
            &self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        ) -> Id<NSArray<ObjectType>, Shared>;

        #[method(indexOfObject:inSortedRange:options:usingComparator:)]
        pub unsafe fn indexOfObject_inSortedRange_options_usingComparator(
            &self,
            obj: &ObjectType,
            r: NSRange,
            opts: NSBinarySearchingOptions,
            cmp: NSComparator,
        ) -> NSUInteger;
    }
);

extern_methods!(
    /// NSArrayCreation
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[method_id(@__retain_semantics Other array)]
        pub unsafe fn array() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other arrayWithObject:)]
        pub unsafe fn arrayWithObject(anObject: &ObjectType) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other arrayWithObjects:count:)]
        pub unsafe fn arrayWithObjects_count(
            objects: NonNull<NonNull<ObjectType>>,
            cnt: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other arrayWithArray:)]
        pub unsafe fn arrayWithArray(array: &NSArray<ObjectType>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithArray:)]
        pub unsafe fn initWithArray(
            this: Option<Allocated<Self>>,
            array: &NSArray<ObjectType>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithArray:copyItems:)]
        pub unsafe fn initWithArray_copyItems(
            this: Option<Allocated<Self>>,
            array: &NSArray<ObjectType>,
            flag: bool,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:error:)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Result<Id<NSArray<ObjectType>, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other arrayWithContentsOfURL:error:)]
        pub unsafe fn arrayWithContentsOfURL_error(
            url: &NSURL,
        ) -> Result<Id<NSArray<ObjectType>, Shared>, Id<NSError, Shared>>;
    }
);

extern_methods!(
    /// NSArrayDiffing
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[method_id(@__retain_semantics Other differenceFromArray:withOptions:usingEquivalenceTest:)]
        pub unsafe fn differenceFromArray_withOptions_usingEquivalenceTest(
            &self,
            other: &NSArray<ObjectType>,
            options: NSOrderedCollectionDifferenceCalculationOptions,
            block: TodoBlock,
        ) -> Id<NSOrderedCollectionDifference<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other differenceFromArray:withOptions:)]
        pub unsafe fn differenceFromArray_withOptions(
            &self,
            other: &NSArray<ObjectType>,
            options: NSOrderedCollectionDifferenceCalculationOptions,
        ) -> Id<NSOrderedCollectionDifference<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other differenceFromArray:)]
        pub unsafe fn differenceFromArray(
            &self,
            other: &NSArray<ObjectType>,
        ) -> Id<NSOrderedCollectionDifference<ObjectType>, Shared>;

        #[method_id(@__retain_semantics Other arrayByApplyingDifference:)]
        pub unsafe fn arrayByApplyingDifference(
            &self,
            difference: &NSOrderedCollectionDifference<ObjectType>,
        ) -> Option<Id<NSArray<ObjectType>, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl<ObjectType: Message> NSArray<ObjectType> {
        #[method(getObjects:)]
        pub unsafe fn getObjects(&self, objects: NonNull<NonNull<ObjectType>>);

        #[method_id(@__retain_semantics Other arrayWithContentsOfFile:)]
        pub unsafe fn arrayWithContentsOfFile(
            path: &NSString,
        ) -> Option<Id<NSArray<ObjectType>, Shared>>;

        #[method_id(@__retain_semantics Other arrayWithContentsOfURL:)]
        pub unsafe fn arrayWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Id<NSArray<ObjectType>, Shared>>;

        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Option<Id<NSArray<ObjectType>, Shared>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Option<Id<NSArray<ObjectType>, Shared>>;

        #[method(writeToFile:atomically:)]
        pub unsafe fn writeToFile_atomically(
            &self,
            path: &NSString,
            useAuxiliaryFile: bool,
        ) -> bool;

        #[method(writeToURL:atomically:)]
        pub unsafe fn writeToURL_atomically(&self, url: &NSURL, atomically: bool) -> bool;
    }
);

__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSMutableArray<ObjectType: Message = Object> {
        _inner0: PhantomData<*mut ObjectType>,
    }

    unsafe impl<ObjectType: Message> ClassType for NSMutableArray<ObjectType> {
        type Super = NSArray;
    }
);

extern_methods!(
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[method(addObject:)]
        pub unsafe fn addObject(&self, anObject: &ObjectType);

        #[method(insertObject:atIndex:)]
        pub unsafe fn insertObject_atIndex(&self, anObject: &ObjectType, index: NSUInteger);

        #[method(removeLastObject)]
        pub unsafe fn removeLastObject(&self);

        #[method(removeObjectAtIndex:)]
        pub unsafe fn removeObjectAtIndex(&self, index: NSUInteger);

        #[method(replaceObjectAtIndex:withObject:)]
        pub unsafe fn replaceObjectAtIndex_withObject(
            &self,
            index: NSUInteger,
            anObject: &ObjectType,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub unsafe fn initWithCapacity(
            this: Option<Allocated<Self>>,
            numItems: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSExtendedMutableArray
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[method(addObjectsFromArray:)]
        pub unsafe fn addObjectsFromArray(&self, otherArray: &NSArray<ObjectType>);

        #[method(exchangeObjectAtIndex:withObjectAtIndex:)]
        pub unsafe fn exchangeObjectAtIndex_withObjectAtIndex(
            &self,
            idx1: NSUInteger,
            idx2: NSUInteger,
        );

        #[method(removeAllObjects)]
        pub unsafe fn removeAllObjects(&self);

        #[method(removeObject:inRange:)]
        pub unsafe fn removeObject_inRange(&self, anObject: &ObjectType, range: NSRange);

        #[method(removeObject:)]
        pub unsafe fn removeObject(&self, anObject: &ObjectType);

        #[method(removeObjectIdenticalTo:inRange:)]
        pub unsafe fn removeObjectIdenticalTo_inRange(&self, anObject: &ObjectType, range: NSRange);

        #[method(removeObjectIdenticalTo:)]
        pub unsafe fn removeObjectIdenticalTo(&self, anObject: &ObjectType);

        #[method(removeObjectsFromIndices:numIndices:)]
        pub unsafe fn removeObjectsFromIndices_numIndices(
            &self,
            indices: NonNull<NSUInteger>,
            cnt: NSUInteger,
        );

        #[method(removeObjectsInArray:)]
        pub unsafe fn removeObjectsInArray(&self, otherArray: &NSArray<ObjectType>);

        #[method(removeObjectsInRange:)]
        pub unsafe fn removeObjectsInRange(&self, range: NSRange);

        #[method(replaceObjectsInRange:withObjectsFromArray:range:)]
        pub unsafe fn replaceObjectsInRange_withObjectsFromArray_range(
            &self,
            range: NSRange,
            otherArray: &NSArray<ObjectType>,
            otherRange: NSRange,
        );

        #[method(replaceObjectsInRange:withObjectsFromArray:)]
        pub unsafe fn replaceObjectsInRange_withObjectsFromArray(
            &self,
            range: NSRange,
            otherArray: &NSArray<ObjectType>,
        );

        #[method(setArray:)]
        pub unsafe fn setArray(&self, otherArray: &NSArray<ObjectType>);

        #[method(sortUsingFunction:context:)]
        pub unsafe fn sortUsingFunction_context(
            &self,
            compare: unsafe extern "C" fn(
                NonNull<ObjectType>,
                NonNull<ObjectType>,
                *mut c_void,
            ) -> NSInteger,
            context: *mut c_void,
        );

        #[method(sortUsingSelector:)]
        pub unsafe fn sortUsingSelector(&self, comparator: Sel);

        #[method(insertObjects:atIndexes:)]
        pub unsafe fn insertObjects_atIndexes(
            &self,
            objects: &NSArray<ObjectType>,
            indexes: &NSIndexSet,
        );

        #[method(removeObjectsAtIndexes:)]
        pub unsafe fn removeObjectsAtIndexes(&self, indexes: &NSIndexSet);

        #[method(replaceObjectsAtIndexes:withObjects:)]
        pub unsafe fn replaceObjectsAtIndexes_withObjects(
            &self,
            indexes: &NSIndexSet,
            objects: &NSArray<ObjectType>,
        );

        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(&self, obj: &ObjectType, idx: NSUInteger);

        #[method(sortUsingComparator:)]
        pub unsafe fn sortUsingComparator(&self, cmptr: NSComparator);

        #[method(sortWithOptions:usingComparator:)]
        pub unsafe fn sortWithOptions_usingComparator(
            &self,
            opts: NSSortOptions,
            cmptr: NSComparator,
        );
    }
);

extern_methods!(
    /// NSMutableArrayCreation
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[method_id(@__retain_semantics Other arrayWithCapacity:)]
        pub unsafe fn arrayWithCapacity(numItems: NSUInteger) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other arrayWithContentsOfFile:)]
        pub unsafe fn arrayWithContentsOfFile(
            path: &NSString,
        ) -> Option<Id<NSMutableArray<ObjectType>, Shared>>;

        #[method_id(@__retain_semantics Other arrayWithContentsOfURL:)]
        pub unsafe fn arrayWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Id<NSMutableArray<ObjectType>, Shared>>;

        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Option<Id<NSMutableArray<ObjectType>, Shared>>;

        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Option<Id<NSMutableArray<ObjectType>, Shared>>;
    }
);

extern_methods!(
    /// NSMutableArrayDiffing
    unsafe impl<ObjectType: Message> NSMutableArray<ObjectType> {
        #[method(applyDifference:)]
        pub unsafe fn applyDifference(
            &self,
            difference: &NSOrderedCollectionDifference<ObjectType>,
        );
    }
);
