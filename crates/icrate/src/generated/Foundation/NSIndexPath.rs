use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSIndexPath;
    unsafe impl ClassType for NSIndexPath {
        type Super = NSObject;
    }
);
impl NSIndexPath {
    pub unsafe fn indexPathWithIndex(index: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![Self::class(), indexPathWithIndex: index]
    }
    pub unsafe fn indexPathWithIndexes_length(
        indexes: TodoArray,
        length: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![Self::class(), indexPathWithIndexes: indexes, length: length]
    }
    pub unsafe fn initWithIndexes_length(
        &self,
        indexes: TodoArray,
        length: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithIndexes: indexes, length: length]
    }
    pub unsafe fn initWithIndex(&self, index: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![self, initWithIndex: index]
    }
    pub unsafe fn indexPathByAddingIndex(&self, index: NSUInteger) -> Id<NSIndexPath, Shared> {
        msg_send_id![self, indexPathByAddingIndex: index]
    }
    pub unsafe fn indexPathByRemovingLastIndex(&self) -> Id<NSIndexPath, Shared> {
        msg_send_id![self, indexPathByRemovingLastIndex]
    }
    pub unsafe fn indexAtPosition(&self, position: NSUInteger) -> NSUInteger {
        msg_send![self, indexAtPosition: position]
    }
    pub unsafe fn length(&self) -> NSUInteger {
        msg_send![self, length]
    }
    pub unsafe fn getIndexes_range(&self, indexes: NonNull<NSUInteger>, positionRange: NSRange) {
        msg_send![self, getIndexes: indexes, range: positionRange]
    }
    pub unsafe fn compare(&self, otherObject: &NSIndexPath) -> NSComparisonResult {
        msg_send![self, compare: otherObject]
    }
}
#[doc = "NSDeprecated"]
impl NSIndexPath {
    pub unsafe fn getIndexes(&self, indexes: NonNull<NSUInteger>) {
        msg_send![self, getIndexes: indexes]
    }
}
