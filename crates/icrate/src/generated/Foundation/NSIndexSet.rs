use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSIndexSet;
    unsafe impl ClassType for NSIndexSet {
        type Super = NSObject;
    }
);
impl NSIndexSet {
    pub unsafe fn indexSet() -> Id<Self, Shared> {
        msg_send_id![Self::class(), indexSet]
    }
    pub unsafe fn indexSetWithIndex(value: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![Self::class(), indexSetWithIndex: value]
    }
    pub unsafe fn indexSetWithIndexesInRange(range: NSRange) -> Id<Self, Shared> {
        msg_send_id![Self::class(), indexSetWithIndexesInRange: range]
    }
    pub unsafe fn initWithIndexesInRange(&self, range: NSRange) -> Id<Self, Shared> {
        msg_send_id![self, initWithIndexesInRange: range]
    }
    pub unsafe fn initWithIndexSet(&self, indexSet: &NSIndexSet) -> Id<Self, Shared> {
        msg_send_id![self, initWithIndexSet: indexSet]
    }
    pub unsafe fn initWithIndex(&self, value: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![self, initWithIndex: value]
    }
    pub unsafe fn isEqualToIndexSet(&self, indexSet: &NSIndexSet) -> bool {
        msg_send![self, isEqualToIndexSet: indexSet]
    }
    pub unsafe fn count(&self) -> NSUInteger {
        msg_send![self, count]
    }
    pub unsafe fn firstIndex(&self) -> NSUInteger {
        msg_send![self, firstIndex]
    }
    pub unsafe fn lastIndex(&self) -> NSUInteger {
        msg_send![self, lastIndex]
    }
    pub unsafe fn indexGreaterThanIndex(&self, value: NSUInteger) -> NSUInteger {
        msg_send![self, indexGreaterThanIndex: value]
    }
    pub unsafe fn indexLessThanIndex(&self, value: NSUInteger) -> NSUInteger {
        msg_send![self, indexLessThanIndex: value]
    }
    pub unsafe fn indexGreaterThanOrEqualToIndex(&self, value: NSUInteger) -> NSUInteger {
        msg_send![self, indexGreaterThanOrEqualToIndex: value]
    }
    pub unsafe fn indexLessThanOrEqualToIndex(&self, value: NSUInteger) -> NSUInteger {
        msg_send![self, indexLessThanOrEqualToIndex: value]
    }
    pub unsafe fn getIndexes_maxCount_inIndexRange(
        &self,
        indexBuffer: NonNull<NSUInteger>,
        bufferSize: NSUInteger,
        range: NSRangePointer,
    ) -> NSUInteger {
        msg_send![
            self,
            getIndexes: indexBuffer,
            maxCount: bufferSize,
            inIndexRange: range
        ]
    }
    pub unsafe fn countOfIndexesInRange(&self, range: NSRange) -> NSUInteger {
        msg_send![self, countOfIndexesInRange: range]
    }
    pub unsafe fn containsIndex(&self, value: NSUInteger) -> bool {
        msg_send![self, containsIndex: value]
    }
    pub unsafe fn containsIndexesInRange(&self, range: NSRange) -> bool {
        msg_send![self, containsIndexesInRange: range]
    }
    pub unsafe fn containsIndexes(&self, indexSet: &NSIndexSet) -> bool {
        msg_send![self, containsIndexes: indexSet]
    }
    pub unsafe fn intersectsIndexesInRange(&self, range: NSRange) -> bool {
        msg_send![self, intersectsIndexesInRange: range]
    }
    pub unsafe fn enumerateIndexesUsingBlock(&self, block: TodoBlock) {
        msg_send![self, enumerateIndexesUsingBlock: block]
    }
    pub unsafe fn enumerateIndexesWithOptions_usingBlock(
        &self,
        opts: NSEnumerationOptions,
        block: TodoBlock,
    ) {
        msg_send![self, enumerateIndexesWithOptions: opts, usingBlock: block]
    }
    pub unsafe fn enumerateIndexesInRange_options_usingBlock(
        &self,
        range: NSRange,
        opts: NSEnumerationOptions,
        block: TodoBlock,
    ) {
        msg_send![
            self,
            enumerateIndexesInRange: range,
            options: opts,
            usingBlock: block
        ]
    }
    pub unsafe fn indexPassingTest(&self, predicate: TodoBlock) -> NSUInteger {
        msg_send![self, indexPassingTest: predicate]
    }
    pub unsafe fn indexWithOptions_passingTest(
        &self,
        opts: NSEnumerationOptions,
        predicate: TodoBlock,
    ) -> NSUInteger {
        msg_send![self, indexWithOptions: opts, passingTest: predicate]
    }
    pub unsafe fn indexInRange_options_passingTest(
        &self,
        range: NSRange,
        opts: NSEnumerationOptions,
        predicate: TodoBlock,
    ) -> NSUInteger {
        msg_send![
            self,
            indexInRange: range,
            options: opts,
            passingTest: predicate
        ]
    }
    pub unsafe fn indexesPassingTest(&self, predicate: TodoBlock) -> Id<NSIndexSet, Shared> {
        msg_send_id![self, indexesPassingTest: predicate]
    }
    pub unsafe fn indexesWithOptions_passingTest(
        &self,
        opts: NSEnumerationOptions,
        predicate: TodoBlock,
    ) -> Id<NSIndexSet, Shared> {
        msg_send_id![self, indexesWithOptions: opts, passingTest: predicate]
    }
    pub unsafe fn indexesInRange_options_passingTest(
        &self,
        range: NSRange,
        opts: NSEnumerationOptions,
        predicate: TodoBlock,
    ) -> Id<NSIndexSet, Shared> {
        msg_send_id![
            self,
            indexesInRange: range,
            options: opts,
            passingTest: predicate
        ]
    }
    pub unsafe fn enumerateRangesUsingBlock(&self, block: TodoBlock) {
        msg_send![self, enumerateRangesUsingBlock: block]
    }
    pub unsafe fn enumerateRangesWithOptions_usingBlock(
        &self,
        opts: NSEnumerationOptions,
        block: TodoBlock,
    ) {
        msg_send![self, enumerateRangesWithOptions: opts, usingBlock: block]
    }
    pub unsafe fn enumerateRangesInRange_options_usingBlock(
        &self,
        range: NSRange,
        opts: NSEnumerationOptions,
        block: TodoBlock,
    ) {
        msg_send![
            self,
            enumerateRangesInRange: range,
            options: opts,
            usingBlock: block
        ]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSMutableIndexSet;
    unsafe impl ClassType for NSMutableIndexSet {
        type Super = NSIndexSet;
    }
);
impl NSMutableIndexSet {
    pub unsafe fn addIndexes(&self, indexSet: &NSIndexSet) {
        msg_send![self, addIndexes: indexSet]
    }
    pub unsafe fn removeIndexes(&self, indexSet: &NSIndexSet) {
        msg_send![self, removeIndexes: indexSet]
    }
    pub unsafe fn removeAllIndexes(&self) {
        msg_send![self, removeAllIndexes]
    }
    pub unsafe fn addIndex(&self, value: NSUInteger) {
        msg_send![self, addIndex: value]
    }
    pub unsafe fn removeIndex(&self, value: NSUInteger) {
        msg_send![self, removeIndex: value]
    }
    pub unsafe fn addIndexesInRange(&self, range: NSRange) {
        msg_send![self, addIndexesInRange: range]
    }
    pub unsafe fn removeIndexesInRange(&self, range: NSRange) {
        msg_send![self, removeIndexesInRange: range]
    }
    pub unsafe fn shiftIndexesStartingAtIndex_by(&self, index: NSUInteger, delta: NSInteger) {
        msg_send![self, shiftIndexesStartingAtIndex: index, by: delta]
    }
}
