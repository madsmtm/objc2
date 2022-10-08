use crate::Foundation::generated::NSCoder::*;
use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDateInterval;
    unsafe impl ClassType for NSDateInterval {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSDateInterval {
        pub unsafe fn startDate(&self) -> Id<NSDate, Shared> {
            msg_send_id![self, startDate]
        }
        pub unsafe fn endDate(&self) -> Id<NSDate, Shared> {
            msg_send_id![self, endDate]
        }
        pub unsafe fn duration(&self) -> NSTimeInterval {
            msg_send![self, duration]
        }
        pub unsafe fn init(&self) -> Id<Self, Shared> {
            msg_send_id![self, init]
        }
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared> {
            msg_send_id![self, initWithCoder: coder]
        }
        pub unsafe fn initWithStartDate_duration(
            &self,
            startDate: &NSDate,
            duration: NSTimeInterval,
        ) -> Id<Self, Shared> {
            msg_send_id![self, initWithStartDate: startDate, duration: duration]
        }
        pub unsafe fn initWithStartDate_endDate(
            &self,
            startDate: &NSDate,
            endDate: &NSDate,
        ) -> Id<Self, Shared> {
            msg_send_id![self, initWithStartDate: startDate, endDate: endDate]
        }
        pub unsafe fn compare(&self, dateInterval: &NSDateInterval) -> NSComparisonResult {
            msg_send![self, compare: dateInterval]
        }
        pub unsafe fn isEqualToDateInterval(&self, dateInterval: &NSDateInterval) -> bool {
            msg_send![self, isEqualToDateInterval: dateInterval]
        }
        pub unsafe fn intersectsDateInterval(&self, dateInterval: &NSDateInterval) -> bool {
            msg_send![self, intersectsDateInterval: dateInterval]
        }
        pub unsafe fn intersectionWithDateInterval(
            &self,
            dateInterval: &NSDateInterval,
        ) -> Option<Id<NSDateInterval, Shared>> {
            msg_send_id![self, intersectionWithDateInterval: dateInterval]
        }
        pub unsafe fn containsDate(&self, date: &NSDate) -> bool {
            msg_send![self, containsDate: date]
        }
    }
);
