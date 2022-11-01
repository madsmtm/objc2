//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern "C" {
    pub static NSSystemClockDidChangeNotification: &'static NSNotificationName;
}

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

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithTimeIntervalSinceReferenceDate:)]
        pub unsafe fn initWithTimeIntervalSinceReferenceDate(
            this: Option<Allocated<Self>>,
            ti: NSTimeInterval,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSExtendedDate
    unsafe impl NSDate {
        #[method(timeIntervalSinceDate:)]
        pub unsafe fn timeIntervalSinceDate(&self, anotherDate: &NSDate) -> NSTimeInterval;

        #[method(timeIntervalSinceNow)]
        pub unsafe fn timeIntervalSinceNow(&self) -> NSTimeInterval;

        #[method(timeIntervalSince1970)]
        pub unsafe fn timeIntervalSince1970(&self) -> NSTimeInterval;

        #[method_id(@__retain_semantics Other addTimeInterval:)]
        pub unsafe fn addTimeInterval(&self, seconds: NSTimeInterval) -> Id<Object, Shared>;

        #[method_id(@__retain_semantics Other dateByAddingTimeInterval:)]
        pub unsafe fn dateByAddingTimeInterval(&self, ti: NSTimeInterval) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other earlierDate:)]
        pub unsafe fn earlierDate(&self, anotherDate: &NSDate) -> Id<NSDate, Shared>;

        #[method_id(@__retain_semantics Other laterDate:)]
        pub unsafe fn laterDate(&self, anotherDate: &NSDate) -> Id<NSDate, Shared>;

        #[method(compare:)]
        pub unsafe fn compare(&self, other: &NSDate) -> NSComparisonResult;

        #[method(isEqualToDate:)]
        pub unsafe fn isEqualToDate(&self, otherDate: &NSDate) -> bool;

        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>)
            -> Id<NSString, Shared>;

        #[method(timeIntervalSinceReferenceDate)]
        pub unsafe fn timeIntervalSinceReferenceDate() -> NSTimeInterval;
    }
);

extern_methods!(
    /// NSDateCreation
    unsafe impl NSDate {
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dateWithTimeIntervalSinceNow:)]
        pub unsafe fn dateWithTimeIntervalSinceNow(secs: NSTimeInterval) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dateWithTimeIntervalSinceReferenceDate:)]
        pub unsafe fn dateWithTimeIntervalSinceReferenceDate(
            ti: NSTimeInterval,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dateWithTimeIntervalSince1970:)]
        pub unsafe fn dateWithTimeIntervalSince1970(secs: NSTimeInterval) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other dateWithTimeInterval:sinceDate:)]
        pub unsafe fn dateWithTimeInterval_sinceDate(
            secsToBeAdded: NSTimeInterval,
            date: &NSDate,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other distantFuture)]
        pub unsafe fn distantFuture() -> Id<NSDate, Shared>;

        #[method_id(@__retain_semantics Other distantPast)]
        pub unsafe fn distantPast() -> Id<NSDate, Shared>;

        #[method_id(@__retain_semantics Other now)]
        pub unsafe fn now() -> Id<NSDate, Shared>;

        #[method_id(@__retain_semantics Init initWithTimeIntervalSinceNow:)]
        pub unsafe fn initWithTimeIntervalSinceNow(
            this: Option<Allocated<Self>>,
            secs: NSTimeInterval,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithTimeIntervalSince1970:)]
        pub unsafe fn initWithTimeIntervalSince1970(
            this: Option<Allocated<Self>>,
            secs: NSTimeInterval,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithTimeInterval:sinceDate:)]
        pub unsafe fn initWithTimeInterval_sinceDate(
            this: Option<Allocated<Self>>,
            secsToBeAdded: NSTimeInterval,
            date: &NSDate,
        ) -> Id<Self, Shared>;
    }
);
