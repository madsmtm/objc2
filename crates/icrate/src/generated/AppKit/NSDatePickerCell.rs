#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDatePickerCell;
    unsafe impl ClassType for NSDatePickerCell {
        type Super = NSActionCell;
    }
);
extern_methods!(
    unsafe impl NSDatePickerCell {
        #[method_id(initTextCell:)]
        pub unsafe fn initTextCell(&self, string: &NSString) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(initImageCell:)]
        pub unsafe fn initImageCell(&self, image: Option<&NSImage>) -> Id<Self, Shared>;
        #[method(datePickerStyle)]
        pub unsafe fn datePickerStyle(&self) -> NSDatePickerStyle;
        #[method(setDatePickerStyle:)]
        pub unsafe fn setDatePickerStyle(&self, datePickerStyle: NSDatePickerStyle);
        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;
        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);
        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: &NSColor);
        #[method_id(textColor)]
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
        #[method_id(calendar)]
        pub unsafe fn calendar(&self) -> Option<Id<NSCalendar, Shared>>;
        #[method(setCalendar:)]
        pub unsafe fn setCalendar(&self, calendar: Option<&NSCalendar>);
        #[method_id(locale)]
        pub unsafe fn locale(&self) -> Option<Id<NSLocale, Shared>>;
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);
        #[method_id(timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Id<NSTimeZone, Shared>>;
        #[method(setTimeZone:)]
        pub unsafe fn setTimeZone(&self, timeZone: Option<&NSTimeZone>);
        #[method_id(dateValue)]
        pub unsafe fn dateValue(&self) -> Id<NSDate, Shared>;
        #[method(setDateValue:)]
        pub unsafe fn setDateValue(&self, dateValue: &NSDate);
        #[method(timeInterval)]
        pub unsafe fn timeInterval(&self) -> NSTimeInterval;
        #[method(setTimeInterval:)]
        pub unsafe fn setTimeInterval(&self, timeInterval: NSTimeInterval);
        #[method_id(minDate)]
        pub unsafe fn minDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method(setMinDate:)]
        pub unsafe fn setMinDate(&self, minDate: Option<&NSDate>);
        #[method_id(maxDate)]
        pub unsafe fn maxDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method(setMaxDate:)]
        pub unsafe fn setMaxDate(&self, maxDate: Option<&NSDate>);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSDatePickerCellDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSDatePickerCellDelegate>);
    }
);
pub type NSDatePickerCellDelegate = NSObject;
