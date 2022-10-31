//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

pub type NSTIFFCompression = NSUInteger;
pub const NSTIFFCompressionNone: NSTIFFCompression = 1;
pub const NSTIFFCompressionCCITTFAX3: NSTIFFCompression = 3;
pub const NSTIFFCompressionCCITTFAX4: NSTIFFCompression = 4;
pub const NSTIFFCompressionLZW: NSTIFFCompression = 5;
pub const NSTIFFCompressionJPEG: NSTIFFCompression = 6;
pub const NSTIFFCompressionNEXT: NSTIFFCompression = 32766;
pub const NSTIFFCompressionPackBits: NSTIFFCompression = 32773;
pub const NSTIFFCompressionOldJPEG: NSTIFFCompression = 32865;

pub type NSBitmapImageFileType = NSUInteger;
pub const NSBitmapImageFileTypeTIFF: NSBitmapImageFileType = 0;
pub const NSBitmapImageFileTypeBMP: NSBitmapImageFileType = 1;
pub const NSBitmapImageFileTypeGIF: NSBitmapImageFileType = 2;
pub const NSBitmapImageFileTypeJPEG: NSBitmapImageFileType = 3;
pub const NSBitmapImageFileTypePNG: NSBitmapImageFileType = 4;
pub const NSBitmapImageFileTypeJPEG2000: NSBitmapImageFileType = 5;

pub type NSImageRepLoadStatus = NSInteger;
pub const NSImageRepLoadStatusUnknownType: NSImageRepLoadStatus = -1;
pub const NSImageRepLoadStatusReadingHeader: NSImageRepLoadStatus = -2;
pub const NSImageRepLoadStatusWillNeedAllData: NSImageRepLoadStatus = -3;
pub const NSImageRepLoadStatusInvalidData: NSImageRepLoadStatus = -4;
pub const NSImageRepLoadStatusUnexpectedEOF: NSImageRepLoadStatus = -5;
pub const NSImageRepLoadStatusCompleted: NSImageRepLoadStatus = -6;

pub type NSBitmapFormat = NSUInteger;
pub const NSBitmapFormatAlphaFirst: NSBitmapFormat = 1 << 0;
pub const NSBitmapFormatAlphaNonpremultiplied: NSBitmapFormat = 1 << 1;
pub const NSBitmapFormatFloatingPointSamples: NSBitmapFormat = 1 << 2;
pub const NSBitmapFormatSixteenBitLittleEndian: NSBitmapFormat = (1 << 8);
pub const NSBitmapFormatThirtyTwoBitLittleEndian: NSBitmapFormat = (1 << 9);
pub const NSBitmapFormatSixteenBitBigEndian: NSBitmapFormat = (1 << 10);
pub const NSBitmapFormatThirtyTwoBitBigEndian: NSBitmapFormat = (1 << 11);

pub type NSBitmapImageRepPropertyKey = NSString;

extern "C" {
    static NSImageCompressionMethod: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    static NSImageCompressionFactor: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    static NSImageDitherTransparency: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    static NSImageRGBColorTable: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    static NSImageInterlaced: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    static NSImageColorSyncProfileData: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    static NSImageFrameCount: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    static NSImageCurrentFrame: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    static NSImageCurrentFrameDuration: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    static NSImageLoopCount: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    static NSImageGamma: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    static NSImageProgressive: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    static NSImageEXIFData: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    static NSImageIPTCData: &'static NSBitmapImageRepPropertyKey;
}

extern "C" {
    static NSImageFallbackBackgroundColor: &'static NSBitmapImageRepPropertyKey;
}

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
    /// NSBitmapImageFileTypeExtensions
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

static NSTIFFFileType: NSBitmapImageFileType = NSBitmapImageFileTypeTIFF;

static NSBMPFileType: NSBitmapImageFileType = NSBitmapImageFileTypeBMP;

static NSGIFFileType: NSBitmapImageFileType = NSBitmapImageFileTypeGIF;

static NSJPEGFileType: NSBitmapImageFileType = NSBitmapImageFileTypeJPEG;

static NSPNGFileType: NSBitmapImageFileType = NSBitmapImageFileTypePNG;

static NSJPEG2000FileType: NSBitmapImageFileType = NSBitmapImageFileTypeJPEG2000;

static NSAlphaFirstBitmapFormat: NSBitmapFormat = NSBitmapFormatAlphaFirst;

static NSAlphaNonpremultipliedBitmapFormat: NSBitmapFormat = NSBitmapFormatAlphaNonpremultiplied;

static NSFloatingPointSamplesBitmapFormat: NSBitmapFormat = NSBitmapFormatFloatingPointSamples;

static NS16BitLittleEndianBitmapFormat: NSBitmapFormat = NSBitmapFormatSixteenBitLittleEndian;

static NS32BitLittleEndianBitmapFormat: NSBitmapFormat = NSBitmapFormatThirtyTwoBitLittleEndian;

static NS16BitBigEndianBitmapFormat: NSBitmapFormat = NSBitmapFormatSixteenBitBigEndian;

static NS32BitBigEndianBitmapFormat: NSBitmapFormat = NSBitmapFormatThirtyTwoBitBigEndian;
