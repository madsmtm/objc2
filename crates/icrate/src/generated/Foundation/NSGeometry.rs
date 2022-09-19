use crate::CoreGraphics::generated::CGBase::*;
use crate::CoreGraphics::generated::CGGeometry::*;
use crate::Foundation::generated::NSCoder::*;
use crate::Foundation::generated::NSValue::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSPoint = CGPoint;
pub type NSSize = CGSize;
pub type NSRect = CGRect;
use super::__exported::NSString;
#[doc = "NSValueGeometryExtensions"]
impl NSValue {
    pub unsafe fn valueWithPoint(point: NSPoint) -> Id<NSValue, Shared> {
        msg_send_id![Self::class(), valueWithPoint: point]
    }
    pub unsafe fn valueWithSize(size: NSSize) -> Id<NSValue, Shared> {
        msg_send_id![Self::class(), valueWithSize: size]
    }
    pub unsafe fn valueWithRect(rect: NSRect) -> Id<NSValue, Shared> {
        msg_send_id![Self::class(), valueWithRect: rect]
    }
    pub unsafe fn valueWithEdgeInsets(insets: NSEdgeInsets) -> Id<NSValue, Shared> {
        msg_send_id![Self::class(), valueWithEdgeInsets: insets]
    }
    pub unsafe fn pointValue(&self) -> NSPoint {
        msg_send![self, pointValue]
    }
    pub unsafe fn sizeValue(&self) -> NSSize {
        msg_send![self, sizeValue]
    }
    pub unsafe fn rectValue(&self) -> NSRect {
        msg_send![self, rectValue]
    }
    pub unsafe fn edgeInsetsValue(&self) -> NSEdgeInsets {
        msg_send![self, edgeInsetsValue]
    }
}
#[doc = "NSGeometryCoding"]
impl NSCoder {
    pub unsafe fn encodePoint(&self, point: NSPoint) {
        msg_send![self, encodePoint: point]
    }
    pub unsafe fn decodePoint(&self) -> NSPoint {
        msg_send![self, decodePoint]
    }
    pub unsafe fn encodeSize(&self, size: NSSize) {
        msg_send![self, encodeSize: size]
    }
    pub unsafe fn decodeSize(&self) -> NSSize {
        msg_send![self, decodeSize]
    }
    pub unsafe fn encodeRect(&self, rect: NSRect) {
        msg_send![self, encodeRect: rect]
    }
    pub unsafe fn decodeRect(&self) -> NSRect {
        msg_send![self, decodeRect]
    }
}
#[doc = "NSGeometryKeyedCoding"]
impl NSCoder {
    pub unsafe fn encodePoint_forKey(&self, point: NSPoint, key: &NSString) {
        msg_send![self, encodePoint: point, forKey: key]
    }
    pub unsafe fn encodeSize_forKey(&self, size: NSSize, key: &NSString) {
        msg_send![self, encodeSize: size, forKey: key]
    }
    pub unsafe fn encodeRect_forKey(&self, rect: NSRect, key: &NSString) {
        msg_send![self, encodeRect: rect, forKey: key]
    }
    pub unsafe fn decodePointForKey(&self, key: &NSString) -> NSPoint {
        msg_send![self, decodePointForKey: key]
    }
    pub unsafe fn decodeSizeForKey(&self, key: &NSString) -> NSSize {
        msg_send![self, decodeSizeForKey: key]
    }
    pub unsafe fn decodeRectForKey(&self, key: &NSString) -> NSRect {
        msg_send![self, decodeRectForKey: key]
    }
}
