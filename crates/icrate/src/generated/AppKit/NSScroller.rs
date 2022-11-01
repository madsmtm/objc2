//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSUsableScrollerParts = NSUInteger;
pub const NSNoScrollerParts: NSUsableScrollerParts = 0;
pub const NSOnlyScrollerArrows: NSUsableScrollerParts = 1;
pub const NSAllScrollerParts: NSUsableScrollerParts = 2;

pub type NSScrollerPart = NSUInteger;
pub const NSScrollerNoPart: NSScrollerPart = 0;
pub const NSScrollerDecrementPage: NSScrollerPart = 1;
pub const NSScrollerKnob: NSScrollerPart = 2;
pub const NSScrollerIncrementPage: NSScrollerPart = 3;
pub const NSScrollerDecrementLine: NSScrollerPart = 4;
pub const NSScrollerIncrementLine: NSScrollerPart = 5;
pub const NSScrollerKnobSlot: NSScrollerPart = 6;

pub type NSScrollerStyle = NSInteger;
pub const NSScrollerStyleLegacy: NSScrollerStyle = 0;
pub const NSScrollerStyleOverlay: NSScrollerStyle = 1;

pub type NSScrollerKnobStyle = NSInteger;
pub const NSScrollerKnobStyleDefault: NSScrollerKnobStyle = 0;
pub const NSScrollerKnobStyleDark: NSScrollerKnobStyle = 1;
pub const NSScrollerKnobStyleLight: NSScrollerKnobStyle = 2;

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

extern "C" {
    pub static NSPreferredScrollerStyleDidChangeNotification: &'static NSNotificationName;
}

pub type NSScrollArrowPosition = NSUInteger;
pub const NSScrollerArrowsMaxEnd: NSScrollArrowPosition = 0;
pub const NSScrollerArrowsMinEnd: NSScrollArrowPosition = 1;
pub const NSScrollerArrowsDefaultSetting: NSScrollArrowPosition = 0;
pub const NSScrollerArrowsNone: NSScrollArrowPosition = 2;

pub type NSScrollerArrow = NSUInteger;
pub const NSScrollerIncrementArrow: NSScrollerArrow = 0;
pub const NSScrollerDecrementArrow: NSScrollerArrow = 1;

extern_methods!(
    /// NSDeprecated
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
