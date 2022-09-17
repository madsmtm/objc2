use super::NSString;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDate;
    unsafe impl ClassType for NSDate {
        type Super = NSObject;
    }
);
impl NSDate {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithTimeIntervalSinceReferenceDate(
        &self,
        ti: NSTimeInterval,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithTimeIntervalSinceReferenceDate: ti]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
    pub unsafe fn timeIntervalSinceReferenceDate(&self) -> NSTimeInterval {
        msg_send![self, timeIntervalSinceReferenceDate]
    }
}
#[doc = "NSExtendedDate"]
impl NSDate {
    pub unsafe fn timeIntervalSinceDate(&self, anotherDate: &NSDate) -> NSTimeInterval {
        msg_send![self, timeIntervalSinceDate: anotherDate]
    }
    pub unsafe fn addTimeInterval(&self, seconds: NSTimeInterval) -> Id<Object, Shared> {
        msg_send_id![self, addTimeInterval: seconds]
    }
    pub unsafe fn dateByAddingTimeInterval(&self, ti: NSTimeInterval) -> Id<Self, Shared> {
        msg_send_id![self, dateByAddingTimeInterval: ti]
    }
    pub unsafe fn earlierDate(&self, anotherDate: &NSDate) -> Id<NSDate, Shared> {
        msg_send_id![self, earlierDate: anotherDate]
    }
    pub unsafe fn laterDate(&self, anotherDate: &NSDate) -> Id<NSDate, Shared> {
        msg_send_id![self, laterDate: anotherDate]
    }
    pub unsafe fn compare(&self, other: &NSDate) -> NSComparisonResult {
        msg_send![self, compare: other]
    }
    pub unsafe fn isEqualToDate(&self, otherDate: &NSDate) -> bool {
        msg_send![self, isEqualToDate: otherDate]
    }
    pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>) -> Id<NSString, Shared> {
        msg_send_id![self, descriptionWithLocale: locale]
    }
    pub unsafe fn timeIntervalSinceNow(&self) -> NSTimeInterval {
        msg_send![self, timeIntervalSinceNow]
    }
    pub unsafe fn timeIntervalSince1970(&self) -> NSTimeInterval {
        msg_send![self, timeIntervalSince1970]
    }
    pub unsafe fn description(&self) -> Id<NSString, Shared> {
        msg_send_id![self, description]
    }
    pub unsafe fn timeIntervalSinceReferenceDate() -> NSTimeInterval {
        msg_send![Self::class(), timeIntervalSinceReferenceDate]
    }
}
#[doc = "NSDateCreation"]
impl NSDate {
    pub unsafe fn date() -> Id<Self, Shared> {
        msg_send_id![Self::class(), date]
    }
    pub unsafe fn dateWithTimeIntervalSinceNow(secs: NSTimeInterval) -> Id<Self, Shared> {
        msg_send_id![Self::class(), dateWithTimeIntervalSinceNow: secs]
    }
    pub unsafe fn dateWithTimeIntervalSinceReferenceDate(ti: NSTimeInterval) -> Id<Self, Shared> {
        msg_send_id![Self::class(), dateWithTimeIntervalSinceReferenceDate: ti]
    }
    pub unsafe fn dateWithTimeIntervalSince1970(secs: NSTimeInterval) -> Id<Self, Shared> {
        msg_send_id![Self::class(), dateWithTimeIntervalSince1970: secs]
    }
    pub unsafe fn dateWithTimeInterval_sinceDate(
        secsToBeAdded: NSTimeInterval,
        date: &NSDate,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            dateWithTimeInterval: secsToBeAdded,
            sinceDate: date
        ]
    }
    pub unsafe fn initWithTimeIntervalSinceNow(&self, secs: NSTimeInterval) -> Id<Self, Shared> {
        msg_send_id![self, initWithTimeIntervalSinceNow: secs]
    }
    pub unsafe fn initWithTimeIntervalSince1970(&self, secs: NSTimeInterval) -> Id<Self, Shared> {
        msg_send_id![self, initWithTimeIntervalSince1970: secs]
    }
    pub unsafe fn initWithTimeInterval_sinceDate(
        &self,
        secsToBeAdded: NSTimeInterval,
        date: &NSDate,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithTimeInterval: secsToBeAdded, sinceDate: date]
    }
    pub unsafe fn distantFuture() -> Id<NSDate, Shared> {
        msg_send_id![Self::class(), distantFuture]
    }
    pub unsafe fn distantPast() -> Id<NSDate, Shared> {
        msg_send_id![Self::class(), distantPast]
    }
    pub unsafe fn now() -> Id<NSDate, Shared> {
        msg_send_id![Self::class(), now]
    }
}
