use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSObjCRuntime::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSBackgroundActivityScheduler;
    unsafe impl ClassType for NSBackgroundActivityScheduler {
        type Super = NSObject;
    }
);
impl NSBackgroundActivityScheduler {
    pub unsafe fn initWithIdentifier(&self, identifier: &NSString) -> Id<Self, Shared> {
        msg_send_id![self, initWithIdentifier: identifier]
    }
    pub unsafe fn identifier(&self) -> Id<NSString, Shared> {
        msg_send_id![self, identifier]
    }
    pub unsafe fn qualityOfService(&self) -> NSQualityOfService {
        msg_send![self, qualityOfService]
    }
    pub unsafe fn setQualityOfService(&self, qualityOfService: NSQualityOfService) {
        msg_send![self, setQualityOfService: qualityOfService]
    }
    pub unsafe fn repeats(&self) -> bool {
        msg_send![self, repeats]
    }
    pub unsafe fn setRepeats(&self, repeats: bool) {
        msg_send![self, setRepeats: repeats]
    }
    pub unsafe fn interval(&self) -> NSTimeInterval {
        msg_send![self, interval]
    }
    pub unsafe fn setInterval(&self, interval: NSTimeInterval) {
        msg_send![self, setInterval: interval]
    }
    pub unsafe fn tolerance(&self) -> NSTimeInterval {
        msg_send![self, tolerance]
    }
    pub unsafe fn setTolerance(&self, tolerance: NSTimeInterval) {
        msg_send![self, setTolerance: tolerance]
    }
    pub unsafe fn scheduleWithBlock(&self, block: TodoBlock) {
        msg_send![self, scheduleWithBlock: block]
    }
    pub unsafe fn invalidate(&self) {
        msg_send![self, invalidate]
    }
    pub unsafe fn shouldDefer(&self) -> bool {
        msg_send![self, shouldDefer]
    }
}
