//! Other quick examples

use core::ptr::NonNull;

use objc2::ffi::{NSInteger, NSUInteger};
use objc2::rc::{Allocated, Id, Unknown};
use objc2::runtime::Object;
use objc2::{class, msg_send, msg_send_id, Encoding, Message, RefEncode};

use crate::{NSCoder, NSObject, NSRange};

pub const NSComparisonResult_NSOrderedAscending: NSComparisonResult = -1;
pub const NSComparisonResult_NSOrderedSame: NSComparisonResult = 0;
pub const NSComparisonResult_NSOrderedDescending: NSComparisonResult = 1;
pub type NSComparisonResult = NSInteger;

#[repr(transparent)]
pub struct NSArray(NSObject);
impl core::ops::Deref for NSArray {
    type Target = NSObject;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for NSArray {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl Message for NSArray {}
unsafe impl RefEncode for NSArray {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}
impl NSArray {
    pub unsafe fn alloc() -> Option<Id<Allocated<Self>, Unknown>> {
        msg_send_id![class!(NSArray), alloc]
    }
}
impl NSArray {
    pub unsafe fn objectAtIndex_(&self, index: NSUInteger) -> NonNull<Object> {
        msg_send![self, objectAtIndex: index]
    }
    pub unsafe fn init(this: Option<Id<Allocated<Self>, Unknown>>) -> Id<Self, Unknown> {
        msg_send_id![this, init].unwrap_unchecked()
    }
    pub unsafe fn initWithObjects_count_(
        this: Option<Id<Allocated<Self>, Unknown>>,
        objects: NonNull<*mut Object>,
        cnt: NSUInteger,
    ) -> Id<Self, Unknown> {
        msg_send_id![this, initWithObjects: objects, count: cnt].unwrap_unchecked()
    }
    pub unsafe fn initWithCoder_(
        this: Option<Id<Allocated<Self>, Unknown>>,
        coder: NonNull<NSCoder>,
    ) -> Option<Id<Self, Unknown>> {
        msg_send_id![this, initWithCoder: coder]
    }
    pub unsafe fn count(&self) -> NSUInteger {
        msg_send![self, count]
    }
    pub unsafe fn firstObject(&self) -> Option<NonNull<Object>> {
        msg_send![self, firstObject]
    }
    pub unsafe fn lastObject(&self) -> Option<NonNull<Object>> {
        msg_send![self, lastObject]
    }
    pub unsafe fn getObjects_range_(&self, objects: NonNull<NonNull<Object>>, range: NSRange) {
        msg_send![self, getObjects: objects, range: range]
    }
}
