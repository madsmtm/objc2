#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSlider;
    unsafe impl ClassType for NSSlider {
        type Super = NSControl;
    }
);
extern_methods!(
    unsafe impl NSSlider {
        #[method(sliderType)]
        pub unsafe fn sliderType(&self) -> NSSliderType;
        #[method(setSliderType:)]
        pub unsafe fn setSliderType(&self, sliderType: NSSliderType);
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
        #[method(knobThickness)]
        pub unsafe fn knobThickness(&self) -> CGFloat;
        #[method(acceptsFirstMouse:)]
        pub unsafe fn acceptsFirstMouse(&self, event: Option<&NSEvent>) -> bool;
        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;
        #[method(setVertical:)]
        pub unsafe fn setVertical(&self, vertical: bool);
        #[method_id(trackFillColor)]
        pub unsafe fn trackFillColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method(setTrackFillColor:)]
        pub unsafe fn setTrackFillColor(&self, trackFillColor: Option<&NSColor>);
    }
);
extern_methods!(
    #[doc = "NSSliderVerticalGetter"]
    unsafe impl NSSlider {
        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;
    }
);
extern_methods!(
    #[doc = "NSTickMarkSupport"]
    unsafe impl NSSlider {
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
    }
);
extern_methods!(
    #[doc = "NSSliderConvenience"]
    unsafe impl NSSlider {
        #[method_id(sliderWithTarget:action:)]
        pub unsafe fn sliderWithTarget_action(
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
        #[method_id(sliderWithValue:minValue:maxValue:target:action:)]
        pub unsafe fn sliderWithValue_minValue_maxValue_target_action(
            value: c_double,
            minValue: c_double,
            maxValue: c_double,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
    }
);
extern_methods!(
    #[doc = "NSSliderDeprecated"]
    unsafe impl NSSlider {
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