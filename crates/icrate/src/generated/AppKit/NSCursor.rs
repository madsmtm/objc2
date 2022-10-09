use super::__exported::NSColor;
use super::__exported::NSEvent;
use super::__exported::NSImage;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSApplication::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCursor;
    unsafe impl ClassType for NSCursor {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSCursor {
        #[method_id(currentCursor)]
        pub unsafe fn currentCursor() -> Id<NSCursor, Shared>;
        #[method_id(currentSystemCursor)]
        pub unsafe fn currentSystemCursor() -> Option<Id<NSCursor, Shared>>;
        #[method_id(arrowCursor)]
        pub unsafe fn arrowCursor() -> Id<NSCursor, Shared>;
        #[method_id(IBeamCursor)]
        pub unsafe fn IBeamCursor() -> Id<NSCursor, Shared>;
        #[method_id(pointingHandCursor)]
        pub unsafe fn pointingHandCursor() -> Id<NSCursor, Shared>;
        #[method_id(closedHandCursor)]
        pub unsafe fn closedHandCursor() -> Id<NSCursor, Shared>;
        #[method_id(openHandCursor)]
        pub unsafe fn openHandCursor() -> Id<NSCursor, Shared>;
        #[method_id(resizeLeftCursor)]
        pub unsafe fn resizeLeftCursor() -> Id<NSCursor, Shared>;
        #[method_id(resizeRightCursor)]
        pub unsafe fn resizeRightCursor() -> Id<NSCursor, Shared>;
        #[method_id(resizeLeftRightCursor)]
        pub unsafe fn resizeLeftRightCursor() -> Id<NSCursor, Shared>;
        #[method_id(resizeUpCursor)]
        pub unsafe fn resizeUpCursor() -> Id<NSCursor, Shared>;
        #[method_id(resizeDownCursor)]
        pub unsafe fn resizeDownCursor() -> Id<NSCursor, Shared>;
        #[method_id(resizeUpDownCursor)]
        pub unsafe fn resizeUpDownCursor() -> Id<NSCursor, Shared>;
        #[method_id(crosshairCursor)]
        pub unsafe fn crosshairCursor() -> Id<NSCursor, Shared>;
        #[method_id(disappearingItemCursor)]
        pub unsafe fn disappearingItemCursor() -> Id<NSCursor, Shared>;
        #[method_id(operationNotAllowedCursor)]
        pub unsafe fn operationNotAllowedCursor() -> Id<NSCursor, Shared>;
        #[method_id(dragLinkCursor)]
        pub unsafe fn dragLinkCursor() -> Id<NSCursor, Shared>;
        #[method_id(dragCopyCursor)]
        pub unsafe fn dragCopyCursor() -> Id<NSCursor, Shared>;
        #[method_id(contextualMenuCursor)]
        pub unsafe fn contextualMenuCursor() -> Id<NSCursor, Shared>;
        #[method_id(IBeamCursorForVerticalLayout)]
        pub unsafe fn IBeamCursorForVerticalLayout() -> Id<NSCursor, Shared>;
        #[method_id(initWithImage:hotSpot:)]
        pub unsafe fn initWithImage_hotSpot(
            &self,
            newImage: &NSImage,
            point: NSPoint,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method(hide)]
        pub unsafe fn hide();
        #[method(unhide)]
        pub unsafe fn unhide();
        #[method(setHiddenUntilMouseMoves:)]
        pub unsafe fn setHiddenUntilMouseMoves(flag: bool);
        #[method(pop)]
        pub unsafe fn pop();
        #[method_id(image)]
        pub unsafe fn image(&self) -> Id<NSImage, Shared>;
        #[method(hotSpot)]
        pub unsafe fn hotSpot(&self) -> NSPoint;
        #[method(push)]
        pub unsafe fn push(&self);
        #[method(pop)]
        pub unsafe fn pop(&self);
        #[method(set)]
        pub unsafe fn set(&self);
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSCursor {
        #[method_id(initWithImage:foregroundColorHint:backgroundColorHint:hotSpot:)]
        pub unsafe fn initWithImage_foregroundColorHint_backgroundColorHint_hotSpot(
            &self,
            newImage: &NSImage,
            fg: Option<&NSColor>,
            bg: Option<&NSColor>,
            hotSpot: NSPoint,
        ) -> Id<Self, Shared>;
        #[method(setOnMouseExited:)]
        pub unsafe fn setOnMouseExited(&self, flag: bool);
        #[method(setOnMouseEntered:)]
        pub unsafe fn setOnMouseEntered(&self, flag: bool);
        #[method(isSetOnMouseExited)]
        pub unsafe fn isSetOnMouseExited(&self) -> bool;
        #[method(isSetOnMouseEntered)]
        pub unsafe fn isSetOnMouseEntered(&self) -> bool;
        #[method(mouseEntered:)]
        pub unsafe fn mouseEntered(&self, event: &NSEvent);
        #[method(mouseExited:)]
        pub unsafe fn mouseExited(&self, event: &NSEvent);
    }
);
