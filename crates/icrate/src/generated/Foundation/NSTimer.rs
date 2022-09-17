#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSTimer;
    unsafe impl ClassType for NSTimer {
        type Super = NSObject;
    }
);
impl NSTimer {
    pub unsafe fn timerWithTimeInterval_invocation_repeats(
        ti: NSTimeInterval,
        invocation: &NSInvocation,
        yesOrNo: bool,
    ) -> Id<NSTimer, Shared> {
        msg_send_id![
            Self::class(),
            timerWithTimeInterval: ti,
            invocation: invocation,
            repeats: yesOrNo
        ]
    }
    pub unsafe fn scheduledTimerWithTimeInterval_invocation_repeats(
        ti: NSTimeInterval,
        invocation: &NSInvocation,
        yesOrNo: bool,
    ) -> Id<NSTimer, Shared> {
        msg_send_id![
            Self::class(),
            scheduledTimerWithTimeInterval: ti,
            invocation: invocation,
            repeats: yesOrNo
        ]
    }
    pub unsafe fn timerWithTimeInterval_target_selector_userInfo_repeats(
        ti: NSTimeInterval,
        aTarget: &Object,
        aSelector: Sel,
        userInfo: Option<&Object>,
        yesOrNo: bool,
    ) -> Id<NSTimer, Shared> {
        msg_send_id![
            Self::class(),
            timerWithTimeInterval: ti,
            target: aTarget,
            selector: aSelector,
            userInfo: userInfo,
            repeats: yesOrNo
        ]
    }
    pub unsafe fn scheduledTimerWithTimeInterval_target_selector_userInfo_repeats(
        ti: NSTimeInterval,
        aTarget: &Object,
        aSelector: Sel,
        userInfo: Option<&Object>,
        yesOrNo: bool,
    ) -> Id<NSTimer, Shared> {
        msg_send_id![
            Self::class(),
            scheduledTimerWithTimeInterval: ti,
            target: aTarget,
            selector: aSelector,
            userInfo: userInfo,
            repeats: yesOrNo
        ]
    }
    pub unsafe fn timerWithTimeInterval_repeats_block(
        interval: NSTimeInterval,
        repeats: bool,
        block: TodoBlock,
    ) -> Id<NSTimer, Shared> {
        msg_send_id![
            Self::class(),
            timerWithTimeInterval: interval,
            repeats: repeats,
            block: block
        ]
    }
    pub unsafe fn scheduledTimerWithTimeInterval_repeats_block(
        interval: NSTimeInterval,
        repeats: bool,
        block: TodoBlock,
    ) -> Id<NSTimer, Shared> {
        msg_send_id![
            Self::class(),
            scheduledTimerWithTimeInterval: interval,
            repeats: repeats,
            block: block
        ]
    }
    pub unsafe fn initWithFireDate_interval_repeats_block(
        &self,
        date: &NSDate,
        interval: NSTimeInterval,
        repeats: bool,
        block: TodoBlock,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithFireDate: date,
            interval: interval,
            repeats: repeats,
            block: block
        ]
    }
    pub unsafe fn initWithFireDate_interval_target_selector_userInfo_repeats(
        &self,
        date: &NSDate,
        ti: NSTimeInterval,
        t: &Object,
        s: Sel,
        ui: Option<&Object>,
        rep: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithFireDate: date,
            interval: ti,
            target: t,
            selector: s,
            userInfo: ui,
            repeats: rep
        ]
    }
    pub unsafe fn fire(&self) {
        msg_send![self, fire]
    }
    pub unsafe fn invalidate(&self) {
        msg_send![self, invalidate]
    }
    pub unsafe fn fireDate(&self) -> Id<NSDate, Shared> {
        msg_send_id![self, fireDate]
    }
    pub unsafe fn setFireDate(&self, fireDate: &NSDate) {
        msg_send![self, setFireDate: fireDate]
    }
    pub unsafe fn timeInterval(&self) -> NSTimeInterval {
        msg_send![self, timeInterval]
    }
    pub unsafe fn tolerance(&self) -> NSTimeInterval {
        msg_send![self, tolerance]
    }
    pub unsafe fn setTolerance(&self, tolerance: NSTimeInterval) {
        msg_send![self, setTolerance: tolerance]
    }
    pub unsafe fn isValid(&self) -> bool {
        msg_send![self, isValid]
    }
    pub unsafe fn userInfo(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, userInfo]
    }
}
