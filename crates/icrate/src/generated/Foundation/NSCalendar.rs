use super::__exported::NSArray;
use super::__exported::NSLocale;
use super::__exported::NSString;
use super::__exported::NSTimeZone;
use crate::CoreFoundation::generated::CFCalendar::*;
use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSCalendarIdentifier = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSCalendar;
    unsafe impl ClassType for NSCalendar {
        type Super = NSObject;
    }
);
impl NSCalendar {
    pub unsafe fn calendarWithIdentifier(
        calendarIdentifierConstant: &NSCalendarIdentifier,
    ) -> Option<Id<NSCalendar, Shared>> {
        msg_send_id![
            Self::class(),
            calendarWithIdentifier: calendarIdentifierConstant
        ]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithCalendarIdentifier(
        &self,
        ident: &NSCalendarIdentifier,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, initWithCalendarIdentifier: ident]
    }
    pub unsafe fn minimumRangeOfUnit(&self, unit: NSCalendarUnit) -> NSRange {
        msg_send![self, minimumRangeOfUnit: unit]
    }
    pub unsafe fn maximumRangeOfUnit(&self, unit: NSCalendarUnit) -> NSRange {
        msg_send![self, maximumRangeOfUnit: unit]
    }
    pub unsafe fn rangeOfUnit_inUnit_forDate(
        &self,
        smaller: NSCalendarUnit,
        larger: NSCalendarUnit,
        date: &NSDate,
    ) -> NSRange {
        msg_send![self, rangeOfUnit: smaller, inUnit: larger, forDate: date]
    }
    pub unsafe fn ordinalityOfUnit_inUnit_forDate(
        &self,
        smaller: NSCalendarUnit,
        larger: NSCalendarUnit,
        date: &NSDate,
    ) -> NSUInteger {
        msg_send![
            self,
            ordinalityOfUnit: smaller,
            inUnit: larger,
            forDate: date
        ]
    }
    pub unsafe fn rangeOfUnit_startDate_interval_forDate(
        &self,
        unit: NSCalendarUnit,
        datep: Option<&mut Option<Id<NSDate, Shared>>>,
        tip: *mut NSTimeInterval,
        date: &NSDate,
    ) -> bool {
        msg_send![
            self,
            rangeOfUnit: unit,
            startDate: datep,
            interval: tip,
            forDate: date
        ]
    }
    pub unsafe fn dateFromComponents(
        &self,
        comps: &NSDateComponents,
    ) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, dateFromComponents: comps]
    }
    pub unsafe fn components_fromDate(
        &self,
        unitFlags: NSCalendarUnit,
        date: &NSDate,
    ) -> Id<NSDateComponents, Shared> {
        msg_send_id![self, components: unitFlags, fromDate: date]
    }
    pub unsafe fn dateByAddingComponents_toDate_options(
        &self,
        comps: &NSDateComponents,
        date: &NSDate,
        opts: NSCalendarOptions,
    ) -> Option<Id<NSDate, Shared>> {
        msg_send_id![
            self,
            dateByAddingComponents: comps,
            toDate: date,
            options: opts
        ]
    }
    pub unsafe fn components_fromDate_toDate_options(
        &self,
        unitFlags: NSCalendarUnit,
        startingDate: &NSDate,
        resultDate: &NSDate,
        opts: NSCalendarOptions,
    ) -> Id<NSDateComponents, Shared> {
        msg_send_id![
            self,
            components: unitFlags,
            fromDate: startingDate,
            toDate: resultDate,
            options: opts
        ]
    }
    pub unsafe fn getEra_year_month_day_fromDate(
        &self,
        eraValuePointer: *mut NSInteger,
        yearValuePointer: *mut NSInteger,
        monthValuePointer: *mut NSInteger,
        dayValuePointer: *mut NSInteger,
        date: &NSDate,
    ) {
        msg_send![
            self,
            getEra: eraValuePointer,
            year: yearValuePointer,
            month: monthValuePointer,
            day: dayValuePointer,
            fromDate: date
        ]
    }
    pub unsafe fn getEra_yearForWeekOfYear_weekOfYear_weekday_fromDate(
        &self,
        eraValuePointer: *mut NSInteger,
        yearValuePointer: *mut NSInteger,
        weekValuePointer: *mut NSInteger,
        weekdayValuePointer: *mut NSInteger,
        date: &NSDate,
    ) {
        msg_send![
            self,
            getEra: eraValuePointer,
            yearForWeekOfYear: yearValuePointer,
            weekOfYear: weekValuePointer,
            weekday: weekdayValuePointer,
            fromDate: date
        ]
    }
    pub unsafe fn getHour_minute_second_nanosecond_fromDate(
        &self,
        hourValuePointer: *mut NSInteger,
        minuteValuePointer: *mut NSInteger,
        secondValuePointer: *mut NSInteger,
        nanosecondValuePointer: *mut NSInteger,
        date: &NSDate,
    ) {
        msg_send![
            self,
            getHour: hourValuePointer,
            minute: minuteValuePointer,
            second: secondValuePointer,
            nanosecond: nanosecondValuePointer,
            fromDate: date
        ]
    }
    pub unsafe fn component_fromDate(&self, unit: NSCalendarUnit, date: &NSDate) -> NSInteger {
        msg_send![self, component: unit, fromDate: date]
    }
    pub unsafe fn dateWithEra_year_month_day_hour_minute_second_nanosecond(
        &self,
        eraValue: NSInteger,
        yearValue: NSInteger,
        monthValue: NSInteger,
        dayValue: NSInteger,
        hourValue: NSInteger,
        minuteValue: NSInteger,
        secondValue: NSInteger,
        nanosecondValue: NSInteger,
    ) -> Option<Id<NSDate, Shared>> {
        msg_send_id![
            self,
            dateWithEra: eraValue,
            year: yearValue,
            month: monthValue,
            day: dayValue,
            hour: hourValue,
            minute: minuteValue,
            second: secondValue,
            nanosecond: nanosecondValue
        ]
    }
    pub unsafe fn dateWithEra_yearForWeekOfYear_weekOfYear_weekday_hour_minute_second_nanosecond(
        &self,
        eraValue: NSInteger,
        yearValue: NSInteger,
        weekValue: NSInteger,
        weekdayValue: NSInteger,
        hourValue: NSInteger,
        minuteValue: NSInteger,
        secondValue: NSInteger,
        nanosecondValue: NSInteger,
    ) -> Option<Id<NSDate, Shared>> {
        msg_send_id![
            self,
            dateWithEra: eraValue,
            yearForWeekOfYear: yearValue,
            weekOfYear: weekValue,
            weekday: weekdayValue,
            hour: hourValue,
            minute: minuteValue,
            second: secondValue,
            nanosecond: nanosecondValue
        ]
    }
    pub unsafe fn startOfDayForDate(&self, date: &NSDate) -> Id<NSDate, Shared> {
        msg_send_id![self, startOfDayForDate: date]
    }
    pub unsafe fn componentsInTimeZone_fromDate(
        &self,
        timezone: &NSTimeZone,
        date: &NSDate,
    ) -> Id<NSDateComponents, Shared> {
        msg_send_id![self, componentsInTimeZone: timezone, fromDate: date]
    }
    pub unsafe fn compareDate_toDate_toUnitGranularity(
        &self,
        date1: &NSDate,
        date2: &NSDate,
        unit: NSCalendarUnit,
    ) -> NSComparisonResult {
        msg_send![
            self,
            compareDate: date1,
            toDate: date2,
            toUnitGranularity: unit
        ]
    }
    pub unsafe fn isDate_equalToDate_toUnitGranularity(
        &self,
        date1: &NSDate,
        date2: &NSDate,
        unit: NSCalendarUnit,
    ) -> bool {
        msg_send![
            self,
            isDate: date1,
            equalToDate: date2,
            toUnitGranularity: unit
        ]
    }
    pub unsafe fn isDate_inSameDayAsDate(&self, date1: &NSDate, date2: &NSDate) -> bool {
        msg_send![self, isDate: date1, inSameDayAsDate: date2]
    }
    pub unsafe fn isDateInToday(&self, date: &NSDate) -> bool {
        msg_send![self, isDateInToday: date]
    }
    pub unsafe fn isDateInYesterday(&self, date: &NSDate) -> bool {
        msg_send![self, isDateInYesterday: date]
    }
    pub unsafe fn isDateInTomorrow(&self, date: &NSDate) -> bool {
        msg_send![self, isDateInTomorrow: date]
    }
    pub unsafe fn isDateInWeekend(&self, date: &NSDate) -> bool {
        msg_send![self, isDateInWeekend: date]
    }
    pub unsafe fn rangeOfWeekendStartDate_interval_containingDate(
        &self,
        datep: Option<&mut Option<Id<NSDate, Shared>>>,
        tip: *mut NSTimeInterval,
        date: &NSDate,
    ) -> bool {
        msg_send![
            self,
            rangeOfWeekendStartDate: datep,
            interval: tip,
            containingDate: date
        ]
    }
    pub unsafe fn nextWeekendStartDate_interval_options_afterDate(
        &self,
        datep: Option<&mut Option<Id<NSDate, Shared>>>,
        tip: *mut NSTimeInterval,
        options: NSCalendarOptions,
        date: &NSDate,
    ) -> bool {
        msg_send![
            self,
            nextWeekendStartDate: datep,
            interval: tip,
            options: options,
            afterDate: date
        ]
    }
    pub unsafe fn components_fromDateComponents_toDateComponents_options(
        &self,
        unitFlags: NSCalendarUnit,
        startingDateComp: &NSDateComponents,
        resultDateComp: &NSDateComponents,
        options: NSCalendarOptions,
    ) -> Id<NSDateComponents, Shared> {
        msg_send_id![
            self,
            components: unitFlags,
            fromDateComponents: startingDateComp,
            toDateComponents: resultDateComp,
            options: options
        ]
    }
    pub unsafe fn dateByAddingUnit_value_toDate_options(
        &self,
        unit: NSCalendarUnit,
        value: NSInteger,
        date: &NSDate,
        options: NSCalendarOptions,
    ) -> Option<Id<NSDate, Shared>> {
        msg_send_id![
            self,
            dateByAddingUnit: unit,
            value: value,
            toDate: date,
            options: options
        ]
    }
    pub unsafe fn enumerateDatesStartingAfterDate_matchingComponents_options_usingBlock(
        &self,
        start: &NSDate,
        comps: &NSDateComponents,
        opts: NSCalendarOptions,
        block: TodoBlock,
    ) {
        msg_send![
            self,
            enumerateDatesStartingAfterDate: start,
            matchingComponents: comps,
            options: opts,
            usingBlock: block
        ]
    }
    pub unsafe fn nextDateAfterDate_matchingComponents_options(
        &self,
        date: &NSDate,
        comps: &NSDateComponents,
        options: NSCalendarOptions,
    ) -> Option<Id<NSDate, Shared>> {
        msg_send_id![
            self,
            nextDateAfterDate: date,
            matchingComponents: comps,
            options: options
        ]
    }
    pub unsafe fn nextDateAfterDate_matchingUnit_value_options(
        &self,
        date: &NSDate,
        unit: NSCalendarUnit,
        value: NSInteger,
        options: NSCalendarOptions,
    ) -> Option<Id<NSDate, Shared>> {
        msg_send_id![
            self,
            nextDateAfterDate: date,
            matchingUnit: unit,
            value: value,
            options: options
        ]
    }
    pub unsafe fn nextDateAfterDate_matchingHour_minute_second_options(
        &self,
        date: &NSDate,
        hourValue: NSInteger,
        minuteValue: NSInteger,
        secondValue: NSInteger,
        options: NSCalendarOptions,
    ) -> Option<Id<NSDate, Shared>> {
        msg_send_id![
            self,
            nextDateAfterDate: date,
            matchingHour: hourValue,
            minute: minuteValue,
            second: secondValue,
            options: options
        ]
    }
    pub unsafe fn dateBySettingUnit_value_ofDate_options(
        &self,
        unit: NSCalendarUnit,
        v: NSInteger,
        date: &NSDate,
        opts: NSCalendarOptions,
    ) -> Option<Id<NSDate, Shared>> {
        msg_send_id![
            self,
            dateBySettingUnit: unit,
            value: v,
            ofDate: date,
            options: opts
        ]
    }
    pub unsafe fn dateBySettingHour_minute_second_ofDate_options(
        &self,
        h: NSInteger,
        m: NSInteger,
        s: NSInteger,
        date: &NSDate,
        opts: NSCalendarOptions,
    ) -> Option<Id<NSDate, Shared>> {
        msg_send_id![
            self,
            dateBySettingHour: h,
            minute: m,
            second: s,
            ofDate: date,
            options: opts
        ]
    }
    pub unsafe fn date_matchesComponents(
        &self,
        date: &NSDate,
        components: &NSDateComponents,
    ) -> bool {
        msg_send![self, date: date, matchesComponents: components]
    }
    pub unsafe fn currentCalendar() -> Id<NSCalendar, Shared> {
        msg_send_id![Self::class(), currentCalendar]
    }
    pub unsafe fn autoupdatingCurrentCalendar() -> Id<NSCalendar, Shared> {
        msg_send_id![Self::class(), autoupdatingCurrentCalendar]
    }
    pub unsafe fn calendarIdentifier(&self) -> Id<NSCalendarIdentifier, Shared> {
        msg_send_id![self, calendarIdentifier]
    }
    pub unsafe fn locale(&self) -> Option<Id<NSLocale, Shared>> {
        msg_send_id![self, locale]
    }
    pub unsafe fn setLocale(&self, locale: Option<&NSLocale>) {
        msg_send![self, setLocale: locale]
    }
    pub unsafe fn timeZone(&self) -> Id<NSTimeZone, Shared> {
        msg_send_id![self, timeZone]
    }
    pub unsafe fn setTimeZone(&self, timeZone: &NSTimeZone) {
        msg_send![self, setTimeZone: timeZone]
    }
    pub unsafe fn firstWeekday(&self) -> NSUInteger {
        msg_send![self, firstWeekday]
    }
    pub unsafe fn setFirstWeekday(&self, firstWeekday: NSUInteger) {
        msg_send![self, setFirstWeekday: firstWeekday]
    }
    pub unsafe fn minimumDaysInFirstWeek(&self) -> NSUInteger {
        msg_send![self, minimumDaysInFirstWeek]
    }
    pub unsafe fn setMinimumDaysInFirstWeek(&self, minimumDaysInFirstWeek: NSUInteger) {
        msg_send![self, setMinimumDaysInFirstWeek: minimumDaysInFirstWeek]
    }
    pub unsafe fn eraSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, eraSymbols]
    }
    pub unsafe fn longEraSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, longEraSymbols]
    }
    pub unsafe fn monthSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, monthSymbols]
    }
    pub unsafe fn shortMonthSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, shortMonthSymbols]
    }
    pub unsafe fn veryShortMonthSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, veryShortMonthSymbols]
    }
    pub unsafe fn standaloneMonthSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, standaloneMonthSymbols]
    }
    pub unsafe fn shortStandaloneMonthSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, shortStandaloneMonthSymbols]
    }
    pub unsafe fn veryShortStandaloneMonthSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, veryShortStandaloneMonthSymbols]
    }
    pub unsafe fn weekdaySymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, weekdaySymbols]
    }
    pub unsafe fn shortWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, shortWeekdaySymbols]
    }
    pub unsafe fn veryShortWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, veryShortWeekdaySymbols]
    }
    pub unsafe fn standaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, standaloneWeekdaySymbols]
    }
    pub unsafe fn shortStandaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, shortStandaloneWeekdaySymbols]
    }
    pub unsafe fn veryShortStandaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, veryShortStandaloneWeekdaySymbols]
    }
    pub unsafe fn quarterSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, quarterSymbols]
    }
    pub unsafe fn shortQuarterSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, shortQuarterSymbols]
    }
    pub unsafe fn standaloneQuarterSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, standaloneQuarterSymbols]
    }
    pub unsafe fn shortStandaloneQuarterSymbols(&self) -> Id<NSArray<NSString>, Shared> {
        msg_send_id![self, shortStandaloneQuarterSymbols]
    }
    pub unsafe fn AMSymbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, AMSymbol]
    }
    pub unsafe fn PMSymbol(&self) -> Id<NSString, Shared> {
        msg_send_id![self, PMSymbol]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSDateComponents;
    unsafe impl ClassType for NSDateComponents {
        type Super = NSObject;
    }
);
impl NSDateComponents {
    pub unsafe fn week(&self) -> NSInteger {
        msg_send![self, week]
    }
    pub unsafe fn setWeek(&self, v: NSInteger) {
        msg_send![self, setWeek: v]
    }
    pub unsafe fn setValue_forComponent(&self, value: NSInteger, unit: NSCalendarUnit) {
        msg_send![self, setValue: value, forComponent: unit]
    }
    pub unsafe fn valueForComponent(&self, unit: NSCalendarUnit) -> NSInteger {
        msg_send![self, valueForComponent: unit]
    }
    pub unsafe fn isValidDateInCalendar(&self, calendar: &NSCalendar) -> bool {
        msg_send![self, isValidDateInCalendar: calendar]
    }
    pub unsafe fn calendar(&self) -> Option<Id<NSCalendar, Shared>> {
        msg_send_id![self, calendar]
    }
    pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>) {
        msg_send![self, setCalendar: calendar]
    }
    pub unsafe fn timeZone(&self) -> Option<Id<NSTimeZone, Shared>> {
        msg_send_id![self, timeZone]
    }
    pub unsafe fn setTimeZone(&self, timeZone: Option<&NSTimeZone>) {
        msg_send![self, setTimeZone: timeZone]
    }
    pub unsafe fn era(&self) -> NSInteger {
        msg_send![self, era]
    }
    pub unsafe fn setEra(&self, era: NSInteger) {
        msg_send![self, setEra: era]
    }
    pub unsafe fn year(&self) -> NSInteger {
        msg_send![self, year]
    }
    pub unsafe fn setYear(&self, year: NSInteger) {
        msg_send![self, setYear: year]
    }
    pub unsafe fn month(&self) -> NSInteger {
        msg_send![self, month]
    }
    pub unsafe fn setMonth(&self, month: NSInteger) {
        msg_send![self, setMonth: month]
    }
    pub unsafe fn day(&self) -> NSInteger {
        msg_send![self, day]
    }
    pub unsafe fn setDay(&self, day: NSInteger) {
        msg_send![self, setDay: day]
    }
    pub unsafe fn hour(&self) -> NSInteger {
        msg_send![self, hour]
    }
    pub unsafe fn setHour(&self, hour: NSInteger) {
        msg_send![self, setHour: hour]
    }
    pub unsafe fn minute(&self) -> NSInteger {
        msg_send![self, minute]
    }
    pub unsafe fn setMinute(&self, minute: NSInteger) {
        msg_send![self, setMinute: minute]
    }
    pub unsafe fn second(&self) -> NSInteger {
        msg_send![self, second]
    }
    pub unsafe fn setSecond(&self, second: NSInteger) {
        msg_send![self, setSecond: second]
    }
    pub unsafe fn nanosecond(&self) -> NSInteger {
        msg_send![self, nanosecond]
    }
    pub unsafe fn setNanosecond(&self, nanosecond: NSInteger) {
        msg_send![self, setNanosecond: nanosecond]
    }
    pub unsafe fn weekday(&self) -> NSInteger {
        msg_send![self, weekday]
    }
    pub unsafe fn setWeekday(&self, weekday: NSInteger) {
        msg_send![self, setWeekday: weekday]
    }
    pub unsafe fn weekdayOrdinal(&self) -> NSInteger {
        msg_send![self, weekdayOrdinal]
    }
    pub unsafe fn setWeekdayOrdinal(&self, weekdayOrdinal: NSInteger) {
        msg_send![self, setWeekdayOrdinal: weekdayOrdinal]
    }
    pub unsafe fn quarter(&self) -> NSInteger {
        msg_send![self, quarter]
    }
    pub unsafe fn setQuarter(&self, quarter: NSInteger) {
        msg_send![self, setQuarter: quarter]
    }
    pub unsafe fn weekOfMonth(&self) -> NSInteger {
        msg_send![self, weekOfMonth]
    }
    pub unsafe fn setWeekOfMonth(&self, weekOfMonth: NSInteger) {
        msg_send![self, setWeekOfMonth: weekOfMonth]
    }
    pub unsafe fn weekOfYear(&self) -> NSInteger {
        msg_send![self, weekOfYear]
    }
    pub unsafe fn setWeekOfYear(&self, weekOfYear: NSInteger) {
        msg_send![self, setWeekOfYear: weekOfYear]
    }
    pub unsafe fn yearForWeekOfYear(&self) -> NSInteger {
        msg_send![self, yearForWeekOfYear]
    }
    pub unsafe fn setYearForWeekOfYear(&self, yearForWeekOfYear: NSInteger) {
        msg_send![self, setYearForWeekOfYear: yearForWeekOfYear]
    }
    pub unsafe fn isLeapMonth(&self) -> bool {
        msg_send![self, isLeapMonth]
    }
    pub unsafe fn setLeapMonth(&self, leapMonth: bool) {
        msg_send![self, setLeapMonth: leapMonth]
    }
    pub unsafe fn date(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, date]
    }
    pub unsafe fn isValidDate(&self) -> bool {
        msg_send![self, isValidDate]
    }
}
