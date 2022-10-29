#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSliderCell;
    unsafe impl ClassType for NSSliderCell {
        type Super = NSActionCell;
    }
);
extern_methods!(
    unsafe impl NSSliderCell {
        #[method(prefersTrackingUntilMouseUp)]
        pub unsafe fn prefersTrackingUntilMouseUp() -> bool;
        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;
        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, minValue: c_double);
        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;
        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, maxValue: c_double);
        #[method(altIncrementValue)]
        pub unsafe fn altIncrementValue(&self) -> c_double;
        #[method(setAltIncrementValue:)]
        pub unsafe fn setAltIncrementValue(&self, altIncrementValue: c_double);
        #[method(sliderType)]
        pub unsafe fn sliderType(&self) -> NSSliderType;
        #[method(setSliderType:)]
        pub unsafe fn setSliderType(&self, sliderType: NSSliderType);
        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;
        #[method(setVertical:)]
        pub unsafe fn setVertical(&self, vertical: bool);
        #[method(trackRect)]
        pub unsafe fn trackRect(&self) -> NSRect;
        #[method(knobThickness)]
        pub unsafe fn knobThickness(&self) -> CGFloat;
        #[method(knobRectFlipped:)]
        pub unsafe fn knobRectFlipped(&self, flipped: bool) -> NSRect;
        #[method(barRectFlipped:)]
        pub unsafe fn barRectFlipped(&self, flipped: bool) -> NSRect;
        #[method(drawKnob:)]
        pub unsafe fn drawKnob(&self, knobRect: NSRect);
        #[method(drawKnob)]
        pub unsafe fn drawKnob(&self);
        #[method(drawBarInside:flipped:)]
        pub unsafe fn drawBarInside_flipped(&self, rect: NSRect, flipped: bool);
    }
);
extern_methods!(
    #[doc = "NSSliderCellVerticalGetter"]
    unsafe impl NSSliderCell {
        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;
    }
);
extern_methods!(
    #[doc = "NSTickMarkSupport"]
    unsafe impl NSSliderCell {
        #[method(numberOfTickMarks)]
        pub unsafe fn numberOfTickMarks(&self) -> NSInteger;
        #[method(setNumberOfTickMarks:)]
        pub unsafe fn setNumberOfTickMarks(&self, numberOfTickMarks: NSInteger);
        #[method(tickMarkPosition)]
        pub unsafe fn tickMarkPosition(&self) -> NSTickMarkPosition;
        #[method(setTickMarkPosition:)]
        pub unsafe fn setTickMarkPosition(&self, tickMarkPosition: NSTickMarkPosition);
        #[method(allowsTickMarkValuesOnly)]
        pub unsafe fn allowsTickMarkValuesOnly(&self) -> bool;
        #[method(setAllowsTickMarkValuesOnly:)]
        pub unsafe fn setAllowsTickMarkValuesOnly(&self, allowsTickMarkValuesOnly: bool);
        #[method(tickMarkValueAtIndex:)]
        pub unsafe fn tickMarkValueAtIndex(&self, index: NSInteger) -> c_double;
        #[method(rectOfTickMarkAtIndex:)]
        pub unsafe fn rectOfTickMarkAtIndex(&self, index: NSInteger) -> NSRect;
        #[method(indexOfTickMarkAtPoint:)]
        pub unsafe fn indexOfTickMarkAtPoint(&self, point: NSPoint) -> NSInteger;
        #[method(closestTickMarkValueToValue:)]
        pub unsafe fn closestTickMarkValueToValue(&self, value: c_double) -> c_double;
        #[method(drawTickMarks)]
        pub unsafe fn drawTickMarks(&self);
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSSliderCell {
        #[method(setTitleCell:)]
        pub unsafe fn setTitleCell(&self, cell: Option<&NSCell>);
        #[method_id(titleCell)]
        pub unsafe fn titleCell(&self) -> Option<Id<Object, Shared>>;
        #[method(setTitleColor:)]
        pub unsafe fn setTitleColor(&self, newColor: Option<&NSColor>);
        #[method_id(titleColor)]
        pub unsafe fn titleColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method(setTitleFont:)]
        pub unsafe fn setTitleFont(&self, fontObj: Option<&NSFont>);
        #[method_id(titleFont)]
        pub unsafe fn titleFont(&self) -> Option<Id<NSFont, Shared>>;
        #[method_id(title)]
        pub unsafe fn title(&self) -> Option<Id<NSString, Shared>>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, string: Option<&NSString>);
        #[method(setKnobThickness:)]
        pub unsafe fn setKnobThickness(&self, thickness: CGFloat);
        #[method(setImage:)]
        pub unsafe fn setImage(&self, backgroundImage: Option<&NSImage>);
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
    }
);