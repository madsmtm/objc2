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
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDateIntervalFormatter;
    unsafe impl ClassType for NSDateIntervalFormatter {
        type Super = NSFormatter;
    }
);
extern_methods!(
    unsafe impl NSDateIntervalFormatter {
        #[method_id(locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale, Shared>;
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);
        #[method_id(calendar)]
        pub unsafe fn calendar(&self) -> Id<NSCalendar, Shared>;
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);
        #[method_id(timeZone)]
        pub unsafe fn timeZone(&self) -> Id<NSTimeZone, Shared>;
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, timeZone: Option<&NSTimeZone>);
        #[method_id(dateTemplate)]
        pub unsafe fn dateTemplate(&self) -> Id<NSString, Shared>;
        #[method(setDateTemplate:)]
        pub unsafe fn setDateTemplate(&self, dateTemplate: Option<&NSString>);
        #[method(dateStyle)]
        pub unsafe fn dateStyle(&self) -> NSDateIntervalFormatterStyle;
        #[method(setDateStyle:)]
        pub unsafe fn setDateStyle(&self, dateStyle: NSDateIntervalFormatterStyle);
        #[method(timeStyle)]
        pub unsafe fn timeStyle(&self) -> NSDateIntervalFormatterStyle;
        #[method(setTimeStyle:)]
        pub unsafe fn setTimeStyle(&self, timeStyle: NSDateIntervalFormatterStyle);
        #[method_id(stringFromDate:toDate:)]
        pub unsafe fn stringFromDate_toDate(
            &self,
            fromDate: &NSDate,
            toDate: &NSDate,
        ) -> Id<NSString, Shared>;
        #[method_id(stringFromDateInterval:)]
        pub unsafe fn stringFromDateInterval(
            &self,
            dateInterval: &NSDateInterval,
        ) -> Option<Id<NSString, Shared>>;
    }
);
