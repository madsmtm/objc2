//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSDatePickerStyle {
        NSDatePickerStyleTextFieldAndStepper = 0,
        NSDatePickerStyleClockAndCalendar = 1,
        NSDatePickerStyleTextField = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSDatePickerMode {
        NSDatePickerModeSingle = 0,
        NSDatePickerModeRange = 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDatePickerElementFlags {
        NSDatePickerElementFlagHourMinute = 0x000c,
        NSDatePickerElementFlagHourMinuteSecond = 0x000e,
        NSDatePickerElementFlagTimeZone = 0x0010,
        NSDatePickerElementFlagYearMonth = 0x00c0,
        NSDatePickerElementFlagYearMonthDay = 0x00e0,
        NSDatePickerElementFlagEra = 0x0100,
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSDatePickerCell;

    unsafe impl ClassType for NSDatePickerCell {
        type Super = NSActionCell;
    }
);

extern_methods!(
    unsafe impl NSDatePickerCell {
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self, Shared>;

        #[method(datePickerStyle)]
        pub unsafe fn datePickerStyle(&self) -> NSDatePickerStyle;

        #[method(setDatePickerStyle:)]
        pub unsafe fn setDatePickerStyle(&self, datePickerStyle: NSDatePickerStyle);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);

        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;

        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: &NSColor);

        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor(&self) -> Id<NSColor, Shared>;

        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, textColor: &NSColor);

        #[method(datePickerMode)]
        pub unsafe fn datePickerMode(&self) -> NSDatePickerMode;

        #[method(setDatePickerMode:)]
        pub unsafe fn setDatePickerMode(&self, datePickerMode: NSDatePickerMode);

        #[method(datePickerElements)]
        pub unsafe fn datePickerElements(&self) -> NSDatePickerElementFlags;

        #[method(setDatePickerElements:)]
        pub unsafe fn setDatePickerElements(&self, datePickerElements: NSDatePickerElementFlags);

        #[method_id(@__retain_semantics Other calendar)]
        pub unsafe fn calendar(&self) -> Option<Id<NSCalendar, Shared>>;

        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);

        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Option<Id<NSLocale, Shared>>;

        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Id<NSTimeZone, Shared>>;

        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, timeZone: Option<&NSTimeZone>);

        #[method_id(@__retain_semantics Other dateValue)]
        pub unsafe fn dateValue(&self) -> Id<NSDate, Shared>;

        #[method(setDateValue:)]
        pub unsafe fn setDateValue(&self, dateValue: &NSDate);

        #[method(timeInterval)]
        pub unsafe fn timeInterval(&self) -> NSTimeInterval;

        #[method(setTimeInterval:)]
        pub unsafe fn setTimeInterval(&self, timeInterval: NSTimeInterval);

        #[method_id(@__retain_semantics Other minDate)]
        pub unsafe fn minDate(&self) -> Option<Id<NSDate, Shared>>;

        #[method(setMinDate:)]
        pub unsafe fn setMinDate(&self, minDate: Option<&NSDate>);

        #[method_id(@__retain_semantics Other maxDate)]
        pub unsafe fn maxDate(&self) -> Option<Id<NSDate, Shared>>;

        #[method(setMaxDate:)]
        pub unsafe fn setMaxDate(&self, maxDate: Option<&NSDate>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSDatePickerCellDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSDatePickerCellDelegate>);
    }
);

pub type NSDatePickerCellDelegate = NSObject;

extern_static!(
    NSTextFieldAndStepperDatePickerStyle: NSDatePickerStyle = NSDatePickerStyleTextFieldAndStepper
);

extern_static!(
    NSClockAndCalendarDatePickerStyle: NSDatePickerStyle = NSDatePickerStyleClockAndCalendar
);

extern_static!(NSTextFieldDatePickerStyle: NSDatePickerStyle = NSDatePickerStyleTextField);

extern_static!(NSSingleDateMode: NSDatePickerMode = NSDatePickerModeSingle);

extern_static!(NSRangeDateMode: NSDatePickerMode = NSDatePickerModeRange);

extern_static!(
    NSHourMinuteDatePickerElementFlag: NSDatePickerElementFlags = NSDatePickerElementFlagHourMinute
);

extern_static!(
    NSHourMinuteSecondDatePickerElementFlag: NSDatePickerElementFlags =
        NSDatePickerElementFlagHourMinuteSecond
);

extern_static!(
    NSTimeZoneDatePickerElementFlag: NSDatePickerElementFlags = NSDatePickerElementFlagTimeZone
);

extern_static!(
    NSYearMonthDatePickerElementFlag: NSDatePickerElementFlags = NSDatePickerElementFlagYearMonth
);

extern_static!(
    NSYearMonthDayDatePickerElementFlag: NSDatePickerElementFlags =
        NSDatePickerElementFlagYearMonthDay
);

extern_static!(NSEraDatePickerElementFlag: NSDatePickerElementFlags = NSDatePickerElementFlagEra);
