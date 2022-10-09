use super::__exported::CIImage;
use super::__exported::NSColor;
use super::__exported::NSColorSpace;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSImageRep::*;
use crate::ApplicationServices::generated::ApplicationServices::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSDictionary::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSBitmapImageRepPropertyKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSBitmapImageRep;
    unsafe impl ClassType for NSBitmapImageRep {
        type Super = NSImageRep;
    }
);
extern_methods!(
    unsafe impl NSBitmapImageRep {
        #[method_id(initWithFocusedViewRect:)]
        pub unsafe fn initWithFocusedViewRect(&self, rect: NSRect) -> Option<Id<Self, Shared>>;
        #[method_id(initWithBitmapDataPlanes:pixelsWide:pixelsHigh:bitsPerSample:samplesPerPixel:hasAlpha:isPlanar:colorSpaceName:bytesPerRow:bitsPerPixel:)]
        pub unsafe fn initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bytesPerRow_bitsPerPixel(
            &self,
            planes: *mut *mut c_uchar,
            width: NSInteger,
            height: NSInteger,
            bps: NSInteger,
            spp: NSInteger,
            alpha: bool,
            isPlanar: bool,
            colorSpaceName: &NSColorSpaceName,
            rBytes: NSInteger,
            pBits: NSInteger,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithBitmapDataPlanes:pixelsWide:pixelsHigh:bitsPerSample:samplesPerPixel:hasAlpha:isPlanar:colorSpaceName:bitmapFormat:bytesPerRow:bitsPerPixel:)]
        pub unsafe fn initWithBitmapDataPlanes_pixelsWide_pixelsHigh_bitsPerSample_samplesPerPixel_hasAlpha_isPlanar_colorSpaceName_bitmapFormat_bytesPerRow_bitsPerPixel(
            &self,
            planes: *mut *mut c_uchar,
            width: NSInteger,
            height: NSInteger,
            bps: NSInteger,
            spp: NSInteger,
            alpha: bool,
            isPlanar: bool,
            colorSpaceName: &NSColorSpaceName,
            bitmapFormat: NSBitmapFormat,
            rBytes: NSInteger,
            pBits: NSInteger,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithCGImage:)]
        pub unsafe fn initWithCGImage(&self, cgImage: CGImageRef) -> Id<Self, Shared>;
        #[method_id(initWithCIImage:)]
        pub unsafe fn initWithCIImage(&self, ciImage: &CIImage) -> Id<Self, Shared>;
        #[method_id(imageRepsWithData:)]
        pub unsafe fn imageRepsWithData(data: &NSData) -> Id<NSArray<NSImageRep>, Shared>;
        #[method_id(imageRepWithData:)]
        pub unsafe fn imageRepWithData(data: &NSData) -> Option<Id<Self, Shared>>;
        #[method_id(initWithData:)]
        pub unsafe fn initWithData(&self, data: &NSData) -> Option<Id<Self, Shared>>;
        #[method(bitmapData)]
        pub unsafe fn bitmapData(&self) -> *mut c_uchar;
        #[method(getBitmapDataPlanes:)]
        pub unsafe fn getBitmapDataPlanes(&self, data: NonNull<*mut c_uchar>);
        #[method(isPlanar)]
        pub unsafe fn isPlanar(&self) -> bool;
        #[method(samplesPerPixel)]
        pub unsafe fn samplesPerPixel(&self) -> NSInteger;
        #[method(bitsPerPixel)]
        pub unsafe fn bitsPerPixel(&self) -> NSInteger;
        #[method(bytesPerRow)]
        pub unsafe fn bytesPerRow(&self) -> NSInteger;
        #[method(bytesPerPlane)]
        pub unsafe fn bytesPerPlane(&self) -> NSInteger;
        #[method(numberOfPlanes)]
        pub unsafe fn numberOfPlanes(&self) -> NSInteger;
        #[method(bitmapFormat)]
        pub unsafe fn bitmapFormat(&self) -> NSBitmapFormat;
        #[method(getCompression:factor:)]
        pub unsafe fn getCompression_factor(
            &self,
            compression: *mut NSTIFFCompression,
            factor: *mut c_float,
        );
        #[method(setCompression:factor:)]
        pub unsafe fn setCompression_factor(&self, compression: NSTIFFCompression, factor: c_float);
        #[method_id(TIFFRepresentation)]
        pub unsafe fn TIFFRepresentation(&self) -> Option<Id<NSData, Shared>>;
        #[method_id(TIFFRepresentationUsingCompression:factor:)]
        pub unsafe fn TIFFRepresentationUsingCompression_factor(
            &self,
            comp: NSTIFFCompression,
            factor: c_float,
        ) -> Option<Id<NSData, Shared>>;
        #[method_id(TIFFRepresentationOfImageRepsInArray:)]
        pub unsafe fn TIFFRepresentationOfImageRepsInArray(
            array: &NSArray<NSImageRep>,
        ) -> Option<Id<NSData, Shared>>;
        #[method_id(TIFFRepresentationOfImageRepsInArray:usingCompression:factor:)]
        pub unsafe fn TIFFRepresentationOfImageRepsInArray_usingCompression_factor(
            array: &NSArray<NSImageRep>,
            comp: NSTIFFCompression,
            factor: c_float,
        ) -> Option<Id<NSData, Shared>>;
        #[method(getTIFFCompressionTypes:count:)]
        pub unsafe fn getTIFFCompressionTypes_count(
            list: NonNull<*mut NSTIFFCompression>,
            numTypes: NonNull<NSInteger>,
        );
        #[method_id(localizedNameForTIFFCompressionType:)]
        pub unsafe fn localizedNameForTIFFCompressionType(
            compression: NSTIFFCompression,
        ) -> Option<Id<NSString, Shared>>;
        #[method(canBeCompressedUsing:)]
        pub unsafe fn canBeCompressedUsing(&self, compression: NSTIFFCompression) -> bool;
        #[method(colorizeByMappingGray:toColor:blackMapping:whiteMapping:)]
        pub unsafe fn colorizeByMappingGray_toColor_blackMapping_whiteMapping(
            &self,
            midPoint: CGFloat,
            midPointColor: Option<&NSColor>,
            shadowColor: Option<&NSColor>,
            lightColor: Option<&NSColor>,
        );
        #[method_id(initForIncrementalLoad)]
        pub unsafe fn initForIncrementalLoad(&self) -> Id<Self, Shared>;
        #[method(incrementalLoadFromData:complete:)]
        pub unsafe fn incrementalLoadFromData_complete(
            &self,
            data: &NSData,
            complete: bool,
        ) -> NSInteger;
        #[method(setColor:atX:y:)]
        pub unsafe fn setColor_atX_y(&self, color: &NSColor, x: NSInteger, y: NSInteger);
        #[method_id(colorAtX:y:)]
        pub unsafe fn colorAtX_y(&self, x: NSInteger, y: NSInteger) -> Option<Id<NSColor, Shared>>;
        #[method(getPixel:atX:y:)]
        pub unsafe fn getPixel_atX_y(&self, p: TodoArray, x: NSInteger, y: NSInteger);
        #[method(setPixel:atX:y:)]
        pub unsafe fn setPixel_atX_y(&self, p: TodoArray, x: NSInteger, y: NSInteger);
        #[method(CGImage)]
        pub unsafe fn CGImage(&self) -> CGImageRef;
        #[method_id(colorSpace)]
        pub unsafe fn colorSpace(&self) -> Id<NSColorSpace, Shared>;
        #[method_id(bitmapImageRepByConvertingToColorSpace:renderingIntent:)]
        pub unsafe fn bitmapImageRepByConvertingToColorSpace_renderingIntent(
            &self,
            targetSpace: &NSColorSpace,
            renderingIntent: NSColorRenderingIntent,
        ) -> Option<Id<NSBitmapImageRep, Shared>>;
        #[method_id(bitmapImageRepByRetaggingWithColorSpace:)]
        pub unsafe fn bitmapImageRepByRetaggingWithColorSpace(
            &self,
            newSpace: &NSColorSpace,
        ) -> Option<Id<NSBitmapImageRep, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSBitmapImageFileTypeExtensions"]
    unsafe impl NSBitmapImageRep {
        #[method_id(representationOfImageRepsInArray:usingType:properties:)]
        pub unsafe fn representationOfImageRepsInArray_usingType_properties(
            imageReps: &NSArray<NSImageRep>,
            storageType: NSBitmapImageFileType,
            properties: &NSDictionary<NSBitmapImageRepPropertyKey, Object>,
        ) -> Option<Id<NSData, Shared>>;
        #[method_id(representationUsingType:properties:)]
        pub unsafe fn representationUsingType_properties(
            &self,
            storageType: NSBitmapImageFileType,
            properties: &NSDictionary<NSBitmapImageRepPropertyKey, Object>,
        ) -> Option<Id<NSData, Shared>>;
        #[method(setProperty:withValue:)]
        pub unsafe fn setProperty_withValue(
            &self,
            property: &NSBitmapImageRepPropertyKey,
            value: Option<&Object>,
        );
        #[method_id(valueForProperty:)]
        pub unsafe fn valueForProperty(
            &self,
            property: &NSBitmapImageRepPropertyKey,
        ) -> Option<Id<Object, Shared>>;
    }
);
