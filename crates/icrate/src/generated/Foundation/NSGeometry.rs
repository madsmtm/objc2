use crate::CoreGraphics::generated::CGBase::*;
use crate::CoreGraphics::generated::CGGeometry::*;
use crate::Foundation::generated::NSCoder::*;
use crate::Foundation::generated::NSValue::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSPoint = CGPoint;
pub type NSSize = CGSize;
pub type NSRect = CGRect;
use super::__exported::NSString;
extern_methods!(
    #[doc = "NSValueGeometryExtensions"]
    unsafe impl NSValue {
        # [method_id (valueWithPoint :)]
        pub unsafe fn valueWithPoint(point: NSPoint) -> Id<NSValue, Shared>;
        # [method_id (valueWithSize :)]
        pub unsafe fn valueWithSize(size: NSSize) -> Id<NSValue, Shared>;
        # [method_id (valueWithRect :)]
        pub unsafe fn valueWithRect(rect: NSRect) -> Id<NSValue, Shared>;
        # [method_id (valueWithEdgeInsets :)]
        pub unsafe fn valueWithEdgeInsets(insets: NSEdgeInsets) -> Id<NSValue, Shared>;
        #[method(pointValue)]
        pub unsafe fn pointValue(&self) -> NSPoint;
        #[method(sizeValue)]
        pub unsafe fn sizeValue(&self) -> NSSize;
        #[method(rectValue)]
        pub unsafe fn rectValue(&self) -> NSRect;
        #[method(edgeInsetsValue)]
        pub unsafe fn edgeInsetsValue(&self) -> NSEdgeInsets;
    }
);
extern_methods!(
    #[doc = "NSGeometryCoding"]
    unsafe impl NSCoder {
        # [method (encodePoint :)]
        pub unsafe fn encodePoint(&self, point: NSPoint);
        #[method(decodePoint)]
        pub unsafe fn decodePoint(&self) -> NSPoint;
        # [method (encodeSize :)]
        pub unsafe fn encodeSize(&self, size: NSSize);
        #[method(decodeSize)]
        pub unsafe fn decodeSize(&self) -> NSSize;
        # [method (encodeRect :)]
        pub unsafe fn encodeRect(&self, rect: NSRect);
        #[method(decodeRect)]
        pub unsafe fn decodeRect(&self) -> NSRect;
    }
);
extern_methods!(
    #[doc = "NSGeometryKeyedCoding"]
    unsafe impl NSCoder {
        # [method (encodePoint : forKey :)]
        pub unsafe fn encodePoint_forKey(&self, point: NSPoint, key: &NSString);
        # [method (encodeSize : forKey :)]
        pub unsafe fn encodeSize_forKey(&self, size: NSSize, key: &NSString);
        # [method (encodeRect : forKey :)]
        pub unsafe fn encodeRect_forKey(&self, rect: NSRect, key: &NSString);
        # [method (decodePointForKey :)]
        pub unsafe fn decodePointForKey(&self, key: &NSString) -> NSPoint;
        # [method (decodeSizeForKey :)]
        pub unsafe fn decodeSizeForKey(&self, key: &NSString) -> NSSize;
        # [method (decodeRectForKey :)]
        pub unsafe fn decodeRectForKey(&self, key: &NSString) -> NSRect;
    }
);
