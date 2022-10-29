#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSLevelIndicatorCell;
    unsafe impl ClassType for NSLevelIndicatorCell {
        type Super = NSActionCell;
    }
);
extern_methods!(
    unsafe impl NSLevelIndicatorCell {
        #[method_id(initWithLevelIndicatorStyle:)]
        pub unsafe fn initWithLevelIndicatorStyle(
            &self,
            levelIndicatorStyle: NSLevelIndicatorStyle,
        ) -> Id<Self, Shared>;
        #[method(levelIndicatorStyle)]
        pub unsafe fn levelIndicatorStyle(&self) -> NSLevelIndicatorStyle;
        #[method(setLevelIndicatorStyle:)]
        pub unsafe fn setLevelIndicatorStyle(&self, levelIndicatorStyle: NSLevelIndicatorStyle);
        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;
        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, minValue: c_double);
        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;
        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, maxValue: c_double);
        #[method(warningValue)]
        pub unsafe fn warningValue(&self) -> c_double;
        #[method(setWarningValue:)]
        pub unsafe fn setWarningValue(&self, warningValue: c_double);
        #[method(criticalValue)]
        pub unsafe fn criticalValue(&self) -> c_double;
        #[method(setCriticalValue:)]
        pub unsafe fn setCriticalValue(&self, criticalValue: c_double);
        #[method(tickMarkPosition)]
        pub unsafe fn tickMarkPosition(&self) -> NSTickMarkPosition;
        #[method(setTickMarkPosition:)]
        pub unsafe fn setTickMarkPosition(&self, tickMarkPosition: NSTickMarkPosition);
        #[method(numberOfTickMarks)]
        pub unsafe fn numberOfTickMarks(&self) -> NSInteger;
        #[method(setNumberOfTickMarks:)]
        pub unsafe fn setNumberOfTickMarks(&self, numberOfTickMarks: NSInteger);
        #[method(numberOfMajorTickMarks)]
        pub unsafe fn numberOfMajorTickMarks(&self) -> NSInteger;
        #[method(setNumberOfMajorTickMarks:)]
        pub unsafe fn setNumberOfMajorTickMarks(&self, numberOfMajorTickMarks: NSInteger);
        #[method(rectOfTickMarkAtIndex:)]
        pub unsafe fn rectOfTickMarkAtIndex(&self, index: NSInteger) -> NSRect;
        #[method(tickMarkValueAtIndex:)]
        pub unsafe fn tickMarkValueAtIndex(&self, index: NSInteger) -> c_double;
    }
);
