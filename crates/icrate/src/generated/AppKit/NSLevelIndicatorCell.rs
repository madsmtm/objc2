//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSLevelIndicatorStyle = NSUInteger;
pub const NSLevelIndicatorStyleRelevancy: NSLevelIndicatorStyle = 0;
pub const NSLevelIndicatorStyleContinuousCapacity: NSLevelIndicatorStyle = 1;
pub const NSLevelIndicatorStyleDiscreteCapacity: NSLevelIndicatorStyle = 2;
pub const NSLevelIndicatorStyleRating: NSLevelIndicatorStyle = 3;

extern_class!(
    #[derive(Debug)]
    pub struct NSLevelIndicatorCell;

    unsafe impl ClassType for NSLevelIndicatorCell {
        type Super = NSActionCell;
    }
);

extern_methods!(
    unsafe impl NSLevelIndicatorCell {
        #[method_id(@__retain_semantics Init initWithLevelIndicatorStyle:)]
        pub unsafe fn initWithLevelIndicatorStyle(
            this: Option<Allocated<Self>>,
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

pub static NSRelevancyLevelIndicatorStyle: NSLevelIndicatorStyle = NSLevelIndicatorStyleRelevancy;

pub static NSContinuousCapacityLevelIndicatorStyle: NSLevelIndicatorStyle =
    NSLevelIndicatorStyleContinuousCapacity;

pub static NSDiscreteCapacityLevelIndicatorStyle: NSLevelIndicatorStyle =
    NSLevelIndicatorStyleDiscreteCapacity;

pub static NSRatingLevelIndicatorStyle: NSLevelIndicatorStyle = NSLevelIndicatorStyleRating;
