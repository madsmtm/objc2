#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDateComponentsFormatter;
    unsafe impl ClassType for NSDateComponentsFormatter {
        type Super = NSFormatter;
    }
);
extern_methods!(
    unsafe impl NSDateComponentsFormatter {
        #[method_id(stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(
            &self,
            obj: Option<&Object>,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(stringFromDateComponents:)]
        pub unsafe fn stringFromDateComponents(
            &self,
            components: &NSDateComponents,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(stringFromDate:toDate:)]
        pub unsafe fn stringFromDate_toDate(
            &self,
            startDate: &NSDate,
            endDate: &NSDate,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(stringFromTimeInterval:)]
        pub unsafe fn stringFromTimeInterval(
            &self,
            ti: NSTimeInterval,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(localizedStringFromDateComponents:unitsStyle:)]
        pub unsafe fn localizedStringFromDateComponents_unitsStyle(
            components: &NSDateComponents,
            unitsStyle: NSDateComponentsFormatterUnitsStyle,
        ) -> Option<Id<NSString, Shared>>;
        #[method(unitsStyle)]
        pub unsafe fn unitsStyle(&self) -> NSDateComponentsFormatterUnitsStyle;
        #[method(setUnitsStyle:)]
        pub unsafe fn setUnitsStyle(&self, unitsStyle: NSDateComponentsFormatterUnitsStyle);
        #[method(allowedUnits)]
        pub unsafe fn allowedUnits(&self) -> NSCalendarUnit;
        #[method(setAllowedUnits:)]
        pub unsafe fn setAllowedUnits(&self, allowedUnits: NSCalendarUnit);
        #[method(zeroFormattingBehavior)]
        pub unsafe fn zeroFormattingBehavior(
            &self,
        ) -> NSDateComponentsFormatterZeroFormattingBehavior;
        #[method(setZeroFormattingBehavior:)]
        pub unsafe fn setZeroFormattingBehavior(
            &self,
            zeroFormattingBehavior: NSDateComponentsFormatterZeroFormattingBehavior,
        );
        #[method_id(calendar)]
        pub unsafe fn calendar(&self) -> Option<Id<NSCalendar, Shared>>;
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);
        #[method_id(referenceDate)]
        pub unsafe fn referenceDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method(setReferenceDate:)]
        pub unsafe fn setReferenceDate(&self, referenceDate: Option<&NSDate>);
        #[method(allowsFractionalUnits)]
        pub unsafe fn allowsFractionalUnits(&self) -> bool;
        #[method(setAllowsFractionalUnits:)]
        pub unsafe fn setAllowsFractionalUnits(&self, allowsFractionalUnits: bool);
        #[method(maximumUnitCount)]
        pub unsafe fn maximumUnitCount(&self) -> NSInteger;
        #[method(setMaximumUnitCount:)]
        pub unsafe fn setMaximumUnitCount(&self, maximumUnitCount: NSInteger);
        #[method(collapsesLargestUnit)]
        pub unsafe fn collapsesLargestUnit(&self) -> bool;
        #[method(setCollapsesLargestUnit:)]
        pub unsafe fn setCollapsesLargestUnit(&self, collapsesLargestUnit: bool);
        #[method(includesApproximationPhrase)]
        pub unsafe fn includesApproximationPhrase(&self) -> bool;
        #[method(setIncludesApproximationPhrase:)]
        pub unsafe fn setIncludesApproximationPhrase(&self, includesApproximationPhrase: bool);
        #[method(includesTimeRemainingPhrase)]
        pub unsafe fn includesTimeRemainingPhrase(&self) -> bool;
        #[method(setIncludesTimeRemainingPhrase:)]
        pub unsafe fn setIncludesTimeRemainingPhrase(&self, includesTimeRemainingPhrase: bool);
        #[method(formattingContext)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;
        #[method(setFormattingContext:)]
        pub unsafe fn setFormattingContext(&self, formattingContext: NSFormattingContext);
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Id<Object, Shared>>>,
            string: &NSString,
            error: Option<&mut Option<Id<NSString, Shared>>>,
        ) -> bool;
    }
);
