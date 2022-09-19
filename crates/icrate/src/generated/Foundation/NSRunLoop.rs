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
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSRunLoop;
    unsafe impl ClassType for NSRunLoop {
        type Super = NSObject;
    }
);
impl NSRunLoop {
    pub unsafe fn getCFRunLoop(&self) -> CFRunLoopRef {
        msg_send![self, getCFRunLoop]
    }
    pub unsafe fn addTimer_forMode(&self, timer: &NSTimer, mode: &NSRunLoopMode) {
        msg_send![self, addTimer: timer, forMode: mode]
    }
    pub unsafe fn addPort_forMode(&self, aPort: &NSPort, mode: &NSRunLoopMode) {
        msg_send![self, addPort: aPort, forMode: mode]
    }
    pub unsafe fn removePort_forMode(&self, aPort: &NSPort, mode: &NSRunLoopMode) {
        msg_send![self, removePort: aPort, forMode: mode]
    }
    pub unsafe fn limitDateForMode(&self, mode: &NSRunLoopMode) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, limitDateForMode: mode]
    }
    pub unsafe fn acceptInputForMode_beforeDate(&self, mode: &NSRunLoopMode, limitDate: &NSDate) {
        msg_send![self, acceptInputForMode: mode, beforeDate: limitDate]
    }
    pub unsafe fn currentRunLoop() -> Id<NSRunLoop, Shared> {
        msg_send_id![Self::class(), currentRunLoop]
    }
    pub unsafe fn mainRunLoop() -> Id<NSRunLoop, Shared> {
        msg_send_id![Self::class(), mainRunLoop]
    }
    pub unsafe fn currentMode(&self) -> Option<Id<NSRunLoopMode, Shared>> {
        msg_send_id![self, currentMode]
    }
}
#[doc = "NSRunLoopConveniences"]
impl NSRunLoop {
    pub unsafe fn run(&self) {
        msg_send![self, run]
    }
    pub unsafe fn runUntilDate(&self, limitDate: &NSDate) {
        msg_send![self, runUntilDate: limitDate]
    }
    pub unsafe fn runMode_beforeDate(&self, mode: &NSRunLoopMode, limitDate: &NSDate) -> bool {
        msg_send![self, runMode: mode, beforeDate: limitDate]
    }
    pub unsafe fn configureAsServer(&self) {
        msg_send![self, configureAsServer]
    }
    pub unsafe fn performInModes_block(&self, modes: TodoGenerics, block: TodoBlock) {
        msg_send![self, performInModes: modes, block: block]
    }
    pub unsafe fn performBlock(&self, block: TodoBlock) {
        msg_send![self, performBlock: block]
    }
}
#[doc = "NSDelayedPerforming"]
impl NSObject {
    pub unsafe fn performSelector_withObject_afterDelay_inModes(
        &self,
        aSelector: Sel,
        anArgument: Option<&Object>,
        delay: NSTimeInterval,
        modes: TodoGenerics,
    ) {
        msg_send![
            self,
            performSelector: aSelector,
            withObject: anArgument,
            afterDelay: delay,
            inModes: modes
        ]
    }
    pub unsafe fn performSelector_withObject_afterDelay(
        &self,
        aSelector: Sel,
        anArgument: Option<&Object>,
        delay: NSTimeInterval,
    ) {
        msg_send![
            self,
            performSelector: aSelector,
            withObject: anArgument,
            afterDelay: delay
        ]
    }
    pub unsafe fn cancelPreviousPerformRequestsWithTarget_selector_object(
        aTarget: &Object,
        aSelector: Sel,
        anArgument: Option<&Object>,
    ) {
        msg_send![
            Self::class(),
            cancelPreviousPerformRequestsWithTarget: aTarget,
            selector: aSelector,
            object: anArgument
        ]
    }
    pub unsafe fn cancelPreviousPerformRequestsWithTarget(aTarget: &Object) {
        msg_send![
            Self::class(),
            cancelPreviousPerformRequestsWithTarget: aTarget
        ]
    }
}
#[doc = "NSOrderedPerform"]
impl NSRunLoop {
    pub unsafe fn performSelector_target_argument_order_modes(
        &self,
        aSelector: Sel,
        target: &Object,
        arg: Option<&Object>,
        order: NSUInteger,
        modes: TodoGenerics,
    ) {
        msg_send![
            self,
            performSelector: aSelector,
            target: target,
            argument: arg,
            order: order,
            modes: modes
        ]
    }
    pub unsafe fn cancelPerformSelector_target_argument(
        &self,
        aSelector: Sel,
        target: &Object,
        arg: Option<&Object>,
    ) {
        msg_send![
            self,
            cancelPerformSelector: aSelector,
            target: target,
            argument: arg
        ]
    }
    pub unsafe fn cancelPerformSelectorsWithTarget(&self, target: &Object) {
        msg_send![self, cancelPerformSelectorsWithTarget: target]
    }
}
