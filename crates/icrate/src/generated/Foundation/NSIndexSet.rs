use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSIndexSet;
    unsafe impl ClassType for NSIndexSet {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSIndexSet {
        #[method_id(indexSet)]
        pub unsafe fn indexSet() -> Id<Self, Shared>;
        # [method_id (indexSetWithIndex :)]
        pub unsafe fn indexSetWithIndex(value: NSUInteger) -> Id<Self, Shared>;
        # [method_id (indexSetWithIndexesInRange :)]
        pub unsafe fn indexSetWithIndexesInRange(range: NSRange) -> Id<Self, Shared>;
        # [method_id (initWithIndexesInRange :)]
        pub unsafe fn initWithIndexesInRange(&self, range: NSRange) -> Id<Self, Shared>;
        # [method_id (initWithIndexSet :)]
        pub unsafe fn initWithIndexSet(&self, indexSet: &NSIndexSet) -> Id<Self, Shared>;
        # [method_id (initWithIndex :)]
        pub unsafe fn initWithIndex(&self, value: NSUInteger) -> Id<Self, Shared>;
        # [method (isEqualToIndexSet :)]
        pub unsafe fn isEqualToIndexSet(&self, indexSet: &NSIndexSet) -> bool;
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;
        #[method(firstIndex)]
        pub unsafe fn firstIndex(&self) -> NSUInteger;
        #[method(lastIndex)]
        pub unsafe fn lastIndex(&self) -> NSUInteger;
        # [method (indexGreaterThanIndex :)]
        pub unsafe fn indexGreaterThanIndex(&self, value: NSUInteger) -> NSUInteger;
        # [method (indexLessThanIndex :)]
        pub unsafe fn indexLessThanIndex(&self, value: NSUInteger) -> NSUInteger;
        # [method (indexGreaterThanOrEqualToIndex :)]
        pub unsafe fn indexGreaterThanOrEqualToIndex(&self, value: NSUInteger) -> NSUInteger;
        # [method (indexLessThanOrEqualToIndex :)]
        pub unsafe fn indexLessThanOrEqualToIndex(&self, value: NSUInteger) -> NSUInteger;
        # [method (getIndexes : maxCount : inIndexRange :)]
        pub unsafe fn getIndexes_maxCount_inIndexRange(
            &self,
            indexBuffer: NonNull<NSUInteger>,
            bufferSize: NSUInteger,
            range: NSRangePointer,
        ) -> NSUInteger;
        # [method (countOfIndexesInRange :)]
        pub unsafe fn countOfIndexesInRange(&self, range: NSRange) -> NSUInteger;
        # [method (containsIndex :)]
        pub unsafe fn containsIndex(&self, value: NSUInteger) -> bool;
        # [method (containsIndexesInRange :)]
        pub unsafe fn containsIndexesInRange(&self, range: NSRange) -> bool;
        # [method (containsIndexes :)]
        pub unsafe fn containsIndexes(&self, indexSet: &NSIndexSet) -> bool;
        # [method (intersectsIndexesInRange :)]
        pub unsafe fn intersectsIndexesInRange(&self, range: NSRange) -> bool;
        # [method (enumerateIndexesUsingBlock :)]
        pub unsafe fn enumerateIndexesUsingBlock(&self, block: TodoBlock);
        # [method (enumerateIndexesWithOptions : usingBlock :)]
        pub unsafe fn enumerateIndexesWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: TodoBlock,
        );
        # [method (enumerateIndexesInRange : options : usingBlock :)]
        pub unsafe fn enumerateIndexesInRange_options_usingBlock(
            &self,
            range: NSRange,
            opts: NSEnumerationOptions,
            block: TodoBlock,
        );
        # [method (indexPassingTest :)]
        pub unsafe fn indexPassingTest(&self, predicate: TodoBlock) -> NSUInteger;
        # [method (indexWithOptions : passingTest :)]
        pub unsafe fn indexWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: TodoBlock,
        ) -> NSUInteger;
        # [method (indexInRange : options : passingTest :)]
        pub unsafe fn indexInRange_options_passingTest(
            &self,
            range: NSRange,
            opts: NSEnumerationOptions,
            predicate: TodoBlock,
        ) -> NSUInteger;
        # [method_id (indexesPassingTest :)]
        pub unsafe fn indexesPassingTest(&self, predicate: TodoBlock) -> Id<NSIndexSet, Shared>;
        # [method_id (indexesWithOptions : passingTest :)]
        pub unsafe fn indexesWithOptions_passingTest(
            &self,
            opts: NSEnumerationOptions,
            predicate: TodoBlock,
        ) -> Id<NSIndexSet, Shared>;
        # [method_id (indexesInRange : options : passingTest :)]
        pub unsafe fn indexesInRange_options_passingTest(
            &self,
            range: NSRange,
            opts: NSEnumerationOptions,
            predicate: TodoBlock,
        ) -> Id<NSIndexSet, Shared>;
        # [method (enumerateRangesUsingBlock :)]
        pub unsafe fn enumerateRangesUsingBlock(&self, block: TodoBlock);
        # [method (enumerateRangesWithOptions : usingBlock :)]
        pub unsafe fn enumerateRangesWithOptions_usingBlock(
            &self,
            opts: NSEnumerationOptions,
            block: TodoBlock,
        );
        # [method (enumerateRangesInRange : options : usingBlock :)]
        pub unsafe fn enumerateRangesInRange_options_usingBlock(
            &self,
            range: NSRange,
            opts: NSEnumerationOptions,
            block: TodoBlock,
        );
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSMutableIndexSet;
    unsafe impl ClassType for NSMutableIndexSet {
        type Super = NSIndexSet;
    }
);
extern_methods!(
    unsafe impl NSMutableIndexSet {
        # [method (addIndexes :)]
        pub unsafe fn addIndexes(&self, indexSet: &NSIndexSet);
        # [method (removeIndexes :)]
        pub unsafe fn removeIndexes(&self, indexSet: &NSIndexSet);
        #[method(removeAllIndexes)]
        pub unsafe fn removeAllIndexes(&self);
        # [method (addIndex :)]
        pub unsafe fn addIndex(&self, value: NSUInteger);
        # [method (removeIndex :)]
        pub unsafe fn removeIndex(&self, value: NSUInteger);
        # [method (addIndexesInRange :)]
        pub unsafe fn addIndexesInRange(&self, range: NSRange);
        # [method (removeIndexesInRange :)]
        pub unsafe fn removeIndexesInRange(&self, range: NSRange);
        # [method (shiftIndexesStartingAtIndex : by :)]
        pub unsafe fn shiftIndexesStartingAtIndex_by(&self, index: NSUInteger, delta: NSInteger);
    }
);
