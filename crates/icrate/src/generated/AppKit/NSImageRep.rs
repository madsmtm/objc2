use super::__exported::NSGraphicsContext;
use super::__exported::NSPasteboard;
use super::__exported::NSURL;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSColorSpace::*;
use crate::AppKit::generated::NSGraphics::*;
use crate::AppKit::generated::NSPasteboard::*;
use crate::AppKit::generated::NSUserInterfaceLayout::*;
use crate::ApplicationServices::generated::ApplicationServices::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSImageHintKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSImageRep;
    unsafe impl ClassType for NSImageRep {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSImageRep {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method(draw)]
        pub unsafe fn draw(&self) -> bool;
        #[method(drawAtPoint:)]
        pub unsafe fn drawAtPoint(&self, point: NSPoint) -> bool;
        #[method(drawInRect:)]
        pub unsafe fn drawInRect(&self, rect: NSRect) -> bool;
        #[method(drawInRect:fromRect:operation:fraction:respectFlipped:hints:)]
        pub unsafe fn drawInRect_fromRect_operation_fraction_respectFlipped_hints(
            &self,
            dstSpacePortionRect: NSRect,
            srcSpacePortionRect: NSRect,
            op: NSCompositingOperation,
            requestedAlpha: CGFloat,
            respectContextIsFlipped: bool,
            hints: Option<&NSDictionary<NSImageHintKey, Object>>,
        ) -> bool;
        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);
        #[method(hasAlpha)]
        pub unsafe fn hasAlpha(&self) -> bool;
        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: bool);
        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;
        #[method(setOpaque:)]
        pub unsafe fn setOpaque(&self, opaque: bool);
        #[method_id(colorSpaceName)]
        pub unsafe fn colorSpaceName(&self) -> Id<NSColorSpaceName, Shared>;
        #[method(setColorSpaceName:)]
        pub unsafe fn setColorSpaceName(&self, colorSpaceName: &NSColorSpaceName);
        #[method(bitsPerSample)]
        pub unsafe fn bitsPerSample(&self) -> NSInteger;
        #[method(setBitsPerSample:)]
        pub unsafe fn setBitsPerSample(&self, bitsPerSample: NSInteger);
        #[method(pixelsWide)]
        pub unsafe fn pixelsWide(&self) -> NSInteger;
        #[method(setPixelsWide:)]
        pub unsafe fn setPixelsWide(&self, pixelsWide: NSInteger);
        #[method(pixelsHigh)]
        pub unsafe fn pixelsHigh(&self) -> NSInteger;
        #[method(setPixelsHigh:)]
        pub unsafe fn setPixelsHigh(&self, pixelsHigh: NSInteger);
        #[method(layoutDirection)]
        pub unsafe fn layoutDirection(&self) -> NSImageLayoutDirection;
        #[method(setLayoutDirection:)]
        pub unsafe fn setLayoutDirection(&self, layoutDirection: NSImageLayoutDirection);
        #[method(registerImageRepClass:)]
        pub unsafe fn registerImageRepClass(imageRepClass: &Class);
        #[method(unregisterImageRepClass:)]
        pub unsafe fn unregisterImageRepClass(imageRepClass: &Class);
        #[method_id(registeredImageRepClasses)]
        pub unsafe fn registeredImageRepClasses() -> Id<NSArray<TodoClass>, Shared>;
        #[method(imageRepClassForFileType:)]
        pub unsafe fn imageRepClassForFileType(type_: &NSString) -> Option<&Class>;
        #[method(imageRepClassForPasteboardType:)]
        pub unsafe fn imageRepClassForPasteboardType(type_: &NSPasteboardType) -> Option<&Class>;
        #[method(imageRepClassForType:)]
        pub unsafe fn imageRepClassForType(type_: &NSString) -> Option<&Class>;
        #[method(imageRepClassForData:)]
        pub unsafe fn imageRepClassForData(data: &NSData) -> Option<&Class>;
        #[method(canInitWithData:)]
        pub unsafe fn canInitWithData(data: &NSData) -> bool;
        #[method_id(imageUnfilteredFileTypes)]
        pub unsafe fn imageUnfilteredFileTypes() -> Id<NSArray<NSString>, Shared>;
        #[method_id(imageUnfilteredPasteboardTypes)]
        pub unsafe fn imageUnfilteredPasteboardTypes() -> Id<NSArray<NSPasteboardType>, Shared>;
        #[method_id(imageFileTypes)]
        pub unsafe fn imageFileTypes() -> Id<NSArray<NSString>, Shared>;
        #[method_id(imagePasteboardTypes)]
        pub unsafe fn imagePasteboardTypes() -> Id<NSArray<NSPasteboardType>, Shared>;
        #[method_id(imageUnfilteredTypes)]
        pub unsafe fn imageUnfilteredTypes() -> Id<NSArray<NSString>, Shared>;
        #[method_id(imageTypes)]
        pub unsafe fn imageTypes() -> Id<NSArray<NSString>, Shared>;
        #[method(canInitWithPasteboard:)]
        pub unsafe fn canInitWithPasteboard(pasteboard: &NSPasteboard) -> bool;
        #[method_id(imageRepsWithContentsOfFile:)]
        pub unsafe fn imageRepsWithContentsOfFile(
            filename: &NSString,
        ) -> Option<Id<NSArray<NSImageRep>, Shared>>;
        #[method_id(imageRepWithContentsOfFile:)]
        pub unsafe fn imageRepWithContentsOfFile(
            filename: &NSString,
        ) -> Option<Id<NSImageRep, Shared>>;
        #[method_id(imageRepsWithContentsOfURL:)]
        pub unsafe fn imageRepsWithContentsOfURL(
            url: &NSURL,
        ) -> Option<Id<NSArray<NSImageRep>, Shared>>;
        #[method_id(imageRepWithContentsOfURL:)]
        pub unsafe fn imageRepWithContentsOfURL(url: &NSURL) -> Option<Id<NSImageRep, Shared>>;
        #[method_id(imageRepsWithPasteboard:)]
        pub unsafe fn imageRepsWithPasteboard(
            pasteboard: &NSPasteboard,
        ) -> Option<Id<NSArray<NSImageRep>, Shared>>;
        #[method_id(imageRepWithPasteboard:)]
        pub unsafe fn imageRepWithPasteboard(
            pasteboard: &NSPasteboard,
        ) -> Option<Id<NSImageRep, Shared>>;
        #[method(CGImageForProposedRect:context:hints:)]
        pub unsafe fn CGImageForProposedRect_context_hints(
            &self,
            proposedDestRect: *mut NSRect,
            context: Option<&NSGraphicsContext>,
            hints: Option<&NSDictionary<NSImageHintKey, Object>>,
        ) -> CGImageRef;
    }
);
