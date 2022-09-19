use crate::Foundation::generated::NSCalendar::*;
use crate::Foundation::generated::NSFormatter::*;
use crate::Foundation::generated::NSNumberFormatter::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDateComponentsFormatter;
    unsafe impl ClassType for NSDateComponentsFormatter {
        type Super = NSFormatter;
    }
);
impl NSDateComponentsFormatter {
    pub unsafe fn stringForObjectValue(
        &self,
        obj: Option<&Object>,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringForObjectValue: obj]
    }
    pub unsafe fn stringFromDateComponents(
        &self,
        components: &NSDateComponents,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringFromDateComponents: components]
    }
    pub unsafe fn stringFromDate_toDate(
        &self,
        startDate: &NSDate,
        endDate: &NSDate,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringFromDate: startDate, toDate: endDate]
    }
    pub unsafe fn stringFromTimeInterval(
        &self,
        ti: NSTimeInterval,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringFromTimeInterval: ti]
    }
    pub unsafe fn localizedStringFromDateComponents_unitsStyle(
        components: &NSDateComponents,
        unitsStyle: NSDateComponentsFormatterUnitsStyle,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![
            Self::class(),
            localizedStringFromDateComponents: components,
            unitsStyle: unitsStyle
        ]
    }
    pub unsafe fn getObjectValue_forString_errorDescription(
        &self,
        obj: *mut *mut Object,
        string: &NSString,
        error: *mut *mut NSString,
    ) -> bool {
        msg_send![
            self,
            getObjectValue: obj,
            forString: string,
            errorDescription: error
        ]
    }
    pub unsafe fn unitsStyle(&self) -> NSDateComponentsFormatterUnitsStyle {
        msg_send![self, unitsStyle]
    }
    pub unsafe fn setUnitsStyle(&self, unitsStyle: NSDateComponentsFormatterUnitsStyle) {
        msg_send![self, setUnitsStyle: unitsStyle]
    }
    pub unsafe fn allowedUnits(&self) -> NSCalendarUnit {
        msg_send![self, allowedUnits]
    }
    pub unsafe fn setAllowedUnits(&self, allowedUnits: NSCalendarUnit) {
        msg_send![self, setAllowedUnits: allowedUnits]
    }
    pub unsafe fn zeroFormattingBehavior(&self) -> NSDateComponentsFormatterZeroFormattingBehavior {
        msg_send![self, zeroFormattingBehavior]
    }
    pub unsafe fn setZeroFormattingBehavior(
        &self,
        zeroFormattingBehavior: NSDateComponentsFormatterZeroFormattingBehavior,
    ) {
        msg_send![self, setZeroFormattingBehavior: zeroFormattingBehavior]
    }
    pub unsafe fn calendar(&self) -> Option<Id<NSCalendar, Shared>> {
        msg_send_id![self, calendar]
    }
    pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>) {
        msg_send![self, setCalendar: calendar]
    }
    pub unsafe fn referenceDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, referenceDate]
    }
    pub unsafe fn setReferenceDate(&self, referenceDate: Option<&NSDate>) {
        msg_send![self, setReferenceDate: referenceDate]
    }
    pub unsafe fn allowsFractionalUnits(&self) -> bool {
        msg_send![self, allowsFractionalUnits]
    }
    pub unsafe fn setAllowsFractionalUnits(&self, allowsFractionalUnits: bool) {
        msg_send![self, setAllowsFractionalUnits: allowsFractionalUnits]
    }
    pub unsafe fn maximumUnitCount(&self) -> NSInteger {
        msg_send![self, maximumUnitCount]
    }
    pub unsafe fn setMaximumUnitCount(&self, maximumUnitCount: NSInteger) {
        msg_send![self, setMaximumUnitCount: maximumUnitCount]
    }
    pub unsafe fn collapsesLargestUnit(&self) -> bool {
        msg_send![self, collapsesLargestUnit]
    }
    pub unsafe fn setCollapsesLargestUnit(&self, collapsesLargestUnit: bool) {
        msg_send![self, setCollapsesLargestUnit: collapsesLargestUnit]
    }
    pub unsafe fn includesApproximationPhrase(&self) -> bool {
        msg_send![self, includesApproximationPhrase]
    }
    pub unsafe fn setIncludesApproximationPhrase(&self, includesApproximationPhrase: bool) {
        msg_send![
            self,
            setIncludesApproximationPhrase: includesApproximationPhrase
        ]
    }
    pub unsafe fn includesTimeRemainingPhrase(&self) -> bool {
        msg_send![self, includesTimeRemainingPhrase]
    }
    pub unsafe fn setIncludesTimeRemainingPhrase(&self, includesTimeRemainingPhrase: bool) {
        msg_send![
            self,
            setIncludesTimeRemainingPhrase: includesTimeRemainingPhrase
        ]
    }
    pub unsafe fn formattingContext(&self) -> NSFormattingContext {
        msg_send![self, formattingContext]
    }
    pub unsafe fn setFormattingContext(&self, formattingContext: NSFormattingContext) {
        msg_send![self, setFormattingContext: formattingContext]
    }
}
