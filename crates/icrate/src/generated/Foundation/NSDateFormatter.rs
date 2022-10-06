use super::__exported::NSArray;
use super::__exported::NSCalendar;
use super::__exported::NSDate;
use super::__exported::NSError;
use super::__exported::NSLocale;
use super::__exported::NSMutableDictionary;
use super::__exported::NSString;
use super::__exported::NSTimeZone;
use crate::CoreFoundation::generated::CFDateFormatter::*;
use crate::Foundation::generated::NSFormatter::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDateFormatter;
    unsafe impl ClassType for NSDateFormatter {
        type Super = NSFormatter;
    }
);
impl NSDateFormatter {
    pub unsafe fn getObjectValue_forString_range_error(
        &self,
        obj: Option<&mut Option<Id<Object, Shared>>>,
        string: &NSString,
        rangep: *mut NSRange,
        error: *mut *mut NSError,
    ) -> bool {
        msg_send![
            self,
            getObjectValue: obj,
            forString: string,
            range: rangep,
            error: error
        ]
    }
    pub unsafe fn stringFromDate(&self, date: &NSDate) -> Id<NSString, Shared> {
        msg_send_id![self, stringFromDate: date]
    }
    pub unsafe fn dateFromString(&self, string: &NSString) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, dateFromString: string]
    }
    pub unsafe fn localizedStringFromDate_dateStyle_timeStyle(
        date: &NSDate,
        dstyle: NSDateFormatterStyle,
        tstyle: NSDateFormatterStyle,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            Self::class(),
            localizedStringFromDate: date,
            dateStyle: dstyle,
            timeStyle: tstyle
        ]
    }
    pub unsafe fn dateFormatFromTemplate_options_locale(
        tmplate: &NSString,
        opts: NSUInteger,
        locale: Option<&NSLocale>,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![
            Self::class(),
            dateFormatFromTemplate: tmplate,
            options: opts,
            locale: locale
        ]
    }
    pub unsafe fn setLocalizedDateFormatFromTemplate(&self, dateFormatTemplate: &NSString) {
        msg_send![self, setLocalizedDateFormatFromTemplate: dateFormatTemplate]
    }
    pub unsafe fn formattingContext(&self) -> NSFormattingContext {
        msg_send![self, formattingContext]
    }
    pub unsafe fn setFormattingContext(&self, formattingContext: NSFormattingContext) {
        msg_send![self, setFormattingContext: formattingContext]
    }
    pub unsafe fn defaultFormatterBehavior() -> NSDateFormatterBehavior {
        msg_send![Self::class(), defaultFormatterBehavior]
    }
    pub unsafe fn setDefaultFormatterBehavior(defaultFormatterBehavior: NSDateFormatterBehavior) {
        msg_send![
            Self::class(),
            setDefaultFormatterBehavior: defaultFormatterBehavior
        ]
    }
    pub unsafe fn dateFormat(&self) -> Id<NSString, Shared> {
        msg_send_id![self, dateFormat]
    }
    pub unsafe fn setDateFormat(&self, dateFormat: Option<&NSString>) {
        msg_send![self, setDateFormat: dateFormat]
    }
    pub unsafe fn dateStyle(&self) -> NSDateFormatterStyle {
        msg_send![self, dateStyle]
    }
    pub unsafe fn setDateStyle(&self, dateStyle: NSDateFormatterStyle) {
        msg_send![self, setDateStyle: dateStyle]
    }
    pub unsafe fn timeStyle(&self) -> NSDateFormatterStyle {
        msg_send![self, timeStyle]
    }
    pub unsafe fn setTimeStyle(&self, timeStyle: NSDateFormatterStyle) {
        msg_send![self, setTimeStyle: timeStyle]
    }
    pub unsafe fn locale(&self) -> Id<NSLocale, Shared> {
        msg_send_id![self, locale]
    }
    pub unsafe fn setLocale(&self, locale: Option<&NSLocale>) {
        msg_send![self, setLocale: locale]
    }
    pub unsafe fn generatesCalendarDates(&self) -> bool {
        msg_send![self, generatesCalendarDates]
    }
    pub unsafe fn setGeneratesCalendarDates(&self, generatesCalendarDates: bool) {
        msg_send![self, setGeneratesCalendarDates: generatesCalendarDates]
    }
    pub unsafe fn formatterBehavior(&self) -> NSDateFormatterBehavior {
        msg_send![self, formatterBehavior]
    }
    pub unsafe fn setFormatterBehavior(&self, formatterBehavior: NSDateFormatterBehavior) {
        msg_send![self, setFormatterBehavior: formatterBehavior]
    }
    pub unsafe fn timeZone(&self) -> Id<NSTimeZone, Shared> {
        msg_send_id![self, timeZone]
    }
    pub unsafe fn setTimeZone(&self, timeZone: Option<&NSTimeZone>) {
        msg_send![self, setTimeZone: timeZone]
    }
    pub unsafe fn calendar(&self) -> Id<NSCalendar, Shared> {
        msg_send_id![self, calendar]
    }
    pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>) {
        msg_send![self, setCalendar: calendar]
    }
    pub unsafe fn isLenient(&self) -> bool {
        msg_send![self, isLenient]
    }
    pub unsafe fn setLenient(&self, lenient: bool) {
        msg_send![self, setLenient: lenient]
    }
    pub unsafe fn twoDigitStartDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, twoDigitStartDate]
    }
    pub unsafe fn setTwoDigitStartDate(&self, twoDigitStartDate: Option<&NSDate>) {
        msg_send![self, setTwoDigitStartDate: twoDigitStartDate]
    }
    pub unsafe fn defaultDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, defaultDate]
    }
    pub unsafe fn setDefaultDate(&self, defaultDate: Option<&NSDate>) {
        msg_send![self, setDefaultDate: defaultDate]
    }
    pub unsafe fn eraSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, eraSymbols]
    }
    pub unsafe fn setEraSymbols(&self, eraSymbols: Option<&NSArray<NSString>>) {
        msg_send![self, setEraSymbols: eraSymbols]
    }
    pub unsafe fn monthSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, monthSymbols]
    }
    pub unsafe fn setMonthSymbols(&self, monthSymbols: Option<&NSArray<NSString>>) {
        msg_send![self, setMonthSymbols: monthSymbols]
    }
    pub unsafe fn shortMonthSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, shortMonthSymbols]
    }
    pub unsafe fn setShortMonthSymbols(&self, shortMonthSymbols: Option<&NSArray<NSString>>) {
        msg_send![self, setShortMonthSymbols: shortMonthSymbols]
    }
    pub unsafe fn weekdaySymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, weekdaySymbols]
    }
    pub unsafe fn setWeekdaySymbols(&self, weekdaySymbols: Option<&NSArray<NSString>>) {
        msg_send![self, setWeekdaySymbols: weekdaySymbols]
    }
    pub unsafe fn shortWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, shortWeekdaySymbols]
    }
    pub unsafe fn setShortWeekdaySymbols(&self, shortWeekdaySymbols: Option<&NSArray<NSString>>) {
        msg_send![self, setShortWeekdaySymbols: shortWeekdaySymbols]
    }
    pub unsafe fn AMSymbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, AMSymbol]
    }
    pub unsafe fn setAMSymbol(&self, AMSymbol: Option<&NSString>) {
        msg_send![self, setAMSymbol: AMSymbol]
    }
    pub unsafe fn PMSymbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, PMSymbol]
    }
    pub unsafe fn setPMSymbol(&self, PMSymbol: Option<&NSString>) {
        msg_send![self, setPMSymbol: PMSymbol]
    }
    pub unsafe fn longEraSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, longEraSymbols]
    }
    pub unsafe fn setLongEraSymbols(&self, longEraSymbols: Option<&NSArray<NSString>>) {
        msg_send![self, setLongEraSymbols: longEraSymbols]
    }
    pub unsafe fn veryShortMonthSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, veryShortMonthSymbols]
    }
    pub unsafe fn setVeryShortMonthSymbols(
        &self,
        veryShortMonthSymbols: Option<&NSArray<NSString>>,
    ) {
        msg_send![self, setVeryShortMonthSymbols: veryShortMonthSymbols]
    }
    pub unsafe fn standaloneMonthSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, standaloneMonthSymbols]
    }
    pub unsafe fn setStandaloneMonthSymbols(
        &self,
        standaloneMonthSymbols: Option<&NSArray<NSString>>,
    ) {
        msg_send![self, setStandaloneMonthSymbols: standaloneMonthSymbols]
    }
    pub unsafe fn shortStandaloneMonthSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, shortStandaloneMonthSymbols]
    }
    pub unsafe fn setShortStandaloneMonthSymbols(
        &self,
        shortStandaloneMonthSymbols: Option<&NSArray<NSString>>,
    ) {
        msg_send![
            self,
            setShortStandaloneMonthSymbols: shortStandaloneMonthSymbols
        ]
    }
    pub unsafe fn veryShortStandaloneMonthSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, veryShortStandaloneMonthSymbols]
    }
    pub unsafe fn setVeryShortStandaloneMonthSymbols(
        &self,
        veryShortStandaloneMonthSymbols: Option<&NSArray<NSString>>,
    ) {
        msg_send![
            self,
            setVeryShortStandaloneMonthSymbols: veryShortStandaloneMonthSymbols
        ]
    }
    pub unsafe fn veryShortWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, veryShortWeekdaySymbols]
    }
    pub unsafe fn setVeryShortWeekdaySymbols(
        &self,
        veryShortWeekdaySymbols: Option<&NSArray<NSString>>,
    ) {
        msg_send![self, setVeryShortWeekdaySymbols: veryShortWeekdaySymbols]
    }
    pub unsafe fn standaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, standaloneWeekdaySymbols]
    }
    pub unsafe fn setStandaloneWeekdaySymbols(
        &self,
        standaloneWeekdaySymbols: Option<&NSArray<NSString>>,
    ) {
        msg_send![self, setStandaloneWeekdaySymbols: standaloneWeekdaySymbols]
    }
    pub unsafe fn shortStandaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, shortStandaloneWeekdaySymbols]
    }
    pub unsafe fn setShortStandaloneWeekdaySymbols(
        &self,
        shortStandaloneWeekdaySymbols: Option<&NSArray<NSString>>,
    ) {
        msg_send![
            self,
            setShortStandaloneWeekdaySymbols: shortStandaloneWeekdaySymbols
        ]
    }
    pub unsafe fn veryShortStandaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, veryShortStandaloneWeekdaySymbols]
    }
    pub unsafe fn setVeryShortStandaloneWeekdaySymbols(
        &self,
        veryShortStandaloneWeekdaySymbols: Option<&NSArray<NSString>>,
    ) {
        msg_send![
            self,
            setVeryShortStandaloneWeekdaySymbols: veryShortStandaloneWeekdaySymbols
        ]
    }
    pub unsafe fn quarterSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, quarterSymbols]
    }
    pub unsafe fn setQuarterSymbols(&self, quarterSymbols: Option<&NSArray<NSString>>) {
        msg_send![self, setQuarterSymbols: quarterSymbols]
    }
    pub unsafe fn shortQuarterSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, shortQuarterSymbols]
    }
    pub unsafe fn setShortQuarterSymbols(&self, shortQuarterSymbols: Option<&NSArray<NSString>>) {
        msg_send![self, setShortQuarterSymbols: shortQuarterSymbols]
    }
    pub unsafe fn standaloneQuarterSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, standaloneQuarterSymbols]
    }
    pub unsafe fn setStandaloneQuarterSymbols(
        &self,
        standaloneQuarterSymbols: Option<&NSArray<NSString>>,
    ) {
        msg_send![self, setStandaloneQuarterSymbols: standaloneQuarterSymbols]
    }
    pub unsafe fn shortStandaloneQuarterSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, shortStandaloneQuarterSymbols]
    }
    pub unsafe fn setShortStandaloneQuarterSymbols(
        &self,
        shortStandaloneQuarterSymbols: Option<&NSArray<NSString>>,
    ) {
        msg_send![
            self,
            setShortStandaloneQuarterSymbols: shortStandaloneQuarterSymbols
        ]
    }
    pub unsafe fn gregorianStartDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, gregorianStartDate]
    }
    pub unsafe fn setGregorianStartDate(&self, gregorianStartDate: Option<&NSDate>) {
        msg_send![self, setGregorianStartDate: gregorianStartDate]
    }
    pub unsafe fn doesRelativeDateFormatting(&self) -> bool {
        msg_send![self, doesRelativeDateFormatting]
    }
    pub unsafe fn setDoesRelativeDateFormatting(&self, doesRelativeDateFormatting: bool) {
        msg_send![
            self,
            setDoesRelativeDateFormatting: doesRelativeDateFormatting
        ]
    }
}
#[doc = "NSDateFormatterCompatibility"]
impl NSDateFormatter {
    pub unsafe fn initWithDateFormat_allowNaturalLanguage(
        &self,
        format: &NSString,
        flag: bool,
    ) -> Id<Object, Shared> {
        msg_send_id![self, initWithDateFormat: format, allowNaturalLanguage: flag]
    }
    pub unsafe fn allowsNaturalLanguage(&self) -> bool {
        msg_send![self, allowsNaturalLanguage]
    }
}
