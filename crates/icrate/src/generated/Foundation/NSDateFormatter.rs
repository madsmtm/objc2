#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDateFormatter;
    unsafe impl ClassType for NSDateFormatter {
        type Super = NSFormatter;
    }
);
extern_methods!(
    unsafe impl NSDateFormatter {
        #[method(formattingContext)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;
        #[method(setFormattingContext:)]
        pub unsafe fn setFormattingContext(&self, formattingContext: NSFormattingContext);
        #[method(getObjectValue:forString:range:error:)]
        pub unsafe fn getObjectValue_forString_range_error(
            &self,
            obj: Option<&mut Option<Id<Object, Shared>>>,
            string: &NSString,
            rangep: *mut NSRange,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method_id(stringFromDate:)]
        pub unsafe fn stringFromDate(&self, date: &NSDate) -> Id<NSString, Shared>;
        #[method_id(dateFromString:)]
        pub unsafe fn dateFromString(&self, string: &NSString) -> Option<Id<NSDate, Shared>>;
        #[method_id(localizedStringFromDate:dateStyle:timeStyle:)]
        pub unsafe fn localizedStringFromDate_dateStyle_timeStyle(
            date: &NSDate,
            dstyle: NSDateFormatterStyle,
            tstyle: NSDateFormatterStyle,
        ) -> Id<NSString, Shared>;
        #[method_id(dateFormatFromTemplate:options:locale:)]
        pub unsafe fn dateFormatFromTemplate_options_locale(
            tmplate: &NSString,
            opts: NSUInteger,
            locale: Option<&NSLocale>,
        ) -> Option<Id<NSString, Shared>>;
        #[method(defaultFormatterBehavior)]
        pub unsafe fn defaultFormatterBehavior() -> NSDateFormatterBehavior;
        #[method(setDefaultFormatterBehavior:)]
        pub unsafe fn setDefaultFormatterBehavior(
            defaultFormatterBehavior: NSDateFormatterBehavior,
        );
        #[method(setLocalizedDateFormatFromTemplate:)]
        pub unsafe fn setLocalizedDateFormatFromTemplate(&self, dateFormatTemplate: &NSString);
        #[method_id(dateFormat)]
        pub unsafe fn dateFormat(&self) -> Id<NSString, Shared>;
        #[method(setDateFormat:)]
        pub unsafe fn setDateFormat(&self, dateFormat: Option<&NSString>);
        #[method(dateStyle)]
        pub unsafe fn dateStyle(&self) -> NSDateFormatterStyle;
        #[method(setDateStyle:)]
        pub unsafe fn setDateStyle(&self, dateStyle: NSDateFormatterStyle);
        #[method(timeStyle)]
        pub unsafe fn timeStyle(&self) -> NSDateFormatterStyle;
        #[method(setTimeStyle:)]
        pub unsafe fn setTimeStyle(&self, timeStyle: NSDateFormatterStyle);
        #[method_id(locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale, Shared>;
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);
        #[method(generatesCalendarDates)]
        pub unsafe fn generatesCalendarDates(&self) -> bool;
        #[method(setGeneratesCalendarDates:)]
        pub unsafe fn setGeneratesCalendarDates(&self, generatesCalendarDates: bool);
        #[method(formatterBehavior)]
        pub unsafe fn formatterBehavior(&self) -> NSDateFormatterBehavior;
        #[method(setFormatterBehavior:)]
        pub unsafe fn setFormatterBehavior(&self, formatterBehavior: NSDateFormatterBehavior);
        #[method_id(timeZone)]
        pub unsafe fn timeZone(&self) -> Id<NSTimeZone, Shared>;
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, timeZone: Option<&NSTimeZone>);
        #[method_id(calendar)]
        pub unsafe fn calendar(&self) -> Id<NSCalendar, Shared>;
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);
        #[method(isLenient)]
        pub unsafe fn isLenient(&self) -> bool;
        #[method(setLenient:)]
        pub unsafe fn setLenient(&self, lenient: bool);
        #[method_id(twoDigitStartDate)]
        pub unsafe fn twoDigitStartDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method(setTwoDigitStartDate:)]
        pub unsafe fn setTwoDigitStartDate(&self, twoDigitStartDate: Option<&NSDate>);
        #[method_id(defaultDate)]
        pub unsafe fn defaultDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method(setDefaultDate:)]
        pub unsafe fn setDefaultDate(&self, defaultDate: Option<&NSDate>);
        #[method_id(eraSymbols)]
        pub unsafe fn eraSymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setEraSymbols:)]
        pub unsafe fn setEraSymbols(&self, eraSymbols: Option<&NSArray<NSString>>);
        #[method_id(monthSymbols)]
        pub unsafe fn monthSymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setMonthSymbols:)]
        pub unsafe fn setMonthSymbols(&self, monthSymbols: Option<&NSArray<NSString>>);
        #[method_id(shortMonthSymbols)]
        pub unsafe fn shortMonthSymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setShortMonthSymbols:)]
        pub unsafe fn setShortMonthSymbols(&self, shortMonthSymbols: Option<&NSArray<NSString>>);
        #[method_id(weekdaySymbols)]
        pub unsafe fn weekdaySymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setWeekdaySymbols:)]
        pub unsafe fn setWeekdaySymbols(&self, weekdaySymbols: Option<&NSArray<NSString>>);
        #[method_id(shortWeekdaySymbols)]
        pub unsafe fn shortWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setShortWeekdaySymbols:)]
        pub unsafe fn setShortWeekdaySymbols(
            &self,
            shortWeekdaySymbols: Option<&NSArray<NSString>>,
        );
        #[method_id(AMSymbol)]
        pub unsafe fn AMSymbol(&self) -> Id<NSString, Shared>;
        #[method(setAMSymbol:)]
        pub unsafe fn setAMSymbol(&self, AMSymbol: Option<&NSString>);
        #[method_id(PMSymbol)]
        pub unsafe fn PMSymbol(&self) -> Id<NSString, Shared>;
        #[method(setPMSymbol:)]
        pub unsafe fn setPMSymbol(&self, PMSymbol: Option<&NSString>);
        #[method_id(longEraSymbols)]
        pub unsafe fn longEraSymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setLongEraSymbols:)]
        pub unsafe fn setLongEraSymbols(&self, longEraSymbols: Option<&NSArray<NSString>>);
        #[method_id(veryShortMonthSymbols)]
        pub unsafe fn veryShortMonthSymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setVeryShortMonthSymbols:)]
        pub unsafe fn setVeryShortMonthSymbols(
            &self,
            veryShortMonthSymbols: Option<&NSArray<NSString>>,
        );
        #[method_id(standaloneMonthSymbols)]
        pub unsafe fn standaloneMonthSymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setStandaloneMonthSymbols:)]
        pub unsafe fn setStandaloneMonthSymbols(
            &self,
            standaloneMonthSymbols: Option<&NSArray<NSString>>,
        );
        #[method_id(shortStandaloneMonthSymbols)]
        pub unsafe fn shortStandaloneMonthSymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setShortStandaloneMonthSymbols:)]
        pub unsafe fn setShortStandaloneMonthSymbols(
            &self,
            shortStandaloneMonthSymbols: Option<&NSArray<NSString>>,
        );
        #[method_id(veryShortStandaloneMonthSymbols)]
        pub unsafe fn veryShortStandaloneMonthSymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setVeryShortStandaloneMonthSymbols:)]
        pub unsafe fn setVeryShortStandaloneMonthSymbols(
            &self,
            veryShortStandaloneMonthSymbols: Option<&NSArray<NSString>>,
        );
        #[method_id(veryShortWeekdaySymbols)]
        pub unsafe fn veryShortWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setVeryShortWeekdaySymbols:)]
        pub unsafe fn setVeryShortWeekdaySymbols(
            &self,
            veryShortWeekdaySymbols: Option<&NSArray<NSString>>,
        );
        #[method_id(standaloneWeekdaySymbols)]
        pub unsafe fn standaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setStandaloneWeekdaySymbols:)]
        pub unsafe fn setStandaloneWeekdaySymbols(
            &self,
            standaloneWeekdaySymbols: Option<&NSArray<NSString>>,
        );
        #[method_id(shortStandaloneWeekdaySymbols)]
        pub unsafe fn shortStandaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setShortStandaloneWeekdaySymbols:)]
        pub unsafe fn setShortStandaloneWeekdaySymbols(
            &self,
            shortStandaloneWeekdaySymbols: Option<&NSArray<NSString>>,
        );
        #[method_id(veryShortStandaloneWeekdaySymbols)]
        pub unsafe fn veryShortStandaloneWeekdaySymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setVeryShortStandaloneWeekdaySymbols:)]
        pub unsafe fn setVeryShortStandaloneWeekdaySymbols(
            &self,
            veryShortStandaloneWeekdaySymbols: Option<&NSArray<NSString>>,
        );
        #[method_id(quarterSymbols)]
        pub unsafe fn quarterSymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setQuarterSymbols:)]
        pub unsafe fn setQuarterSymbols(&self, quarterSymbols: Option<&NSArray<NSString>>);
        #[method_id(shortQuarterSymbols)]
        pub unsafe fn shortQuarterSymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setShortQuarterSymbols:)]
        pub unsafe fn setShortQuarterSymbols(
            &self,
            shortQuarterSymbols: Option<&NSArray<NSString>>,
        );
        #[method_id(standaloneQuarterSymbols)]
        pub unsafe fn standaloneQuarterSymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setStandaloneQuarterSymbols:)]
        pub unsafe fn setStandaloneQuarterSymbols(
            &self,
            standaloneQuarterSymbols: Option<&NSArray<NSString>>,
        );
        #[method_id(shortStandaloneQuarterSymbols)]
        pub unsafe fn shortStandaloneQuarterSymbols(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(setShortStandaloneQuarterSymbols:)]
        pub unsafe fn setShortStandaloneQuarterSymbols(
            &self,
            shortStandaloneQuarterSymbols: Option<&NSArray<NSString>>,
        );
        #[method_id(gregorianStartDate)]
        pub unsafe fn gregorianStartDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method(setGregorianStartDate:)]
        pub unsafe fn setGregorianStartDate(&self, gregorianStartDate: Option<&NSDate>);
        #[method(doesRelativeDateFormatting)]
        pub unsafe fn doesRelativeDateFormatting(&self) -> bool;
        #[method(setDoesRelativeDateFormatting:)]
        pub unsafe fn setDoesRelativeDateFormatting(&self, doesRelativeDateFormatting: bool);
    }
);
extern_methods!(
    #[doc = "NSDateFormatterCompatibility"]
    unsafe impl NSDateFormatter {
        #[method_id(initWithDateFormat:allowNaturalLanguage:)]
        pub unsafe fn initWithDateFormat_allowNaturalLanguage(
            &self,
            format: &NSString,
            flag: bool,
        ) -> Id<Object, Shared>;
        #[method(allowsNaturalLanguage)]
        pub unsafe fn allowsNaturalLanguage(&self) -> bool;
    }
);
