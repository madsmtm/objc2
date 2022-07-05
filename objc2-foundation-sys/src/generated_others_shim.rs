//! Other quick examples

use core::ptr::NonNull;

use std::os::raw::c_void;

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

#[repr(transparent)]
pub struct NSData(NSObject);
impl core::ops::Deref for NSData {
    type Target = NSObject;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for NSData {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl Message for NSData {}
unsafe impl RefEncode for NSData {
    const ENCODING_REF: Encoding<'static> = Encoding::Object;
}
impl NSData {
    pub unsafe fn alloc() -> Option<Id<Allocated<Self>, Unknown>> {
        msg_send_id![class!(NSData), alloc]
    }
}
impl NSData {
    pub unsafe fn length(&self) -> NSUInteger {
        msg_send![self, length]
    }
    pub unsafe fn bytes(&self) -> *const c_void {
        msg_send![self, bytes]
    }
}
impl NSData {
    pub unsafe fn initWithBytes_length_(
        this: Option<Id<Allocated<Self>, Unknown>>,
        bytes: *const c_void,
        length: NSUInteger,
    ) -> Id<Self, Unknown> {
        msg_send_id![this, initWithBytes: bytes, length: length].unwrap_unchecked()
    }
    pub unsafe fn initWithBytesNoCopy_length_deallocator_(
        this: Option<Id<Allocated<Self>, Unknown>>,
        bytes: *mut c_void,
        length: NSUInteger,
        deallocator: *mut c_void,
    ) -> Id<Self, Unknown> {
        msg_send_id![
            this,
            initWithBytesNoCopy: bytes,
            length: length,
            deallocator: deallocator,
        ]
        .unwrap_unchecked()
    }
}
