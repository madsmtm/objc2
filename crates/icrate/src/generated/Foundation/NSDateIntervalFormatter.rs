use super::__exported::NSCalendar;
use super::__exported::NSDate;
use super::__exported::NSLocale;
use super::__exported::NSTimeZone;
use crate::dispatch::generated::dispatch::*;
use crate::Foundation::generated::NSDateInterval::*;
use crate::Foundation::generated::NSFormatter::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDateIntervalFormatter;
    unsafe impl ClassType for NSDateIntervalFormatter {
        type Super = NSFormatter;
    }
);
impl NSDateIntervalFormatter {
    pub unsafe fn locale(&self) -> Id<NSLocale, Shared> {
        msg_send_id![self, locale]
    }
    pub unsafe fn setLocale(&self, locale: Option<&NSLocale>) {
        msg_send![self, setLocale: locale]
    }
    pub unsafe fn calendar(&self) -> Id<NSCalendar, Shared> {
        msg_send_id![self, calendar]
    }
    pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>) {
        msg_send![self, setCalendar: calendar]
    }
    pub unsafe fn timeZone(&self) -> Id<NSTimeZone, Shared> {
        msg_send_id![self, timeZone]
    }
    pub unsafe fn setTimeZone(&self, timeZone: Option<&NSTimeZone>) {
        msg_send![self, setTimeZone: timeZone]
    }
    pub unsafe fn dateTemplate(&self) -> Id<NSString, Shared> {
        msg_send_id![self, dateTemplate]
    }
    pub unsafe fn setDateTemplate(&self, dateTemplate: Option<&NSString>) {
        msg_send![self, setDateTemplate: dateTemplate]
    }
    pub unsafe fn dateStyle(&self) -> NSDateIntervalFormatterStyle {
        msg_send![self, dateStyle]
    }
    pub unsafe fn setDateStyle(&self, dateStyle: NSDateIntervalFormatterStyle) {
        msg_send![self, setDateStyle: dateStyle]
    }
    pub unsafe fn timeStyle(&self) -> NSDateIntervalFormatterStyle {
        msg_send![self, timeStyle]
    }
    pub unsafe fn setTimeStyle(&self, timeStyle: NSDateIntervalFormatterStyle) {
        msg_send![self, setTimeStyle: timeStyle]
    }
    pub unsafe fn stringFromDate_toDate(
        &self,
        fromDate: &NSDate,
        toDate: &NSDate,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, stringFromDate: fromDate, toDate: toDate]
    }
    pub unsafe fn stringFromDateInterval(
        &self,
        dateInterval: &NSDateInterval,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringFromDateInterval: dateInterval]
    }
}
