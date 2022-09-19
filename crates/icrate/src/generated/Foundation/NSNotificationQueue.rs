use super::__exported::NSArray;
use super::__exported::NSNotification;
use super::__exported::NSNotificationCenter;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRunLoop::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSNotificationQueue;
    unsafe impl ClassType for NSNotificationQueue {
        type Super = NSObject;
    }
);
impl NSNotificationQueue {
    pub unsafe fn initWithNotificationCenter(
        &self,
        notificationCenter: &NSNotificationCenter,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithNotificationCenter: notificationCenter]
    }
    pub unsafe fn enqueueNotification_postingStyle(
        &self,
        notification: &NSNotification,
        postingStyle: NSPostingStyle,
    ) {
        msg_send![
            self,
            enqueueNotification: notification,
            postingStyle: postingStyle
        ]
    }
    pub unsafe fn enqueueNotification_postingStyle_coalesceMask_forModes(
        &self,
        notification: &NSNotification,
        postingStyle: NSPostingStyle,
        coalesceMask: NSNotificationCoalescing,
        modes: Option<&NSArray<NSRunLoopMode>>,
    ) {
        msg_send![
            self,
            enqueueNotification: notification,
            postingStyle: postingStyle,
            coalesceMask: coalesceMask,
            forModes: modes
        ]
    }
    pub unsafe fn dequeueNotificationsMatching_coalesceMask(
        &self,
        notification: &NSNotification,
        coalesceMask: NSUInteger,
    ) {
        msg_send![
            self,
            dequeueNotificationsMatching: notification,
            coalesceMask: coalesceMask
        ]
    }
    pub unsafe fn defaultQueue() -> Id<NSNotificationQueue, Shared> {
        msg_send_id![Self::class(), defaultQueue]
    }
}
