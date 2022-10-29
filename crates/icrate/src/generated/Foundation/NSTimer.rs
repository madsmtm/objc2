#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTimer;
    unsafe impl ClassType for NSTimer {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTimer {
        #[method_id(timerWithTimeInterval:invocation:repeats:)]
        pub unsafe fn timerWithTimeInterval_invocation_repeats(
            ti: NSTimeInterval,
            invocation: &NSInvocation,
            yesOrNo: bool,
        ) -> Id<NSTimer, Shared>;
        #[method_id(scheduledTimerWithTimeInterval:invocation:repeats:)]
        pub unsafe fn scheduledTimerWithTimeInterval_invocation_repeats(
            ti: NSTimeInterval,
            invocation: &NSInvocation,
            yesOrNo: bool,
        ) -> Id<NSTimer, Shared>;
        #[method_id(timerWithTimeInterval:target:selector:userInfo:repeats:)]
        pub unsafe fn timerWithTimeInterval_target_selector_userInfo_repeats(
            ti: NSTimeInterval,
            aTarget: &Object,
            aSelector: Sel,
            userInfo: Option<&Object>,
            yesOrNo: bool,
        ) -> Id<NSTimer, Shared>;
        #[method_id(scheduledTimerWithTimeInterval:target:selector:userInfo:repeats:)]
        pub unsafe fn scheduledTimerWithTimeInterval_target_selector_userInfo_repeats(
            ti: NSTimeInterval,
            aTarget: &Object,
            aSelector: Sel,
            userInfo: Option<&Object>,
            yesOrNo: bool,
        ) -> Id<NSTimer, Shared>;
        #[method_id(timerWithTimeInterval:repeats:block:)]
        pub unsafe fn timerWithTimeInterval_repeats_block(
            interval: NSTimeInterval,
            repeats: bool,
            block: TodoBlock,
        ) -> Id<NSTimer, Shared>;
        #[method_id(scheduledTimerWithTimeInterval:repeats:block:)]
        pub unsafe fn scheduledTimerWithTimeInterval_repeats_block(
            interval: NSTimeInterval,
            repeats: bool,
            block: TodoBlock,
        ) -> Id<NSTimer, Shared>;
        #[method_id(initWithFireDate:interval:repeats:block:)]
        pub unsafe fn initWithFireDate_interval_repeats_block(
            &self,
            date: &NSDate,
            interval: NSTimeInterval,
            repeats: bool,
            block: TodoBlock,
        ) -> Id<Self, Shared>;
        #[method_id(initWithFireDate:interval:target:selector:userInfo:repeats:)]
        pub unsafe fn initWithFireDate_interval_target_selector_userInfo_repeats(
            &self,
            date: &NSDate,
            ti: NSTimeInterval,
            t: &Object,
            s: Sel,
            ui: Option<&Object>,
            rep: bool,
        ) -> Id<Self, Shared>;
        #[method(fire)]
        pub unsafe fn fire(&self);
        #[method_id(fireDate)]
        pub unsafe fn fireDate(&self) -> Id<NSDate, Shared>;
        #[method(setFireDate:)]
        pub unsafe fn setFireDate(&self, fireDate: &NSDate);
        #[method(timeInterval)]
        pub unsafe fn timeInterval(&self) -> NSTimeInterval;
        #[method(tolerance)]
        pub unsafe fn tolerance(&self) -> NSTimeInterval;
        #[method(setTolerance:)]
        pub unsafe fn setTolerance(&self, tolerance: NSTimeInterval);
        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
        #[method(isValid)]
        pub unsafe fn isValid(&self) -> bool;
        #[method_id(userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<Object, Shared>>;
    }
);
