#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSGraphicsContextAttributeKey = NSString;
pub type NSGraphicsContextRepresentationFormatName = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSGraphicsContext;
    unsafe impl ClassType for NSGraphicsContext {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSGraphicsContext {
        #[method_id(graphicsContextWithAttributes:)]
        pub unsafe fn graphicsContextWithAttributes(
            attributes: &NSDictionary<NSGraphicsContextAttributeKey, Object>,
        ) -> Option<Id<NSGraphicsContext, Shared>>;
        #[method_id(graphicsContextWithWindow:)]
        pub unsafe fn graphicsContextWithWindow(window: &NSWindow)
            -> Id<NSGraphicsContext, Shared>;
        #[method_id(graphicsContextWithBitmapImageRep:)]
        pub unsafe fn graphicsContextWithBitmapImageRep(
            bitmapRep: &NSBitmapImageRep,
        ) -> Option<Id<NSGraphicsContext, Shared>>;
        #[method_id(graphicsContextWithCGContext:flipped:)]
        pub unsafe fn graphicsContextWithCGContext_flipped(
            graphicsPort: CGContextRef,
            initialFlippedState: bool,
        ) -> Id<NSGraphicsContext, Shared>;
        #[method_id(currentContext)]
        pub unsafe fn currentContext() -> Option<Id<NSGraphicsContext, Shared>>;
        #[method(setCurrentContext:)]
        pub unsafe fn setCurrentContext(currentContext: Option<&NSGraphicsContext>);
        #[method(currentContextDrawingToScreen)]
        pub unsafe fn currentContextDrawingToScreen() -> bool;
        #[method(saveGraphicsState)]
        pub unsafe fn saveGraphicsState();
        #[method(restoreGraphicsState)]
        pub unsafe fn restoreGraphicsState();
        #[method_id(attributes)]
        pub unsafe fn attributes(
            &self,
        ) -> Option<Id<NSDictionary<NSGraphicsContextAttributeKey, Object>, Shared>>;
        #[method(isDrawingToScreen)]
        pub unsafe fn isDrawingToScreen(&self) -> bool;
        #[method(saveGraphicsState)]
        pub unsafe fn saveGraphicsState(&self);
        #[method(restoreGraphicsState)]
        pub unsafe fn restoreGraphicsState(&self);
        #[method(flushGraphics)]
        pub unsafe fn flushGraphics(&self);
        #[method(CGContext)]
        pub unsafe fn CGContext(&self) -> CGContextRef;
        #[method(isFlipped)]
        pub unsafe fn isFlipped(&self) -> bool;
    }
);
extern_methods!(
    #[doc = "NSGraphicsContext_RenderingOptions"]
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
    #[doc = "NSQuartzCoreAdditions"]
    unsafe impl NSGraphicsContext {
        #[method_id(CIContext)]
        pub unsafe fn CIContext(&self) -> Option<Id<CIContext, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSGraphicsContextDeprecated"]
    unsafe impl NSGraphicsContext {
        #[method(setGraphicsState:)]
        pub unsafe fn setGraphicsState(gState: NSInteger);
        #[method_id(focusStack)]
        pub unsafe fn focusStack(&self) -> Option<Id<Object, Shared>>;
        #[method(setFocusStack:)]
        pub unsafe fn setFocusStack(&self, stack: Option<&Object>);
        #[method_id(graphicsContextWithGraphicsPort:flipped:)]
        pub unsafe fn graphicsContextWithGraphicsPort_flipped(
            graphicsPort: NonNull<c_void>,
            initialFlippedState: bool,
        ) -> Id<NSGraphicsContext, Shared>;
        #[method(graphicsPort)]
        pub unsafe fn graphicsPort(&self) -> NonNull<c_void>;
    }
);
