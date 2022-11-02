//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSCompositingOperation {
        NSCompositingOperationClear = 0,
        NSCompositingOperationCopy = 1,
        NSCompositingOperationSourceOver = 2,
        NSCompositingOperationSourceIn = 3,
        NSCompositingOperationSourceOut = 4,
        NSCompositingOperationSourceAtop = 5,
        NSCompositingOperationDestinationOver = 6,
        NSCompositingOperationDestinationIn = 7,
        NSCompositingOperationDestinationOut = 8,
        NSCompositingOperationDestinationAtop = 9,
        NSCompositingOperationXOR = 10,
        NSCompositingOperationPlusDarker = 11,
        NSCompositingOperationHighlight = 12,
        NSCompositingOperationPlusLighter = 13,
        NSCompositingOperationMultiply = 14,
        NSCompositingOperationScreen = 15,
        NSCompositingOperationOverlay = 16,
        NSCompositingOperationDarken = 17,
        NSCompositingOperationLighten = 18,
        NSCompositingOperationColorDodge = 19,
        NSCompositingOperationColorBurn = 20,
        NSCompositingOperationSoftLight = 21,
        NSCompositingOperationHardLight = 22,
        NSCompositingOperationDifference = 23,
        NSCompositingOperationExclusion = 24,
        NSCompositingOperationHue = 25,
        NSCompositingOperationSaturation = 26,
        NSCompositingOperationColor = 27,
        NSCompositingOperationLuminosity = 28,
    }
);

extern_static!(NSCompositeClear: NSCompositingOperation = NSCompositingOperationClear);

extern_static!(NSCompositeCopy: NSCompositingOperation = NSCompositingOperationCopy);

extern_static!(NSCompositeSourceOver: NSCompositingOperation = NSCompositingOperationSourceOver);

extern_static!(NSCompositeSourceIn: NSCompositingOperation = NSCompositingOperationSourceIn);

extern_static!(NSCompositeSourceOut: NSCompositingOperation = NSCompositingOperationSourceOut);

extern_static!(NSCompositeSourceAtop: NSCompositingOperation = NSCompositingOperationSourceAtop);

extern_static!(
    NSCompositeDestinationOver: NSCompositingOperation = NSCompositingOperationDestinationOver
);

extern_static!(
    NSCompositeDestinationIn: NSCompositingOperation = NSCompositingOperationDestinationIn
);

extern_static!(
    NSCompositeDestinationOut: NSCompositingOperation = NSCompositingOperationDestinationOut
);

extern_static!(
    NSCompositeDestinationAtop: NSCompositingOperation = NSCompositingOperationDestinationAtop
);

extern_static!(NSCompositeXOR: NSCompositingOperation = NSCompositingOperationXOR);

extern_static!(NSCompositePlusDarker: NSCompositingOperation = NSCompositingOperationPlusDarker);

extern_static!(NSCompositeHighlight: NSCompositingOperation = NSCompositingOperationHighlight);

extern_static!(NSCompositePlusLighter: NSCompositingOperation = NSCompositingOperationPlusLighter);

extern_static!(NSCompositeMultiply: NSCompositingOperation = NSCompositingOperationMultiply);

extern_static!(NSCompositeScreen: NSCompositingOperation = NSCompositingOperationScreen);

extern_static!(NSCompositeOverlay: NSCompositingOperation = NSCompositingOperationOverlay);

extern_static!(NSCompositeDarken: NSCompositingOperation = NSCompositingOperationDarken);

extern_static!(NSCompositeLighten: NSCompositingOperation = NSCompositingOperationLighten);

extern_static!(NSCompositeColorDodge: NSCompositingOperation = NSCompositingOperationColorDodge);

extern_static!(NSCompositeColorBurn: NSCompositingOperation = NSCompositingOperationColorBurn);

extern_static!(NSCompositeSoftLight: NSCompositingOperation = NSCompositingOperationSoftLight);

