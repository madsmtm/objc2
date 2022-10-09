use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSCell::*;
use crate::AppKit::generated::NSControl::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScroller;
    unsafe impl ClassType for NSScroller {
        type Super = NSControl;
    }
);
extern_methods!(
    unsafe impl NSScroller {
        #[method(isCompatibleWithOverlayScrollers)]
        pub unsafe fn isCompatibleWithOverlayScrollers() -> bool;
        #[method(scrollerWidthForControlSize:scrollerStyle:)]
        pub unsafe fn scrollerWidthForControlSize_scrollerStyle(
            controlSize: NSControlSize,
            scrollerStyle: NSScrollerStyle,
        ) -> CGFloat;
        #[method(preferredScrollerStyle)]
        pub unsafe fn preferredScrollerStyle() -> NSScrollerStyle;
        #[method(scrollerStyle)]
        pub unsafe fn scrollerStyle(&self) -> NSScrollerStyle;
        #[method(setScrollerStyle:)]
        pub unsafe fn setScrollerStyle(&self, scrollerStyle: NSScrollerStyle);
        #[method(knobStyle)]
        pub unsafe fn knobStyle(&self) -> NSScrollerKnobStyle;
        #[method(setKnobStyle:)]
        pub unsafe fn setKnobStyle(&self, knobStyle: NSScrollerKnobStyle);
        #[method(rectForPart:)]
        pub unsafe fn rectForPart(&self, partCode: NSScrollerPart) -> NSRect;
        #[method(checkSpaceForParts)]
        pub unsafe fn checkSpaceForParts(&self);
        #[method(usableParts)]
        pub unsafe fn usableParts(&self) -> NSUsableScrollerParts;
        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;
        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, controlSize: NSControlSize);
        #[method(drawKnob)]
        pub unsafe fn drawKnob(&self);
        #[method(drawKnobSlotInRect:highlight:)]
        pub unsafe fn drawKnobSlotInRect_highlight(&self, slotRect: NSRect, flag: bool);
        #[method(testPart:)]
        pub unsafe fn testPart(&self, point: NSPoint) -> NSScrollerPart;
        #[method(trackKnob:)]
        pub unsafe fn trackKnob(&self, event: &NSEvent);
        #[method(hitPart)]
        pub unsafe fn hitPart(&self) -> NSScrollerPart;
        #[method(knobProportion)]
        pub unsafe fn knobProportion(&self) -> CGFloat;
        #[method(setKnobProportion:)]
        pub unsafe fn setKnobProportion(&self, knobProportion: CGFloat);
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSScroller {
        #[method(scrollerWidthForControlSize:)]
        pub unsafe fn scrollerWidthForControlSize(controlSize: NSControlSize) -> CGFloat;
        #[method(scrollerWidth)]
        pub unsafe fn scrollerWidth() -> CGFloat;
        #[method(setFloatValue:knobProportion:)]
        pub unsafe fn setFloatValue_knobProportion(&self, value: c_float, proportion: CGFloat);
        #[method(arrowsPosition)]
        pub unsafe fn arrowsPosition(&self) -> NSScrollArrowPosition;
        #[method(setArrowsPosition:)]
        pub unsafe fn setArrowsPosition(&self, arrowsPosition: NSScrollArrowPosition);
        #[method(controlTint)]
        pub unsafe fn controlTint(&self) -> NSControlTint;
        #[method(setControlTint:)]
        pub unsafe fn setControlTint(&self, controlTint: NSControlTint);
        #[method(highlight:)]
        pub unsafe fn highlight(&self, flag: bool);
        #[method(trackScrollButtons:)]
        pub unsafe fn trackScrollButtons(&self, event: &NSEvent);
        #[method(drawParts)]
        pub unsafe fn drawParts(&self);
        #[method(drawArrow:highlight:)]
        pub unsafe fn drawArrow_highlight(&self, whichArrow: NSScrollerArrow, flag: bool);
    }
);
