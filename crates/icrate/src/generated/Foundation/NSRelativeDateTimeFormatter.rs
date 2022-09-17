use super::NSCalendar;
use super::NSDateComponents;
use super::NSLocale;
use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSFormatter::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSRelativeDateTimeFormatter;
    unsafe impl ClassType for NSRelativeDateTimeFormatter {
        type Super = NSFormatter;
    }
);
impl NSRelativeDateTimeFormatter {
    pub unsafe fn localizedStringFromDateComponents(
        &self,
        dateComponents: &NSDateComponents,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, localizedStringFromDateComponents: dateComponents]
    }
    pub unsafe fn localizedStringFromTimeInterval(
        &self,
        timeInterval: NSTimeInterval,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, localizedStringFromTimeInterval: timeInterval]
    }
    pub unsafe fn localizedStringForDate_relativeToDate(
        &self,
        date: &NSDate,
        referenceDate: &NSDate,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            self,
            localizedStringForDate: date,
            relativeToDate: referenceDate
        ]
    }
    pub unsafe fn stringForObjectValue(
        &self,
        obj: Option<&Object>,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringForObjectValue: obj]
    }
    pub unsafe fn dateTimeStyle(&self) -> NSRelativeDateTimeFormatterStyle {
        msg_send![self, dateTimeStyle]
    }
    pub unsafe fn setDateTimeStyle(&self, dateTimeStyle: NSRelativeDateTimeFormatterStyle) {
        msg_send![self, setDateTimeStyle: dateTimeStyle]
    }
    pub unsafe fn unitsStyle(&self) -> NSRelativeDateTimeFormatterUnitsStyle {
        msg_send![self, unitsStyle]
    }
    pub unsafe fn setUnitsStyle(&self, unitsStyle: NSRelativeDateTimeFormatterUnitsStyle) {
        msg_send![self, setUnitsStyle: unitsStyle]
    }
    pub unsafe fn formattingContext(&self) -> NSFormattingContext {
        msg_send![self, formattingContext]
    }
    pub unsafe fn setFormattingContext(&self, formattingContext: NSFormattingContext) {
        msg_send![self, setFormattingContext: formattingContext]
    }
    pub unsafe fn calendar(&self) -> Id<NSCalendar, Shared> {
        msg_send_id![self, calendar]
    }
    pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>) {
        msg_send![self, setCalendar: calendar]
    }
    pub unsafe fn locale(&self) -> Id<NSLocale, Shared> {
        msg_send_id![self, locale]
    }
    pub unsafe fn setLocale(&self, locale: Option<&NSLocale>) {
        msg_send![self, setLocale: locale]
    }
}
