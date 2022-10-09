use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSButtonCell::*;
use crate::AppKit::generated::NSMenu::*;
use crate::AppKit::generated::NSMenuItem::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSMenuItemCell;
    unsafe impl ClassType for NSMenuItemCell {
        type Super = NSButtonCell;
    }
);
extern_methods!(
    unsafe impl NSMenuItemCell {
        #[method_id(initTextCell:)]
        pub unsafe fn initTextCell(&self, string: &NSString) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(menuItem)]
        pub unsafe fn menuItem(&self) -> Option<Id<NSMenuItem, Shared>>;
        #[method(setMenuItem:)]
        pub unsafe fn setMenuItem(&self, menuItem: Option<&NSMenuItem>);
        #[method(needsSizing)]
        pub unsafe fn needsSizing(&self) -> bool;
        #[method(setNeedsSizing:)]
        pub unsafe fn setNeedsSizing(&self, needsSizing: bool);
        #[method(calcSize)]
        pub unsafe fn calcSize(&self);
        #[method(needsDisplay)]
        pub unsafe fn needsDisplay(&self) -> bool;
        #[method(setNeedsDisplay:)]
        pub unsafe fn setNeedsDisplay(&self, needsDisplay: bool);
        #[method(stateImageWidth)]
        pub unsafe fn stateImageWidth(&self) -> CGFloat;
        #[method(imageWidth)]
        pub unsafe fn imageWidth(&self) -> CGFloat;
        #[method(titleWidth)]
        pub unsafe fn titleWidth(&self) -> CGFloat;
        #[method(keyEquivalentWidth)]
        pub unsafe fn keyEquivalentWidth(&self) -> CGFloat;
        #[method(stateImageRectForBounds:)]
        pub unsafe fn stateImageRectForBounds(&self, cellFrame: NSRect) -> NSRect;
        #[method(titleRectForBounds:)]
        pub unsafe fn titleRectForBounds(&self, cellFrame: NSRect) -> NSRect;
        #[method(keyEquivalentRectForBounds:)]
        pub unsafe fn keyEquivalentRectForBounds(&self, cellFrame: NSRect) -> NSRect;
        #[method(drawSeparatorItemWithFrame:inView:)]
        pub unsafe fn drawSeparatorItemWithFrame_inView(
            &self,
            cellFrame: NSRect,
            controlView: &NSView,
        );
        #[method(drawStateImageWithFrame:inView:)]
        pub unsafe fn drawStateImageWithFrame_inView(
            &self,
            cellFrame: NSRect,
            controlView: &NSView,
        );
        #[method(drawImageWithFrame:inView:)]
        pub unsafe fn drawImageWithFrame_inView(&self, cellFrame: NSRect, controlView: &NSView);
        #[method(drawTitleWithFrame:inView:)]
        pub unsafe fn drawTitleWithFrame_inView(&self, cellFrame: NSRect, controlView: &NSView);
        #[method(drawKeyEquivalentWithFrame:inView:)]
        pub unsafe fn drawKeyEquivalentWithFrame_inView(
            &self,
            cellFrame: NSRect,
            controlView: &NSView,
        );
        #[method(drawBorderAndBackgroundWithFrame:inView:)]
        pub unsafe fn drawBorderAndBackgroundWithFrame_inView(
            &self,
            cellFrame: NSRect,
            controlView: &NSView,
        );
        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;
        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);
    }
);