extern_static!(NSCompositeHardLight: NSCompositingOperation = NSCompositingOperationHardLight);

extern_static!(NSCompositeDifference: NSCompositingOperation = NSCompositingOperationDifference);

extern_static!(NSCompositeExclusion: NSCompositingOperation = NSCompositingOperationExclusion);

extern_static!(NSCompositeHue: NSCompositingOperation = NSCompositingOperationHue);

extern_static!(NSCompositeSaturation: NSCompositingOperation = NSCompositingOperationSaturation);

extern_static!(NSCompositeColor: NSCompositingOperation = NSCompositingOperationColor);

extern_static!(NSCompositeLuminosity: NSCompositingOperation = NSCompositingOperationLuminosity);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBackingStoreType {
        NSBackingStoreRetained = 0,
        NSBackingStoreNonretained = 1,
        NSBackingStoreBuffered = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSWindowOrderingMode {
        NSWindowAbove = 1,
        NSWindowBelow = -1,
        NSWindowOut = 0,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSFocusRingPlacement {
        NSFocusRingOnly = 0,
        NSFocusRingBelow = 1,
        NSFocusRingAbove = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSFocusRingType {
        NSFocusRingTypeDefault = 0,
        NSFocusRingTypeNone = 1,
        NSFocusRingTypeExterior = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSColorRenderingIntent {
        NSColorRenderingIntentDefault = 0,
        NSColorRenderingIntentAbsoluteColorimetric = 1,
        NSColorRenderingIntentRelativeColorimetric = 2,
        NSColorRenderingIntentPerceptual = 3,
        NSColorRenderingIntentSaturation = 4,
    }
);

pub type NSColorSpaceName = NSString;

extern_static!(NSCalibratedWhiteColorSpace: &'static NSColorSpaceName);

extern_static!(NSCalibratedRGBColorSpace: &'static NSColorSpaceName);

extern_static!(NSDeviceWhiteColorSpace: &'static NSColorSpaceName);

extern_static!(NSDeviceRGBColorSpace: &'static NSColorSpaceName);

extern_static!(NSDeviceCMYKColorSpace: &'static NSColorSpaceName);

extern_static!(NSNamedColorSpace: &'static NSColorSpaceName);

extern_static!(NSPatternColorSpace: &'static NSColorSpaceName);

extern_static!(NSCustomColorSpace: &'static NSColorSpaceName);

extern_static!(NSCalibratedBlackColorSpace: &'static NSColorSpaceName);

extern_static!(NSDeviceBlackColorSpace: &'static NSColorSpaceName);

ns_enum!(
    #[underlying(i32)]
    pub enum NSWindowDepth {
        NSWindowDepthTwentyfourBitRGB = 0x208,
        NSWindowDepthSixtyfourBitRGB = 0x210,
        NSWindowDepthOnehundredtwentyeightBitRGB = 0x220,
    }
);

extern_static!(NSWhite: CGFloat);

extern_static!(NSLightGray: CGFloat);

extern_static!(NSDarkGray: CGFloat);

extern_static!(NSBlack: CGFloat);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSDisplayGamut {
        NSDisplayGamutSRGB = 1,
        NSDisplayGamutP3 = 2,
    }
);

pub type NSDeviceDescriptionKey = NSString;

extern_static!(NSDeviceResolution: &'static NSDeviceDescriptionKey);

extern_static!(NSDeviceColorSpaceName: &'static NSDeviceDescriptionKey);

extern_static!(NSDeviceBitsPerSample: &'static NSDeviceDescriptionKey);

extern_static!(NSDeviceIsScreen: &'static NSDeviceDescriptionKey);

extern_static!(NSDeviceIsPrinter: &'static NSDeviceDescriptionKey);

extern_static!(NSDeviceSize: &'static NSDeviceDescriptionKey);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSAnimationEffect {
        NSAnimationEffectDisappearingItemDefault = 0,
        NSAnimationEffectPoof = 10,
    }
);
