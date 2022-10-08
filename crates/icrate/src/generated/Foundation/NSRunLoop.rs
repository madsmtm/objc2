use super::__exported::NSArray;
use super::__exported::NSPort;
use super::__exported::NSString;
use super::__exported::NSTimer;
use crate::CoreFoundation::generated::CFRunLoop::*;
use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSRunLoop;
    unsafe impl ClassType for NSRunLoop {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSRunLoop {
        #[method_id(currentRunLoop)]
        pub unsafe fn currentRunLoop() -> Id<NSRunLoop, Shared>;
        #[method_id(mainRunLoop)]
        pub unsafe fn mainRunLoop() -> Id<NSRunLoop, Shared>;
        #[method_id(currentMode)]
        pub unsafe fn currentMode(&self) -> Option<Id<NSRunLoopMode, Shared>>;
        #[method(getCFRunLoop)]
        pub unsafe fn getCFRunLoop(&self) -> CFRunLoopRef;
        # [method (addTimer : forMode :)]
        pub unsafe fn addTimer_forMode(&self, timer: &NSTimer, mode: &NSRunLoopMode);
        # [method (addPort : forMode :)]
        pub unsafe fn addPort_forMode(&self, aPort: &NSPort, mode: &NSRunLoopMode);
        # [method (removePort : forMode :)]
        pub unsafe fn removePort_forMode(&self, aPort: &NSPort, mode: &NSRunLoopMode);
        # [method_id (limitDateForMode :)]
        pub unsafe fn limitDateForMode(&self, mode: &NSRunLoopMode) -> Option<Id<NSDate, Shared>>;
        # [method (acceptInputForMode : beforeDate :)]
        pub unsafe fn acceptInputForMode_beforeDate(
            &self,
            mode: &NSRunLoopMode,
            limitDate: &NSDate,
        );
    }
);
extern_methods!(
    #[doc = "NSRunLoopConveniences"]
    unsafe impl NSRunLoop {
        #[method(run)]
        pub unsafe fn run(&self);
        # [method (runUntilDate :)]
        pub unsafe fn runUntilDate(&self, limitDate: &NSDate);
        # [method (runMode : beforeDate :)]
        pub unsafe fn runMode_beforeDate(&self, mode: &NSRunLoopMode, limitDate: &NSDate) -> bool;
        #[method(configureAsServer)]
        pub unsafe fn configureAsServer(&self);
        # [method (performInModes : block :)]
        pub unsafe fn performInModes_block(&self, modes: &NSArray<NSRunLoopMode>, block: TodoBlock);
        # [method (performBlock :)]
        pub unsafe fn performBlock(&self, block: TodoBlock);
    }
);
extern_methods!(
    #[doc = "NSDelayedPerforming"]
    unsafe impl NSObject {
        # [method (performSelector : withObject : afterDelay : inModes :)]
        pub unsafe fn performSelector_withObject_afterDelay_inModes(
            &self,
            aSelector: Sel,
            anArgument: Option<&Object>,
            delay: NSTimeInterval,
            modes: &NSArray<NSRunLoopMode>,
        );
        # [method (performSelector : withObject : afterDelay :)]
        pub unsafe fn performSelector_withObject_afterDelay(
            &self,
            aSelector: Sel,
            anArgument: Option<&Object>,
            delay: NSTimeInterval,
        );
        # [method (cancelPreviousPerformRequestsWithTarget : selector : object :)]
        pub unsafe fn cancelPreviousPerformRequestsWithTarget_selector_object(
            aTarget: &Object,
            aSelector: Sel,
            anArgument: Option<&Object>,
        );
        # [method (cancelPreviousPerformRequestsWithTarget :)]
        pub unsafe fn cancelPreviousPerformRequestsWithTarget(aTarget: &Object);
    }
);
extern_methods!(
    #[doc = "NSOrderedPerform"]
    unsafe impl NSRunLoop {
        # [method (performSelector : target : argument : order : modes :)]
        pub unsafe fn performSelector_target_argument_order_modes(
            &self,
            aSelector: Sel,
            target: &Object,
            arg: Option<&Object>,
            order: NSUInteger,
            modes: &NSArray<NSRunLoopMode>,
        );
        # [method (cancelPerformSelector : target : argument :)]
        pub unsafe fn cancelPerformSelector_target_argument(
            &self,
            aSelector: Sel,
            target: &Object,
            arg: Option<&Object>,
        );
        # [method (cancelPerformSelectorsWithTarget :)]
        pub unsafe fn cancelPerformSelectorsWithTarget(&self, target: &Object);
    }
);
