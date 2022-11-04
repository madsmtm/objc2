//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSGraphicsContextAttributeKey = NSString;

extern_static!(NSGraphicsContextDestinationAttributeName: &'static NSGraphicsContextAttributeKey);

extern_static!(
    NSGraphicsContextRepresentationFormatAttributeName: &'static NSGraphicsContextAttributeKey
);

pub type NSGraphicsContextRepresentationFormatName = NSString;

extern_static!(NSGraphicsContextPSFormat: &'static NSGraphicsContextRepresentationFormatName);

extern_static!(NSGraphicsContextPDFFormat: &'static NSGraphicsContextRepresentationFormatName);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSImageInterpolation {
        NSImageInterpolationDefault = 0,
        NSImageInterpolationNone = 1,
        NSImageInterpolationLow = 2,
        NSImageInterpolationMedium = 4,
        NSImageInterpolationHigh = 3,
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSGraphicsContext;

    unsafe impl ClassType for NSGraphicsContext {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSGraphicsContext {
        #[method_id(@__retain_semantics Other graphicsContextWithAttributes:)]
        pub unsafe fn graphicsContextWithAttributes(
            attributes: &NSDictionary<NSGraphicsContextAttributeKey, Object>,
        ) -> Option<Id<NSGraphicsContext, Shared>>;

        #[method_id(@__retain_semantics Other graphicsContextWithWindow:)]
        pub unsafe fn graphicsContextWithWindow(window: &NSWindow)
            -> Id<NSGraphicsContext, Shared>;

        #[method_id(@__retain_semantics Other graphicsContextWithBitmapImageRep:)]
        pub unsafe fn graphicsContextWithBitmapImageRep(
            bitmapRep: &NSBitmapImageRep,
        ) -> Option<Id<NSGraphicsContext, Shared>>;

        #[method_id(@__retain_semantics Other currentContext)]
        pub unsafe fn currentContext() -> Option<Id<NSGraphicsContext, Shared>>;

        #[method(setCurrentContext:)]
        pub unsafe fn setCurrentContext(currentContext: Option<&NSGraphicsContext>);

        #[method(currentContextDrawingToScreen)]
        pub unsafe fn currentContextDrawingToScreen() -> bool;

        #[method_id(@__retain_semantics Other attributes)]
        pub unsafe fn attributes(
            &self,
        ) -> Option<Id<NSDictionary<NSGraphicsContextAttributeKey, Object>, Shared>>;

        #[method(isDrawingToScreen)]
        pub unsafe fn isDrawingToScreen(&self) -> bool;

        #[method(flushGraphics)]
        pub unsafe fn flushGraphics(&self);

        #[method(isFlipped)]
        pub unsafe fn isFlipped(&self) -> bool;
    }
);

extern_methods!(
    /// NSGraphicsContext_RenderingOptions
    unsafe impl NSGraphicsContext {
        #[method(shouldAntialias)]
        pub unsafe fn shouldAntialias(&self) -> bool;

        #[method(setShouldAntialias:)]
        pub unsafe fn setShouldAntialias(&self, shouldAntialias: bool);

        #[method(imageInterpolation)]
        pub unsafe fn imageInterpolation(&self) -> NSImageInterpolation;

        #[method(setImageInterpolation:)]
        pub unsafe fn setImageInterpolation(&self, imageInterpolation: NSImageInterpolation);

        #[method(patternPhase)]
        pub unsafe fn patternPhase(&self) -> NSPoint;

        #[method(setPatternPhase:)]
        pub unsafe fn setPatternPhase(&self, patternPhase: NSPoint);

        #[method(compositingOperation)]
        pub unsafe fn compositingOperation(&self) -> NSCompositingOperation;

        #[method(setCompositingOperation:)]
        pub unsafe fn setCompositingOperation(&self, compositingOperation: NSCompositingOperation);

        #[method(colorRenderingIntent)]
        pub unsafe fn colorRenderingIntent(&self) -> NSColorRenderingIntent;

        #[method(setColorRenderingIntent:)]
        pub unsafe fn setColorRenderingIntent(&self, colorRenderingIntent: NSColorRenderingIntent);
    }
);

extern_methods!(
    /// NSQuartzCoreAdditions
    unsafe impl NSGraphicsContext {}
);

extern_methods!(
    /// NSGraphicsContextDeprecated
    unsafe impl NSGraphicsContext {
        #[method(setGraphicsState:)]
        pub unsafe fn setGraphicsState(gState: NSInteger);

        #[method_id(@__retain_semantics Other focusStack)]
        pub unsafe fn focusStack(&self) -> Option<Id<Object, Shared>>;

        #[method(setFocusStack:)]
        pub unsafe fn setFocusStack(&self, stack: Option<&Object>);

        #[method_id(@__retain_semantics Other graphicsContextWithGraphicsPort:flipped:)]
        pub unsafe fn graphicsContextWithGraphicsPort_flipped(
            graphicsPort: NonNull<c_void>,
            initialFlippedState: bool,
        ) -> Id<NSGraphicsContext, Shared>;

        #[method(graphicsPort)]
        pub unsafe fn graphicsPort(&self) -> NonNull<c_void>;
    }
);
