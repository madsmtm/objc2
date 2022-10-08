use super::__exported::NSArray;
use super::__exported::NSString;
use super::__exported::NSTimeZone;
use crate::Foundation::generated::NSDate::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSCalendarDate;
    unsafe impl ClassType for NSCalendarDate {
        type Super = NSDate;
    }
);
extern_methods!(
    unsafe impl NSCalendarDate {
        #[method_id(calendarDate)]
        pub unsafe fn calendarDate() -> Id<Object, Shared>;
        # [method_id (dateWithString : calendarFormat : locale :)]
        pub unsafe fn dateWithString_calendarFormat_locale(
            description: &NSString,
            format: &NSString,
            locale: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;
        # [method_id (dateWithString : calendarFormat :)]
        pub unsafe fn dateWithString_calendarFormat(
            description: &NSString,
            format: &NSString,
        ) -> Option<Id<Object, Shared>>;
        # [method_id (dateWithYear : month : day : hour : minute : second : timeZone :)]
        pub unsafe fn dateWithYear_month_day_hour_minute_second_timeZone(
            year: NSInteger,
            month: NSUInteger,
            day: NSUInteger,
            hour: NSUInteger,
            minute: NSUInteger,
            second: NSUInteger,
            aTimeZone: Option<&NSTimeZone>,
        ) -> Id<Object, Shared>;
        # [method_id (dateByAddingYears : months : days : hours : minutes : seconds :)]
        pub unsafe fn dateByAddingYears_months_days_hours_minutes_seconds(
            &self,
            year: NSInteger,
            month: NSInteger,
            day: NSInteger,
            hour: NSInteger,
            minute: NSInteger,
            second: NSInteger,
        ) -> Id<NSCalendarDate, Shared>;
        #[method(dayOfCommonEra)]
        pub unsafe fn dayOfCommonEra(&self) -> NSInteger;
        #[method(dayOfMonth)]
        pub unsafe fn dayOfMonth(&self) -> NSInteger;
        #[method(dayOfWeek)]
        pub unsafe fn dayOfWeek(&self) -> NSInteger;
        #[method(dayOfYear)]
        pub unsafe fn dayOfYear(&self) -> NSInteger;
        #[method(hourOfDay)]
        pub unsafe fn hourOfDay(&self) -> NSInteger;
        #[method(minuteOfHour)]
        pub unsafe fn minuteOfHour(&self) -> NSInteger;
        #[method(monthOfYear)]
        pub unsafe fn monthOfYear(&self) -> NSInteger;
        #[method(secondOfMinute)]
        pub unsafe fn secondOfMinute(&self) -> NSInteger;
        #[method(yearOfCommonEra)]
        pub unsafe fn yearOfCommonEra(&self) -> NSInteger;
        #[method_id(calendarFormat)]
        pub unsafe fn calendarFormat(&self) -> Id<NSString, Shared>;
        # [method_id (descriptionWithCalendarFormat : locale :)]
        pub unsafe fn descriptionWithCalendarFormat_locale(
            &self,
            format: &NSString,
            locale: Option<&Object>,
        ) -> Id<NSString, Shared>;
        # [method_id (descriptionWithCalendarFormat :)]
        pub unsafe fn descriptionWithCalendarFormat(
            &self,
            format: &NSString,
        ) -> Id<NSString, Shared>;
        # [method_id (descriptionWithLocale :)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>)
            -> Id<NSString, Shared>;
        #[method_id(timeZone)]
        pub unsafe fn timeZone(&self) -> Id<NSTimeZone, Shared>;
        # [method_id (initWithString : calendarFormat : locale :)]
        pub unsafe fn initWithString_calendarFormat_locale(
            &self,
            description: &NSString,
            format: &NSString,
            locale: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;
        # [method_id (initWithString : calendarFormat :)]
        pub unsafe fn initWithString_calendarFormat(
            &self,
            description: &NSString,
            format: &NSString,
        ) -> Option<Id<Object, Shared>>;
        # [method_id (initWithString :)]
        pub unsafe fn initWithString(&self, description: &NSString) -> Option<Id<Object, Shared>>;
        # [method_id (initWithYear : month : day : hour : minute : second : timeZone :)]
        pub unsafe fn initWithYear_month_day_hour_minute_second_timeZone(
            &self,
            year: NSInteger,
            month: NSUInteger,
            day: NSUInteger,
            hour: NSUInteger,
            minute: NSUInteger,
            second: NSUInteger,
            aTimeZone: Option<&NSTimeZone>,
        ) -> Id<Object, Shared>;
        # [method (setCalendarFormat :)]
        pub unsafe fn setCalendarFormat(&self, format: Option<&NSString>);
        # [method (setTimeZone :)]
        pub unsafe fn setTimeZone(&self, aTimeZone: Option<&NSTimeZone>);
        # [method (years : months : days : hours : minutes : seconds : sinceDate :)]
        pub unsafe fn years_months_days_hours_minutes_seconds_sinceDate(
            &self,
            yp: *mut NSInteger,
            mop: *mut NSInteger,
            dp: *mut NSInteger,
            hp: *mut NSInteger,
            mip: *mut NSInteger,
            sp: *mut NSInteger,
            date: &NSCalendarDate,
        );
        #[method_id(distantFuture)]
        pub unsafe fn distantFuture() -> Id<Self, Shared>;
        #[method_id(distantPast)]
        pub unsafe fn distantPast() -> Id<Self, Shared>;
    }
);
extern_methods!(
    #[doc = "NSCalendarDateExtras"]
    unsafe impl NSDate {
        # [method_id (dateWithNaturalLanguageString : locale :)]
        pub unsafe fn dateWithNaturalLanguageString_locale(
            string: &NSString,
            locale: Option<&Object>,
        ) -> Option<Id<Object, Shared>>;
        # [method_id (dateWithNaturalLanguageString :)]
        pub unsafe fn dateWithNaturalLanguageString(
            string: &NSString,
        ) -> Option<Id<Object, Shared>>;
        # [method_id (dateWithString :)]
        pub unsafe fn dateWithString(aString: &NSString) -> Id<Object, Shared>;
        # [method_id (dateWithCalendarFormat : timeZone :)]
        pub unsafe fn dateWithCalendarFormat_timeZone(
            &self,
            format: Option<&NSString>,
            aTimeZone: Option<&NSTimeZone>,
        ) -> Id<NSCalendarDate, Shared>;
        # [method_id (descriptionWithCalendarFormat : timeZone : locale :)]
        pub unsafe fn descriptionWithCalendarFormat_timeZone_locale(
            &self,
            format: Option<&NSString>,
            aTimeZone: Option<&NSTimeZone>,
            locale: Option<&Object>,
        ) -> Option<Id<NSString, Shared>>;
        # [method_id (initWithString :)]
        pub unsafe fn initWithString(&self, description: &NSString) -> Option<Id<Object, Shared>>;
    }
);
