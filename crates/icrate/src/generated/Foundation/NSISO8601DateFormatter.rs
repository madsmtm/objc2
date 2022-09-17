#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSISO8601DateFormatter;
    unsafe impl ClassType for NSISO8601DateFormatter {
        type Super = NSFormatter;
    }
);
impl NSISO8601DateFormatter {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn stringFromDate(&self, date: &NSDate) -> Id<NSString, Shared> {
        msg_send_id![self, stringFromDate: date]
    }
    pub unsafe fn dateFromString(&self, string: &NSString) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, dateFromString: string]
    }
    pub unsafe fn stringFromDate_timeZone_formatOptions(
        date: &NSDate,
        timeZone: &NSTimeZone,
        formatOptions: NSISO8601DateFormatOptions,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            Self::class(),
            stringFromDate: date,
            timeZone: timeZone,
            formatOptions: formatOptions
        ]
    }
    pub unsafe fn timeZone(&self) -> Id<NSTimeZone, Shared> {
        msg_send_id![self, timeZone]
    }
    pub unsafe fn setTimeZone(&self, timeZone: Option<&NSTimeZone>) {
        msg_send![self, setTimeZone: timeZone]
    }
    pub unsafe fn formatOptions(&self) -> NSISO8601DateFormatOptions {
        msg_send![self, formatOptions]
    }
    pub unsafe fn setFormatOptions(&self, formatOptions: NSISO8601DateFormatOptions) {
        msg_send![self, setFormatOptions: formatOptions]
    }
}
