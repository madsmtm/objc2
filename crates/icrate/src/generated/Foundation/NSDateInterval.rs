#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDateInterval;
    unsafe impl ClassType for NSDateInterval {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSDateInterval {
        #[method_id(startDate)]
        pub unsafe fn startDate(&self) -> Id<NSDate, Shared>;
        #[method_id(endDate)]
        pub unsafe fn endDate(&self) -> Id<NSDate, Shared>;
        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(initWithStartDate:duration:)]
        pub unsafe fn initWithStartDate_duration(
            &self,
            startDate: &NSDate,
            duration: NSTimeInterval,
        ) -> Id<Self, Shared>;
        #[method_id(initWithStartDate:endDate:)]
        pub unsafe fn initWithStartDate_endDate(
            &self,
            startDate: &NSDate,
            endDate: &NSDate,
        ) -> Id<Self, Shared>;
        #[method(compare:)]
        pub unsafe fn compare(&self, dateInterval: &NSDateInterval) -> NSComparisonResult;
        #[method(isEqualToDateInterval:)]
        pub unsafe fn isEqualToDateInterval(&self, dateInterval: &NSDateInterval) -> bool;
        #[method(intersectsDateInterval:)]
        pub unsafe fn intersectsDateInterval(&self, dateInterval: &NSDateInterval) -> bool;
        #[method_id(intersectionWithDateInterval:)]
        pub unsafe fn intersectionWithDateInterval(
            &self,
            dateInterval: &NSDateInterval,
        ) -> Option<Id<NSDateInterval, Shared>>;
        #[method(containsDate:)]
        pub unsafe fn containsDate(&self, date: &NSDate) -> bool;
    }
);
