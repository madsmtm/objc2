use super::__exported::NSDate;
use super::__exported::NSString;
use super::__exported::NSTimeZone;
use crate::CoreFoundation::generated::CFDateFormatter::*;
use crate::Foundation::generated::NSFormatter::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
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
        # [method (setTimeZone :)]
        pub unsafe fn setTimeZone(&self, timeZone: Option<&NSTimeZone>);
        #[method(formatOptions)]
        pub unsafe fn formatOptions(&self) -> NSISO8601DateFormatOptions;
        # [method (setFormatOptions :)]
        pub unsafe fn setFormatOptions(&self, formatOptions: NSISO8601DateFormatOptions);
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        # [method_id (stringFromDate :)]
        pub unsafe fn stringFromDate(&self, date: &NSDate) -> Id<NSString, Shared>;
        # [method_id (dateFromString :)]
        pub unsafe fn dateFromString(&self, string: &NSString) -> Option<Id<NSDate, Shared>>;
        # [method_id (stringFromDate : timeZone : formatOptions :)]
        pub unsafe fn stringFromDate_timeZone_formatOptions(
            date: &NSDate,
            timeZone: &NSTimeZone,
            formatOptions: NSISO8601DateFormatOptions,
        ) -> Id<NSString, Shared>;
    }
);
