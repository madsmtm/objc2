//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

pub type NSISO8601DateFormatOptions = NSUInteger;
pub const NSISO8601DateFormatWithYear: NSISO8601DateFormatOptions = kCFISO8601DateFormatWithYear;
pub const NSISO8601DateFormatWithMonth: NSISO8601DateFormatOptions = kCFISO8601DateFormatWithMonth;
pub const NSISO8601DateFormatWithWeekOfYear: NSISO8601DateFormatOptions =
    kCFISO8601DateFormatWithWeekOfYear;
pub const NSISO8601DateFormatWithDay: NSISO8601DateFormatOptions = kCFISO8601DateFormatWithDay;
pub const NSISO8601DateFormatWithTime: NSISO8601DateFormatOptions = kCFISO8601DateFormatWithTime;
pub const NSISO8601DateFormatWithTimeZone: NSISO8601DateFormatOptions =
    kCFISO8601DateFormatWithTimeZone;
pub const NSISO8601DateFormatWithSpaceBetweenDateAndTime: NSISO8601DateFormatOptions =
    kCFISO8601DateFormatWithSpaceBetweenDateAndTime;
pub const NSISO8601DateFormatWithDashSeparatorInDate: NSISO8601DateFormatOptions =
    kCFISO8601DateFormatWithDashSeparatorInDate;
pub const NSISO8601DateFormatWithColonSeparatorInTime: NSISO8601DateFormatOptions =
    kCFISO8601DateFormatWithColonSeparatorInTime;
pub const NSISO8601DateFormatWithColonSeparatorInTimeZone: NSISO8601DateFormatOptions =
    kCFISO8601DateFormatWithColonSeparatorInTimeZone;
pub const NSISO8601DateFormatWithFractionalSeconds: NSISO8601DateFormatOptions =
    kCFISO8601DateFormatWithFractionalSeconds;
pub const NSISO8601DateFormatWithFullDate: NSISO8601DateFormatOptions =
    kCFISO8601DateFormatWithFullDate;
pub const NSISO8601DateFormatWithFullTime: NSISO8601DateFormatOptions =
    kCFISO8601DateFormatWithFullTime;
pub const NSISO8601DateFormatWithInternetDateTime: NSISO8601DateFormatOptions =
    kCFISO8601DateFormatWithInternetDateTime;

extern_class!(
    #[derive(Debug)]
    pub struct NSISO8601DateFormatter;

    unsafe impl ClassType for NSISO8601DateFormatter {
        type Super = NSFormatter;
    }
);

extern_methods!(
    unsafe impl NSISO8601DateFormatter {
        #[method_id(timeZone)]
        pub unsafe fn timeZone(&self) -> Id<NSTimeZone, Shared>;

        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, timeZone: Option<&NSTimeZone>);

        #[method(formatOptions)]
        pub unsafe fn formatOptions(&self) -> NSISO8601DateFormatOptions;

        #[method(setFormatOptions:)]
        pub unsafe fn setFormatOptions(&self, formatOptions: NSISO8601DateFormatOptions);

        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;

        #[method_id(stringFromDate:)]
        pub unsafe fn stringFromDate(&self, date: &NSDate) -> Id<NSString, Shared>;

        #[method_id(dateFromString:)]
        pub unsafe fn dateFromString(&self, string: &NSString) -> Option<Id<NSDate, Shared>>;

        #[method_id(stringFromDate:timeZone:formatOptions:)]
        pub unsafe fn stringFromDate_timeZone_formatOptions(
            date: &NSDate,
            timeZone: &NSTimeZone,
            formatOptions: NSISO8601DateFormatOptions,
        ) -> Id<NSString, Shared>;
    }
);
