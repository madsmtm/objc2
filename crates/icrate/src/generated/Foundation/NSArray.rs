extern_class!(
    #[derive(Debug)]
    struct NSArray;
    unsafe impl ClassType for NSArray {
        type Super = NSObject;
    }
);
impl NSArray {
    pub unsafe fn objectAtIndex(&self, index: NSUInteger) -> ObjectType {
        msg_send![self, objectAtIndex: index]
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
#[doc = "NSExtendedArray"]
impl NSArray {
    pub unsafe fn arrayByAddingObject(&self, anObject: ObjectType) -> TodoGenerics {
        msg_send![self, arrayByAddingObject: anObject]
    }
    pub unsafe fn arrayByAddingObjectsFromArray(&self, otherArray: TodoGenerics) -> TodoGenerics {
        msg_send![self, arrayByAddingObjectsFromArray: otherArray]
    }
    pub unsafe fn componentsJoinedByString(&self, separator: &NSString) -> Id<NSString, Shared> {
        msg_send_id![self, componentsJoinedByString: separator]
    }
    pub unsafe fn containsObject(&self, anObject: ObjectType) -> bool {
        msg_send![self, containsObject: anObject]
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
    pub unsafe fn firstObjectCommonWithArray(&self, otherArray: TodoGenerics) -> ObjectType {
        msg_send![self, firstObjectCommonWithArray: otherArray]
    }
    pub unsafe fn getObjects_range(&self, objects: TodoArray, range: NSRange) {
        msg_send![self, getObjects: objects, range: range]
    }
    pub unsafe fn indexOfObject(&self, anObject: ObjectType) -> NSUInteger {
        msg_send![self, indexOfObject: anObject]
    }
    pub unsafe fn indexOfObject_inRange(&self, anObject: ObjectType, range: NSRange) -> NSUInteger {
        msg_send![self, indexOfObject: anObject, inRange: range]
    }
    pub unsafe fn indexOfObjectIdenticalTo(&self, anObject: ObjectType) -> NSUInteger {
        msg_send![self, indexOfObjectIdenticalTo: anObject]
    }
    pub unsafe fn indexOfObjectIdenticalTo_inRange(
        &self,
        anObject: ObjectType,
        range: NSRange,
    ) -> NSUInteger {
        msg_send![self, indexOfObjectIdenticalTo: anObject, inRange: range]
    }
    pub unsafe fn isEqualToArray(&self, otherArray: TodoGenerics) -> bool {
        msg_send![self, isEqualToArray: otherArray]
    }
    pub unsafe fn objectEnumerator(&self) -> TodoGenerics {
        msg_send![self, objectEnumerator]
    }
    pub unsafe fn reverseObjectEnumerator(&self) -> TodoGenerics {
        msg_send![self, reverseObjectEnumerator]
    }
    pub unsafe fn sortedArrayUsingFunction_context(
        &self,
        comparator: NonNull<TodoFunction>,
        context: *mut c_void,
    ) -> TodoGenerics {
        msg_send![self, sortedArrayUsingFunction: comparator, context: context]
    }
    pub unsafe fn sortedArrayUsingFunction_context_hint(
        &self,
        comparator: NonNull<TodoFunction>,
        context: *mut c_void,
        hint: Option<&NSData>,
    ) -> TodoGenerics {
        msg_send![
            self,
            sortedArrayUsingFunction: comparator,
            context: context,
            hint: hint
        ]
    }
    pub unsafe fn sortedArrayUsingSelector(&self, comparator: Sel) -> TodoGenerics {
        msg_send![self, sortedArrayUsingSelector: comparator]
    }
    pub unsafe fn subarrayWithRange(&self, range: NSRange) -> TodoGenerics {
        msg_send![self, subarrayWithRange: range]
    }
    pub unsafe fn writeToURL_error(&self, url: &NSURL, error: *mut Option<&NSError>) -> bool {
        msg_send![self, writeToURL: url, error: error]
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
    pub unsafe fn objectsAtIndexes(&self, indexes: &NSIndexSet) -> TodoGenerics {
        msg_send![self, objectsAtIndexes: indexes]
    }
    pub unsafe fn objectAtIndexedSubscript(&self, idx: NSUInteger) -> ObjectType {
        msg_send![self, objectAtIndexedSubscript: idx]
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
    pub unsafe fn indexOfObject_inSortedRange_options_usingComparator(
        &self,
        obj: ObjectType,
        r: NSRange,
        opts: NSBinarySearchingOptions,
        cmp: NSComparator,
    ) -> NSUInteger {
        msg_send![
            self,
            indexOfObject: obj,
            inSortedRange: r,
            options: opts,
            usingComparator: cmp
        ]
    }
    pub unsafe fn description(&self) -> Id<NSString, Shared> {
        msg_send_id![self, description]
    }
    pub unsafe fn firstObject(&self) -> ObjectType {
        msg_send![self, firstObject]
    }
    pub unsafe fn lastObject(&self) -> ObjectType {
        msg_send![self, lastObject]
    }
    pub unsafe fn sortedArrayHint(&self) -> Id<NSData, Shared> {
        msg_send_id![self, sortedArrayHint]
    }
}
#[doc = "NSArrayCreation"]
impl NSArray {
    pub unsafe fn array() -> Id<Self, Shared> {
        msg_send_id![Self::class(), array]
    }
    pub unsafe fn arrayWithObject(anObject: ObjectType) -> Id<Self, Shared> {
        msg_send_id![Self::class(), arrayWithObject: anObject]
    }
    pub unsafe fn arrayWithObjects_count(objects: TodoArray, cnt: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![Self::class(), arrayWithObjects: objects, count: cnt]
    }
    pub unsafe fn arrayWithArray(array: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![Self::class(), arrayWithArray: array]
    }
    pub unsafe fn initWithArray(&self, array: TodoGenerics) -> Id<Self, Shared> {
        msg_send_id![self, initWithArray: array]
    }
    pub unsafe fn initWithArray_copyItems(
        &self,
        array: TodoGenerics,
        flag: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithArray: array, copyItems: flag]
    }
    pub unsafe fn initWithContentsOfURL_error(
        &self,
        url: &NSURL,
        error: *mut Option<&NSError>,
    ) -> TodoGenerics {
        msg_send![self, initWithContentsOfURL: url, error: error]
    }
    pub unsafe fn arrayWithContentsOfURL_error(
        url: &NSURL,
        error: *mut Option<&NSError>,
    ) -> TodoGenerics {
        msg_send![Self::class(), arrayWithContentsOfURL: url, error: error]
    }
}
#[doc = "NSArrayDiffing"]
impl NSArray {
    pub unsafe fn differenceFromArray_withOptions_usingEquivalenceTest(
        &self,
        other: TodoGenerics,
        options: NSOrderedCollectionDifferenceCalculationOptions,
        block: TodoBlock,
    ) -> TodoGenerics {
        msg_send![
            self,
            differenceFromArray: other,
            withOptions: options,
            usingEquivalenceTest: block
        ]
    }
    pub unsafe fn differenceFromArray_withOptions(
        &self,
        other: TodoGenerics,
        options: NSOrderedCollectionDifferenceCalculationOptions,
    ) -> TodoGenerics {
        msg_send![self, differenceFromArray: other, withOptions: options]
    }
    pub unsafe fn differenceFromArray(&self, other: TodoGenerics) -> TodoGenerics {
        msg_send![self, differenceFromArray: other]
    }
    pub unsafe fn arrayByApplyingDifference(&self, difference: TodoGenerics) -> TodoGenerics {
        msg_send![self, arrayByApplyingDifference: difference]
    }
}
#[doc = "NSDeprecated"]
impl NSArray {
    pub unsafe fn getObjects(&self, objects: TodoArray) {
        msg_send![self, getObjects: objects]
    }
    pub unsafe fn arrayWithContentsOfFile(path: &NSString) -> TodoGenerics {
        msg_send![Self::class(), arrayWithContentsOfFile: path]
    }
    pub unsafe fn arrayWithContentsOfURL(url: &NSURL) -> TodoGenerics {
        msg_send![Self::class(), arrayWithContentsOfURL: url]
    }
    pub unsafe fn initWithContentsOfFile(&self, path: &NSString) -> TodoGenerics {
        msg_send![self, initWithContentsOfFile: path]
    }
    pub unsafe fn initWithContentsOfURL(&self, url: &NSURL) -> TodoGenerics {
        msg_send![self, initWithContentsOfURL: url]
    }
    pub unsafe fn writeToFile_atomically(&self, path: &NSString, useAuxiliaryFile: bool) -> bool {
        msg_send![self, writeToFile: path, atomically: useAuxiliaryFile]
    }
    pub unsafe fn writeToURL_atomically(&self, url: &NSURL, atomically: bool) -> bool {
        msg_send![self, writeToURL: url, atomically: atomically]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSMutableArray;
    unsafe impl ClassType for NSMutableArray {
        type Super = NSArray;
    }
);
impl NSMutableArray {
    pub unsafe fn addObject(&self, anObject: ObjectType) {
        msg_send![self, addObject: anObject]
    }
    pub unsafe fn insertObject_atIndex(&self, anObject: ObjectType, index: NSUInteger) {
        msg_send![self, insertObject: anObject, atIndex: index]
    }
    pub unsafe fn removeLastObject(&self) {
        msg_send![self, removeLastObject]
    }
    pub unsafe fn removeObjectAtIndex(&self, index: NSUInteger) {
        msg_send![self, removeObjectAtIndex: index]
    }
    pub unsafe fn replaceObjectAtIndex_withObject(&self, index: NSUInteger, anObject: ObjectType) {
        msg_send![self, replaceObjectAtIndex: index, withObject: anObject]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithCapacity(&self, numItems: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![self, initWithCapacity: numItems]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
}
#[doc = "NSExtendedMutableArray"]
impl NSMutableArray {
    pub unsafe fn addObjectsFromArray(&self, otherArray: TodoGenerics) {
        msg_send![self, addObjectsFromArray: otherArray]
    }
    pub unsafe fn exchangeObjectAtIndex_withObjectAtIndex(
        &self,
        idx1: NSUInteger,
        idx2: NSUInteger,
    ) {
        msg_send![self, exchangeObjectAtIndex: idx1, withObjectAtIndex: idx2]
    }
    pub unsafe fn removeAllObjects(&self) {
        msg_send![self, removeAllObjects]
    }
    pub unsafe fn removeObject_inRange(&self, anObject: ObjectType, range: NSRange) {
        msg_send![self, removeObject: anObject, inRange: range]
    }
    pub unsafe fn removeObject(&self, anObject: ObjectType) {
        msg_send![self, removeObject: anObject]
    }
    pub unsafe fn removeObjectIdenticalTo_inRange(&self, anObject: ObjectType, range: NSRange) {
        msg_send![self, removeObjectIdenticalTo: anObject, inRange: range]
    }
    pub unsafe fn removeObjectIdenticalTo(&self, anObject: ObjectType) {
        msg_send![self, removeObjectIdenticalTo: anObject]
    }
    pub unsafe fn removeObjectsFromIndices_numIndices(
        &self,
        indices: NonNull<NSUInteger>,
        cnt: NSUInteger,
    ) {
        msg_send![self, removeObjectsFromIndices: indices, numIndices: cnt]
    }
    pub unsafe fn removeObjectsInArray(&self, otherArray: TodoGenerics) {
        msg_send![self, removeObjectsInArray: otherArray]
    }
    pub unsafe fn removeObjectsInRange(&self, range: NSRange) {
        msg_send![self, removeObjectsInRange: range]
    }
    pub unsafe fn replaceObjectsInRange_withObjectsFromArray_range(
        &self,
        range: NSRange,
        otherArray: TodoGenerics,
        otherRange: NSRange,
    ) {
        msg_send![
            self,
            replaceObjectsInRange: range,
            withObjectsFromArray: otherArray,
            range: otherRange
        ]
    }
    pub unsafe fn replaceObjectsInRange_withObjectsFromArray(
        &self,
        range: NSRange,
        otherArray: TodoGenerics,
    ) {
        msg_send![
            self,
            replaceObjectsInRange: range,
            withObjectsFromArray: otherArray
        ]
    }
    pub unsafe fn setArray(&self, otherArray: TodoGenerics) {
        msg_send![self, setArray: otherArray]
    }
    pub unsafe fn sortUsingFunction_context(
        &self,
        compare: NonNull<TodoFunction>,
        context: *mut c_void,
    ) {
        msg_send![self, sortUsingFunction: compare, context: context]
    }
    pub unsafe fn sortUsingSelector(&self, comparator: Sel) {
        msg_send![self, sortUsingSelector: comparator]
    }
    pub unsafe fn insertObjects_atIndexes(&self, objects: TodoGenerics, indexes: &NSIndexSet) {
        msg_send![self, insertObjects: objects, atIndexes: indexes]
    }
    pub unsafe fn removeObjectsAtIndexes(&self, indexes: &NSIndexSet) {
        msg_send![self, removeObjectsAtIndexes: indexes]
    }
    pub unsafe fn replaceObjectsAtIndexes_withObjects(
        &self,
        indexes: &NSIndexSet,
        objects: TodoGenerics,
    ) {
        msg_send![self, replaceObjectsAtIndexes: indexes, withObjects: objects]
    }
    pub unsafe fn setObject_atIndexedSubscript(&self, obj: ObjectType, idx: NSUInteger) {
        msg_send![self, setObject: obj, atIndexedSubscript: idx]
    }
    pub unsafe fn sortUsingComparator(&self, cmptr: NSComparator) {
        msg_send![self, sortUsingComparator: cmptr]
    }
    pub unsafe fn sortWithOptions_usingComparator(&self, opts: NSSortOptions, cmptr: NSComparator) {
        msg_send![self, sortWithOptions: opts, usingComparator: cmptr]
    }
}
#[doc = "NSMutableArrayCreation"]
impl NSMutableArray {
    pub unsafe fn arrayWithCapacity(numItems: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![Self::class(), arrayWithCapacity: numItems]
    }
    pub unsafe fn arrayWithContentsOfFile(path: &NSString) -> TodoGenerics {
        msg_send![Self::class(), arrayWithContentsOfFile: path]
    }
    pub unsafe fn arrayWithContentsOfURL(url: &NSURL) -> TodoGenerics {
        msg_send![Self::class(), arrayWithContentsOfURL: url]
    }
    pub unsafe fn initWithContentsOfFile(&self, path: &NSString) -> TodoGenerics {
        msg_send![self, initWithContentsOfFile: path]
    }
    pub unsafe fn initWithContentsOfURL(&self, url: &NSURL) -> TodoGenerics {
        msg_send![self, initWithContentsOfURL: url]
    }
}
#[doc = "NSMutableArrayDiffing"]
impl NSMutableArray {
    pub unsafe fn applyDifference(&self, difference: TodoGenerics) {
        msg_send![self, applyDifference: difference]
    }
}
