#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDate;
    unsafe impl ClassType for NSDate {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSDate {
        #[method(timeIntervalSinceReferenceDate)]
        pub unsafe fn timeIntervalSinceReferenceDate(&self) -> NSTimeInterval;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithTimeIntervalSinceReferenceDate:)]
        pub unsafe fn initWithTimeIntervalSinceReferenceDate(
            &self,
            ti: NSTimeInterval,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSExtendedDate"]
    unsafe impl NSDate {
        #[method(timeIntervalSinceDate:)]
        pub unsafe fn timeIntervalSinceDate(&self, anotherDate: &NSDate) -> NSTimeInterval;
        #[method(timeIntervalSinceNow)]
        pub unsafe fn timeIntervalSinceNow(&self) -> NSTimeInterval;
        #[method(timeIntervalSince1970)]
        pub unsafe fn timeIntervalSince1970(&self) -> NSTimeInterval;
        #[method_id(addTimeInterval:)]
        pub unsafe fn addTimeInterval(&self, seconds: NSTimeInterval) -> Id<Object, Shared>;
        #[method_id(dateByAddingTimeInterval:)]
        pub unsafe fn dateByAddingTimeInterval(&self, ti: NSTimeInterval) -> Id<Self, Shared>;
        #[method_id(earlierDate:)]
        pub unsafe fn earlierDate(&self, anotherDate: &NSDate) -> Id<NSDate, Shared>;
        #[method_id(laterDate:)]
        pub unsafe fn laterDate(&self, anotherDate: &NSDate) -> Id<NSDate, Shared>;
        #[method(compare:)]
        pub unsafe fn compare(&self, other: &NSDate) -> NSComparisonResult;
        #[method(isEqualToDate:)]
        pub unsafe fn isEqualToDate(&self, otherDate: &NSDate) -> bool;
        #[method_id(description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;
        #[method_id(descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>)
            -> Id<NSString, Shared>;
        #[method(timeIntervalSinceReferenceDate)]
        pub unsafe fn timeIntervalSinceReferenceDate() -> NSTimeInterval;
    }
);
extern_methods!(
    #[doc = "NSDateCreation"]
    unsafe impl NSDate {
        #[method_id(date)]
        pub unsafe fn date() -> Id<Self, Shared>;
        #[method_id(dateWithTimeIntervalSinceNow:)]
        pub unsafe fn dateWithTimeIntervalSinceNow(secs: NSTimeInterval) -> Id<Self, Shared>;
        #[method_id(dateWithTimeIntervalSinceReferenceDate:)]
        pub unsafe fn dateWithTimeIntervalSinceReferenceDate(
            ti: NSTimeInterval,
        ) -> Id<Self, Shared>;
        #[method_id(dateWithTimeIntervalSince1970:)]
        pub unsafe fn dateWithTimeIntervalSince1970(secs: NSTimeInterval) -> Id<Self, Shared>;
        #[method_id(dateWithTimeInterval:sinceDate:)]
        pub unsafe fn dateWithTimeInterval_sinceDate(
            secsToBeAdded: NSTimeInterval,
            date: &NSDate,
        ) -> Id<Self, Shared>;
        #[method_id(distantFuture)]
        pub unsafe fn distantFuture() -> Id<NSDate, Shared>;
        #[method_id(distantPast)]
        pub unsafe fn distantPast() -> Id<NSDate, Shared>;
        #[method_id(now)]
        pub unsafe fn now() -> Id<NSDate, Shared>;
        #[method_id(initWithTimeIntervalSinceNow:)]
        pub unsafe fn initWithTimeIntervalSinceNow(&self, secs: NSTimeInterval)
            -> Id<Self, Shared>;
        #[method_id(initWithTimeIntervalSince1970:)]
        pub unsafe fn initWithTimeIntervalSince1970(
            &self,
            secs: NSTimeInterval,
        ) -> Id<Self, Shared>;
        #[method_id(initWithTimeInterval:sinceDate:)]
        pub unsafe fn initWithTimeInterval_sinceDate(
            &self,
            secsToBeAdded: NSTimeInterval,
            date: &NSDate,
        ) -> Id<Self, Shared>;
    }
);
