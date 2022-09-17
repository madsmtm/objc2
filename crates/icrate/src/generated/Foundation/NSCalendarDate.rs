#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCalendarDate;
    unsafe impl ClassType for NSCalendarDate {
        type Super = NSDate;
    }
);
impl NSCalendarDate {
    pub unsafe fn calendarDate() -> Id<Object, Shared> {
        msg_send_id![Self::class(), calendarDate]
    }
    pub unsafe fn dateWithString_calendarFormat_locale(
        description: &NSString,
        format: &NSString,
        locale: Option<&Object>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            Self::class(),
            dateWithString: description,
            calendarFormat: format,
            locale: locale
        ]
    }
    pub unsafe fn dateWithString_calendarFormat(
        description: &NSString,
        format: &NSString,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            Self::class(),
            dateWithString: description,
            calendarFormat: format
        ]
    }
    pub unsafe fn dateWithYear_month_day_hour_minute_second_timeZone(
        year: NSInteger,
        month: NSUInteger,
        day: NSUInteger,
        hour: NSUInteger,
        minute: NSUInteger,
        second: NSUInteger,
        aTimeZone: Option<&NSTimeZone>,
    ) -> Id<Object, Shared> {
        msg_send_id![
            Self::class(),
            dateWithYear: year,
            month: month,
            day: day,
            hour: hour,
            minute: minute,
            second: second,
            timeZone: aTimeZone
        ]
    }
    pub unsafe fn dateByAddingYears_months_days_hours_minutes_seconds(
        &self,
        year: NSInteger,
        month: NSInteger,
        day: NSInteger,
        hour: NSInteger,
        minute: NSInteger,
        second: NSInteger,
    ) -> Id<NSCalendarDate, Shared> {
        msg_send_id![
            self,
            dateByAddingYears: year,
            months: month,
            days: day,
            hours: hour,
            minutes: minute,
            seconds: second
        ]
    }
    pub unsafe fn dayOfCommonEra(&self) -> NSInteger {
        msg_send![self, dayOfCommonEra]
    }
    pub unsafe fn dayOfMonth(&self) -> NSInteger {
        msg_send![self, dayOfMonth]
    }
    pub unsafe fn dayOfWeek(&self) -> NSInteger {
        msg_send![self, dayOfWeek]
    }
    pub unsafe fn dayOfYear(&self) -> NSInteger {
        msg_send![self, dayOfYear]
    }
    pub unsafe fn hourOfDay(&self) -> NSInteger {
        msg_send![self, hourOfDay]
    }
    pub unsafe fn minuteOfHour(&self) -> NSInteger {
        msg_send![self, minuteOfHour]
    }
    pub unsafe fn monthOfYear(&self) -> NSInteger {
        msg_send![self, monthOfYear]
    }
    pub unsafe fn secondOfMinute(&self) -> NSInteger {
        msg_send![self, secondOfMinute]
    }
    pub unsafe fn yearOfCommonEra(&self) -> NSInteger {
        msg_send![self, yearOfCommonEra]
    }
    pub unsafe fn calendarFormat(&self) -> Id<NSString, Shared> {
        msg_send_id![self, calendarFormat]
    }
    pub unsafe fn descriptionWithCalendarFormat_locale(
        &self,
        format: &NSString,
        locale: Option<&Object>,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, descriptionWithCalendarFormat: format, locale: locale]
    }
    pub unsafe fn descriptionWithCalendarFormat(&self, format: &NSString) -> Id<NSString, Shared> {
        msg_send_id![self, descriptionWithCalendarFormat: format]
    }
    pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>) -> Id<NSString, Shared> {
        msg_send_id![self, descriptionWithLocale: locale]
    }
    pub unsafe fn timeZone(&self) -> Id<NSTimeZone, Shared> {
        msg_send_id![self, timeZone]
    }
    pub unsafe fn initWithString_calendarFormat_locale(
        &self,
        description: &NSString,
        format: &NSString,
        locale: Option<&Object>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            self,
            initWithString: description,
            calendarFormat: format,
            locale: locale
        ]
    }
    pub unsafe fn initWithString_calendarFormat(
        &self,
        description: &NSString,
        format: &NSString,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, initWithString: description, calendarFormat: format]
    }
    pub unsafe fn initWithString(&self, description: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, initWithString: description]
    }
    pub unsafe fn initWithYear_month_day_hour_minute_second_timeZone(
        &self,
        year: NSInteger,
        month: NSUInteger,
        day: NSUInteger,
        hour: NSUInteger,
        minute: NSUInteger,
        second: NSUInteger,
        aTimeZone: Option<&NSTimeZone>,
    ) -> Id<Object, Shared> {
        msg_send_id![
            self,
            initWithYear: year,
            month: month,
            day: day,
            hour: hour,
            minute: minute,
            second: second,
            timeZone: aTimeZone
        ]
    }
    pub unsafe fn setCalendarFormat(&self, format: Option<&NSString>) {
        msg_send![self, setCalendarFormat: format]
    }
    pub unsafe fn setTimeZone(&self, aTimeZone: Option<&NSTimeZone>) {
        msg_send![self, setTimeZone: aTimeZone]
    }
    pub unsafe fn years_months_days_hours_minutes_seconds_sinceDate(
        &self,
        yp: *mut NSInteger,
        mop: *mut NSInteger,
        dp: *mut NSInteger,
        hp: *mut NSInteger,
        mip: *mut NSInteger,
        sp: *mut NSInteger,
        date: &NSCalendarDate,
    ) {
        msg_send![
            self,
            years: yp,
            months: mop,
            days: dp,
            hours: hp,
            minutes: mip,
            seconds: sp,
            sinceDate: date
        ]
    }
    pub unsafe fn distantFuture() -> Id<Self, Shared> {
        msg_send_id![Self::class(), distantFuture]
    }
    pub unsafe fn distantPast() -> Id<Self, Shared> {
        msg_send_id![Self::class(), distantPast]
    }
}
#[doc = "NSCalendarDateExtras"]
impl NSDate {
    pub unsafe fn dateWithNaturalLanguageString_locale(
        string: &NSString,
        locale: Option<&Object>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            Self::class(),
            dateWithNaturalLanguageString: string,
            locale: locale
        ]
    }
    pub unsafe fn dateWithNaturalLanguageString(string: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![Self::class(), dateWithNaturalLanguageString: string]
    }
    pub unsafe fn dateWithString(aString: &NSString) -> Id<Object, Shared> {
        msg_send_id![Self::class(), dateWithString: aString]
    }
    pub unsafe fn dateWithCalendarFormat_timeZone(
        &self,
        format: Option<&NSString>,
        aTimeZone: Option<&NSTimeZone>,
    ) -> Id<NSCalendarDate, Shared> {
        msg_send_id![self, dateWithCalendarFormat: format, timeZone: aTimeZone]
    }
    pub unsafe fn descriptionWithCalendarFormat_timeZone_locale(
        &self,
        format: Option<&NSString>,
        aTimeZone: Option<&NSTimeZone>,
        locale: Option<&Object>,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![
            self,
            descriptionWithCalendarFormat: format,
            timeZone: aTimeZone,
            locale: locale
        ]
    }
    pub unsafe fn initWithString(&self, description: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, initWithString: description]
    }
}
