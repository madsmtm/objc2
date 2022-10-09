use super::__exported::NSColor;
use super::__exported::NSGraphicsContext;
use super::__exported::NSImageRep;
use super::__exported::NSURL;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSBitmapImageRep::*;
use crate::AppKit::generated::NSFontDescriptor::*;
use crate::AppKit::generated::NSGraphics::*;
use crate::AppKit::generated::NSLayoutConstraint::*;
use crate::AppKit::generated::NSPasteboard::*;
use crate::ApplicationServices::generated::ApplicationServices::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSBundle::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSImageName = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSImage;
    unsafe impl ClassType for NSImage {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSImage {
        #[method_id(imageNamed:)]
        pub unsafe fn imageNamed(name: &NSImageName) -> Option<Id<NSImage, Shared>>;
        #[method_id(imageWithSystemSymbolName:accessibilityDescription:)]
        pub unsafe fn imageWithSystemSymbolName_accessibilityDescription(
            symbolName: &NSString,
            description: Option<&NSString>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithSize:)]
        pub unsafe fn initWithSize(&self, size: NSSize) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(initWithData:)]
        pub unsafe fn initWithData(&self, data: &NSData) -> Option<Id<Self, Shared>>;
        #[method_id(initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            &self,
            fileName: &NSString,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(&self, url: &NSURL) -> Option<Id<Self, Shared>>;
        #[method_id(initByReferencingFile:)]
        pub unsafe fn initByReferencingFile(&self, fileName: &NSString)
            -> Option<Id<Self, Shared>>;
        #[method_id(initByReferencingURL:)]
        pub unsafe fn initByReferencingURL(&self, url: &NSURL) -> Id<Self, Shared>;
        #[method_id(initWithPasteboard:)]
        pub unsafe fn initWithPasteboard(
            &self,
            pasteboard: &NSPasteboard,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithDataIgnoringOrientation:)]
        pub unsafe fn initWithDataIgnoringOrientation(
            &self,
            data: &NSData,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(imageWithSize:flipped:drawingHandler:)]
        pub unsafe fn imageWithSize_flipped_drawingHandler(
            size: NSSize,
            drawingHandlerShouldBeCalledWithFlippedContext: bool,
            drawingHandler: TodoBlock,
        ) -> Id<Self, Shared>;
        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);
        #[method(setName:)]
        pub unsafe fn setName(&self, string: Option<&NSImageName>) -> bool;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Id<NSImageName, Shared>>;
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: &NSColor);
        #[method(usesEPSOnResolutionMismatch)]
        pub unsafe fn usesEPSOnResolutionMismatch(&self) -> bool;
        #[method(setUsesEPSOnResolutionMismatch:)]
        pub unsafe fn setUsesEPSOnResolutionMismatch(&self, usesEPSOnResolutionMismatch: bool);
        #[method(prefersColorMatch)]
        pub unsafe fn prefersColorMatch(&self) -> bool;
        #[method(setPrefersColorMatch:)]
        pub unsafe fn setPrefersColorMatch(&self, prefersColorMatch: bool);
        #[method(matchesOnMultipleResolution)]
        pub unsafe fn matchesOnMultipleResolution(&self) -> bool;
        #[method(setMatchesOnMultipleResolution:)]
        pub unsafe fn setMatchesOnMultipleResolution(&self, matchesOnMultipleResolution: bool);
        #[method(matchesOnlyOnBestFittingAxis)]
        pub unsafe fn matchesOnlyOnBestFittingAxis(&self) -> bool;
        #[method(setMatchesOnlyOnBestFittingAxis:)]
        pub unsafe fn setMatchesOnlyOnBestFittingAxis(&self, matchesOnlyOnBestFittingAxis: bool);
        #[method(drawAtPoint:fromRect:operation:fraction:)]
        pub unsafe fn drawAtPoint_fromRect_operation_fraction(
            &self,
            point: NSPoint,
            fromRect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );
        #[method(drawInRect:fromRect:operation:fraction:)]
        pub unsafe fn drawInRect_fromRect_operation_fraction(
            &self,
            rect: NSRect,
            fromRect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );
        #[method(drawInRect:fromRect:operation:fraction:respectFlipped:hints:)]
        pub unsafe fn drawInRect_fromRect_operation_fraction_respectFlipped_hints(
            &self,
            dstSpacePortionRect: NSRect,
            srcSpacePortionRect: NSRect,
            op: NSCompositingOperation,
            requestedAlpha: CGFloat,
            respectContextIsFlipped: bool,
            hints: Option<&NSDictionary<NSImageHintKey, Object>>,
        );
        #[method(drawRepresentation:inRect:)]
        pub unsafe fn drawRepresentation_inRect(&self, imageRep: &NSImageRep, rect: NSRect)
            -> bool;
        #[method(drawInRect:)]
        pub unsafe fn drawInRect(&self, rect: NSRect);
        #[method(recache)]
        pub unsafe fn recache(&self);
        #[method_id(TIFFRepresentation)]
        pub unsafe fn TIFFRepresentation(&self) -> Option<Id<NSData, Shared>>;
        #[method_id(TIFFRepresentationUsingCompression:factor:)]
        pub unsafe fn TIFFRepresentationUsingCompression_factor(
            &self,
            comp: NSTIFFCompression,
            factor: c_float,
        ) -> Option<Id<NSData, Shared>>;
        #[method_id(representations)]
        pub unsafe fn representations(&self) -> Id<NSArray<NSImageRep>, Shared>;
        #[method(addRepresentations:)]
        pub unsafe fn addRepresentations(&self, imageReps: &NSArray<NSImageRep>);
        #[method(addRepresentation:)]
        pub unsafe fn addRepresentation(&self, imageRep: &NSImageRep);
        #[method(removeRepresentation:)]
        pub unsafe fn removeRepresentation(&self, imageRep: &NSImageRep);
        #[method(isValid)]
        pub unsafe fn isValid(&self) -> bool;
        #[method(lockFocus)]
        pub unsafe fn lockFocus(&self);
        #[method(lockFocusFlipped:)]
        pub unsafe fn lockFocusFlipped(&self, flipped: bool);
        #[method(unlockFocus)]
        pub unsafe fn unlockFocus(&self);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSImageDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSImageDelegate>);
        #[method_id(imageTypes)]
        pub unsafe fn imageTypes() -> Id<NSArray<NSString>, Shared>;
        #[method_id(imageUnfilteredTypes)]
        pub unsafe fn imageUnfilteredTypes() -> Id<NSArray<NSString>, Shared>;
        #[method(canInitWithPasteboard:)]
        pub unsafe fn canInitWithPasteboard(pasteboard: &NSPasteboard) -> bool;
        #[method(cancelIncrementalLoad)]
        pub unsafe fn cancelIncrementalLoad(&self);
        #[method(cacheMode)]
        pub unsafe fn cacheMode(&self) -> NSImageCacheMode;
        #[method(setCacheMode:)]
        pub unsafe fn setCacheMode(&self, cacheMode: NSImageCacheMode);
        #[method(alignmentRect)]
        pub unsafe fn alignmentRect(&self) -> NSRect;
        #[method(setAlignmentRect:)]
        pub unsafe fn setAlignmentRect(&self, alignmentRect: NSRect);
        #[method(isTemplate)]
        pub unsafe fn isTemplate(&self) -> bool;
        #[method(setTemplate:)]
        pub unsafe fn setTemplate(&self, template: bool);
        #[method_id(accessibilityDescription)]
        pub unsafe fn accessibilityDescription(&self) -> Option<Id<NSString, Shared>>;
        #[method(setAccessibilityDescription:)]
        pub unsafe fn setAccessibilityDescription(
            &self,
            accessibilityDescription: Option<&NSString>,
        );
        #[method_id(initWithCGImage:size:)]
        pub unsafe fn initWithCGImage_size(
            &self,
            cgImage: CGImageRef,
            size: NSSize,
        ) -> Id<Self, Shared>;
        #[method(CGImageForProposedRect:context:hints:)]
        pub unsafe fn CGImageForProposedRect_context_hints(
            &self,
            proposedDestRect: *mut NSRect,
            referenceContext: Option<&NSGraphicsContext>,
            hints: Option<&NSDictionary<NSImageHintKey, Object>>,
        ) -> CGImageRef;
        #[method_id(bestRepresentationForRect:context:hints:)]
        pub unsafe fn bestRepresentationForRect_context_hints(
            &self,
            rect: NSRect,
            referenceContext: Option<&NSGraphicsContext>,
            hints: Option<&NSDictionary<NSImageHintKey, Object>>,
        ) -> Option<Id<NSImageRep, Shared>>;
        #[method(hitTestRect:withImageDestinationRect:context:hints:flipped:)]
        pub unsafe fn hitTestRect_withImageDestinationRect_context_hints_flipped(
            &self,
            testRectDestSpace: NSRect,
            imageRectDestSpace: NSRect,
            context: Option<&NSGraphicsContext>,
            hints: Option<&NSDictionary<NSImageHintKey, Object>>,
            flipped: bool,
        ) -> bool;
        #[method(recommendedLayerContentsScale:)]
        pub unsafe fn recommendedLayerContentsScale(
            &self,
            preferredContentsScale: CGFloat,
        ) -> CGFloat;
        #[method_id(layerContentsForContentsScale:)]
        pub unsafe fn layerContentsForContentsScale(
            &self,
            layerContentsScale: CGFloat,
        ) -> Id<Object, Shared>;
        #[method(capInsets)]
        pub unsafe fn capInsets(&self) -> NSEdgeInsets;
        #[method(setCapInsets:)]
        pub unsafe fn setCapInsets(&self, capInsets: NSEdgeInsets);
        #[method(resizingMode)]
        pub unsafe fn resizingMode(&self) -> NSImageResizingMode;
        #[method(setResizingMode:)]
        pub unsafe fn setResizingMode(&self, resizingMode: NSImageResizingMode);
        #[method_id(imageWithSymbolConfiguration:)]
        pub unsafe fn imageWithSymbolConfiguration(
            &self,
            configuration: &NSImageSymbolConfiguration,
        ) -> Option<Id<NSImage, Shared>>;
        #[method_id(symbolConfiguration)]
        pub unsafe fn symbolConfiguration(&self) -> Id<NSImageSymbolConfiguration, Shared>;
    }
);
extern_methods!(
    unsafe impl NSImage {}
);
pub type NSImageDelegate = NSObject;
extern_methods!(
    #[doc = "NSBundleImageExtension"]
    unsafe impl NSBundle {
        #[method_id(imageForResource:)]
        pub unsafe fn imageForResource(&self, name: &NSImageName) -> Option<Id<NSImage, Shared>>;
        #[method_id(pathForImageResource:)]
        pub unsafe fn pathForImageResource(
            &self,
            name: &NSImageName,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(URLForImageResource:)]
        pub unsafe fn URLForImageResource(&self, name: &NSImageName) -> Option<Id<NSURL, Shared>>;
    }
);
extern_methods!(
    unsafe impl NSImage {
        #[method_id(bestRepresentationForDevice:)]
        pub unsafe fn bestRepresentationForDevice(
            &self,
            deviceDescription: Option<&NSDictionary>,
        ) -> Option<Id<NSImageRep, Shared>>;
        #[method_id(imageUnfilteredFileTypes)]
        pub unsafe fn imageUnfilteredFileTypes() -> Id<NSArray<NSString>, Shared>;
        #[method_id(imageUnfilteredPasteboardTypes)]
        pub unsafe fn imageUnfilteredPasteboardTypes() -> Id<NSArray<NSPasteboardType>, Shared>;
        #[method_id(imageFileTypes)]
        pub unsafe fn imageFileTypes() -> Id<NSArray<NSString>, Shared>;
        #[method_id(imagePasteboardTypes)]
        pub unsafe fn imagePasteboardTypes() -> Id<NSArray<NSPasteboardType>, Shared>;
        #[method_id(initWithIconRef:)]
        pub unsafe fn initWithIconRef(&self, iconRef: IconRef) -> Id<Self, Shared>;
        #[method(setFlipped:)]
        pub unsafe fn setFlipped(&self, flag: bool);
        #[method(isFlipped)]
        pub unsafe fn isFlipped(&self) -> bool;
        #[method(dissolveToPoint:fraction:)]
        pub unsafe fn dissolveToPoint_fraction(&self, point: NSPoint, fraction: CGFloat);
        #[method(dissolveToPoint:fromRect:fraction:)]
        pub unsafe fn dissolveToPoint_fromRect_fraction(
            &self,
            point: NSPoint,
            rect: NSRect,
            fraction: CGFloat,
        );
        #[method(compositeToPoint:operation:)]
        pub unsafe fn compositeToPoint_operation(&self, point: NSPoint, op: NSCompositingOperation);
        #[method(compositeToPoint:fromRect:operation:)]
        pub unsafe fn compositeToPoint_fromRect_operation(
            &self,
            point: NSPoint,
            rect: NSRect,
            op: NSCompositingOperation,
        );
        #[method(compositeToPoint:operation:fraction:)]
        pub unsafe fn compositeToPoint_operation_fraction(
            &self,
            point: NSPoint,
            op: NSCompositingOperation,
            delta: CGFloat,
        );
        #[method(compositeToPoint:fromRect:operation:fraction:)]
        pub unsafe fn compositeToPoint_fromRect_operation_fraction(
            &self,
            point: NSPoint,
            rect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );
        #[method(lockFocusOnRepresentation:)]
        pub unsafe fn lockFocusOnRepresentation(&self, imageRepresentation: Option<&NSImageRep>);
        #[method(setScalesWhenResized:)]
        pub unsafe fn setScalesWhenResized(&self, flag: bool);
        #[method(scalesWhenResized)]
        pub unsafe fn scalesWhenResized(&self) -> bool;
        #[method(setDataRetained:)]
        pub unsafe fn setDataRetained(&self, flag: bool);
        #[method(isDataRetained)]
        pub unsafe fn isDataRetained(&self) -> bool;
        #[method(setCachedSeparately:)]
        pub unsafe fn setCachedSeparately(&self, flag: bool);
        #[method(isCachedSeparately)]
        pub unsafe fn isCachedSeparately(&self) -> bool;
        #[method(setCacheDepthMatchesImageDepth:)]
        pub unsafe fn setCacheDepthMatchesImageDepth(&self, flag: bool);
        #[method(cacheDepthMatchesImageDepth)]
        pub unsafe fn cacheDepthMatchesImageDepth(&self) -> bool;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSImageSymbolConfiguration;
    unsafe impl ClassType for NSImageSymbolConfiguration {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSImageSymbolConfiguration {
        #[method_id(configurationWithPointSize:weight:scale:)]
        pub unsafe fn configurationWithPointSize_weight_scale(
            pointSize: CGFloat,
            weight: NSFontWeight,
            scale: NSImageSymbolScale,
        ) -> Id<Self, Shared>;
        #[method_id(configurationWithPointSize:weight:)]
        pub unsafe fn configurationWithPointSize_weight(
            pointSize: CGFloat,
            weight: NSFontWeight,
        ) -> Id<Self, Shared>;
        #[method_id(configurationWithTextStyle:scale:)]
        pub unsafe fn configurationWithTextStyle_scale(
            style: &NSFontTextStyle,
            scale: NSImageSymbolScale,
        ) -> Id<Self, Shared>;
        #[method_id(configurationWithTextStyle:)]
        pub unsafe fn configurationWithTextStyle(style: &NSFontTextStyle) -> Id<Self, Shared>;
        #[method_id(configurationWithScale:)]
        pub unsafe fn configurationWithScale(scale: NSImageSymbolScale) -> Id<Self, Shared>;
        #[method_id(configurationWithHierarchicalColor:)]
        pub unsafe fn configurationWithHierarchicalColor(
            hierarchicalColor: &NSColor,
        ) -> Id<Self, Shared>;
        #[method_id(configurationWithPaletteColors:)]
        pub unsafe fn configurationWithPaletteColors(
            paletteColors: &NSArray<NSColor>,
        ) -> Id<Self, Shared>;
        #[method_id(configurationPreferringMulticolor)]
        pub unsafe fn configurationPreferringMulticolor() -> Id<Self, Shared>;
        #[method_id(configurationByApplyingConfiguration:)]
        pub unsafe fn configurationByApplyingConfiguration(
            &self,
            configuration: &NSImageSymbolConfiguration,
        ) -> Id<Self, Shared>;
    }
);
