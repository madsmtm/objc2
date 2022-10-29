#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSStringDrawingContext;
    unsafe impl ClassType for NSStringDrawingContext {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSStringDrawingContext {
        #[method(minimumScaleFactor)]
        pub unsafe fn minimumScaleFactor(&self) -> CGFloat;
        #[method(setMinimumScaleFactor:)]
        pub unsafe fn setMinimumScaleFactor(&self, minimumScaleFactor: CGFloat);
        #[method(actualScaleFactor)]
        pub unsafe fn actualScaleFactor(&self) -> CGFloat;
        #[method(totalBounds)]
        pub unsafe fn totalBounds(&self) -> NSRect;
    }
);
extern_methods!(
    #[doc = "NSStringDrawing"]
    unsafe impl NSString {
        #[method(sizeWithAttributes:)]
        pub unsafe fn sizeWithAttributes(
            &self,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        ) -> NSSize;
        #[method(drawAtPoint:withAttributes:)]
        pub unsafe fn drawAtPoint_withAttributes(
            &self,
            point: NSPoint,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        );
        #[method(drawInRect:withAttributes:)]
        pub unsafe fn drawInRect_withAttributes(
            &self,
            rect: NSRect,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        );
    }
);
extern_methods!(
    #[doc = "NSStringDrawing"]
    unsafe impl NSAttributedString {
        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;
        #[method(drawAtPoint:)]
        pub unsafe fn drawAtPoint(&self, point: NSPoint);
        #[method(drawInRect:)]
        pub unsafe fn drawInRect(&self, rect: NSRect);
    }
);
extern_methods!(
    #[doc = "NSExtendedStringDrawing"]
    unsafe impl NSString {
        #[method(drawWithRect:options:attributes:context:)]
        pub unsafe fn drawWithRect_options_attributes_context(
            &self,
            rect: NSRect,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, Object>>,
            context: Option<&NSStringDrawingContext>,
        );
        #[method(boundingRectWithSize:options:attributes:context:)]
        pub unsafe fn boundingRectWithSize_options_attributes_context(
            &self,
            size: NSSize,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, Object>>,
            context: Option<&NSStringDrawingContext>,
        ) -> NSRect;
    }
);
extern_methods!(
    #[doc = "NSExtendedStringDrawing"]
    unsafe impl NSAttributedString {
        #[method(drawWithRect:options:context:)]
        pub unsafe fn drawWithRect_options_context(
            &self,
            rect: NSRect,
            options: NSStringDrawingOptions,
            context: Option<&NSStringDrawingContext>,
        );
        #[method(boundingRectWithSize:options:context:)]
        pub unsafe fn boundingRectWithSize_options_context(
            &self,
            size: NSSize,
            options: NSStringDrawingOptions,
            context: Option<&NSStringDrawingContext>,
        ) -> NSRect;
    }
);
extern_methods!(
    #[doc = "NSStringDrawingDeprecated"]
    unsafe impl NSString {
        #[method(drawWithRect:options:attributes:)]
        pub unsafe fn drawWithRect_options_attributes(
            &self,
            rect: NSRect,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        );
        #[method(boundingRectWithSize:options:attributes:)]
        pub unsafe fn boundingRectWithSize_options_attributes(
            &self,
            size: NSSize,
            options: NSStringDrawingOptions,
            attributes: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        ) -> NSRect;
    }
);
extern_methods!(
    #[doc = "NSStringDrawingDeprecated"]
    unsafe impl NSAttributedString {
        #[method(drawWithRect:options:)]
        pub unsafe fn drawWithRect_options(&self, rect: NSRect, options: NSStringDrawingOptions);
        #[method(boundingRectWithSize:options:)]
        pub unsafe fn boundingRectWithSize_options(
            &self,
            size: NSSize,
            options: NSStringDrawingOptions,
        ) -> NSRect;
    }
);
