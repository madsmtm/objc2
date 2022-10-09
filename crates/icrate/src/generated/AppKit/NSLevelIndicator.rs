use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSControl::*;
use crate::AppKit::generated::NSLevelIndicatorCell::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSLevelIndicator;
    unsafe impl ClassType for NSLevelIndicator {
        type Super = NSControl;
    }
);
extern_methods!(
    unsafe impl NSLevelIndicator {
        #[method(levelIndicatorStyle)]
        pub unsafe fn levelIndicatorStyle(&self) -> NSLevelIndicatorStyle;
        #[method(setLevelIndicatorStyle:)]
        pub unsafe fn setLevelIndicatorStyle(&self, levelIndicatorStyle: NSLevelIndicatorStyle);
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;
        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);
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
        #[method(tickMarkValueAtIndex:)]
        pub unsafe fn tickMarkValueAtIndex(&self, index: NSInteger) -> c_double;
        #[method(rectOfTickMarkAtIndex:)]
        pub unsafe fn rectOfTickMarkAtIndex(&self, index: NSInteger) -> NSRect;
        #[method_id(fillColor)]
        pub unsafe fn fillColor(&self) -> Id<NSColor, Shared>;
        #[method(setFillColor:)]
        pub unsafe fn setFillColor(&self, fillColor: Option<&NSColor>);
        #[method_id(warningFillColor)]
        pub unsafe fn warningFillColor(&self) -> Id<NSColor, Shared>;
        #[method(setWarningFillColor:)]
        pub unsafe fn setWarningFillColor(&self, warningFillColor: Option<&NSColor>);
        #[method_id(criticalFillColor)]
        pub unsafe fn criticalFillColor(&self) -> Id<NSColor, Shared>;
        #[method(setCriticalFillColor:)]
        pub unsafe fn setCriticalFillColor(&self, criticalFillColor: Option<&NSColor>);
        #[method(drawsTieredCapacityLevels)]
        pub unsafe fn drawsTieredCapacityLevels(&self) -> bool;
        #[method(setDrawsTieredCapacityLevels:)]
        pub unsafe fn setDrawsTieredCapacityLevels(&self, drawsTieredCapacityLevels: bool);
        #[method(placeholderVisibility)]
        pub unsafe fn placeholderVisibility(&self) -> NSLevelIndicatorPlaceholderVisibility;
        #[method(setPlaceholderVisibility:)]
        pub unsafe fn setPlaceholderVisibility(
            &self,
            placeholderVisibility: NSLevelIndicatorPlaceholderVisibility,
        );
        #[method_id(ratingImage)]
        pub unsafe fn ratingImage(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setRatingImage:)]
        pub unsafe fn setRatingImage(&self, ratingImage: Option<&NSImage>);
        #[method_id(ratingPlaceholderImage)]
        pub unsafe fn ratingPlaceholderImage(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setRatingPlaceholderImage:)]
        pub unsafe fn setRatingPlaceholderImage(&self, ratingPlaceholderImage: Option<&NSImage>);
    }
);
