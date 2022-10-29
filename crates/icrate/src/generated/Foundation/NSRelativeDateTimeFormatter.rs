#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSRelativeDateTimeFormatter;
    unsafe impl ClassType for NSRelativeDateTimeFormatter {
        type Super = NSFormatter;
    }
);
extern_methods!(
    unsafe impl NSRelativeDateTimeFormatter {
        #[method(dateTimeStyle)]
        pub unsafe fn dateTimeStyle(&self) -> NSRelativeDateTimeFormatterStyle;
        #[method(setDateTimeStyle:)]
        pub unsafe fn setDateTimeStyle(&self, dateTimeStyle: NSRelativeDateTimeFormatterStyle);
        #[method(unitsStyle)]
        pub unsafe fn unitsStyle(&self) -> NSRelativeDateTimeFormatterUnitsStyle;
        #[method(setUnitsStyle:)]
        pub unsafe fn setUnitsStyle(&self, unitsStyle: NSRelativeDateTimeFormatterUnitsStyle);
        #[method(formattingContext)]
        pub unsafe fn formattingContext(&self) -> NSFormattingContext;
        #[method(setFormattingContext:)]
        pub unsafe fn setFormattingContext(&self, formattingContext: NSFormattingContext);
        #[method_id(calendar)]
        pub unsafe fn calendar(&self) -> Id<NSCalendar, Shared>;
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);
        #[method_id(locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale, Shared>;
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);
        #[method_id(localizedStringFromDateComponents:)]
        pub unsafe fn localizedStringFromDateComponents(
            &self,
            dateComponents: &NSDateComponents,
        ) -> Id<NSString, Shared>;
        #[method_id(localizedStringFromTimeInterval:)]
        pub unsafe fn localizedStringFromTimeInterval(
            &self,
            timeInterval: NSTimeInterval,
        ) -> Id<NSString, Shared>;
        #[method_id(localizedStringForDate:relativeToDate:)]
        pub unsafe fn localizedStringForDate_relativeToDate(
            &self,
            date: &NSDate,
            referenceDate: &NSDate,
        ) -> Id<NSString, Shared>;
        #[method_id(stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(
            &self,
            obj: Option<&Object>,
        ) -> Option<Id<NSString, Shared>>;
    }
);
