//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

pub type NSPostingStyle = NSUInteger;
pub const NSPostWhenIdle: NSPostingStyle = 1;
pub const NSPostASAP: NSPostingStyle = 2;
pub const NSPostNow: NSPostingStyle = 3;

pub type NSNotificationCoalescing = NSUInteger;
pub const NSNotificationNoCoalescing: NSNotificationCoalescing = 0;
pub const NSNotificationCoalescingOnName: NSNotificationCoalescing = 1;
pub const NSNotificationCoalescingOnSender: NSNotificationCoalescing = 2;

extern_class!(
    #[derive(Debug)]
    pub struct NSNotificationQueue;

    unsafe impl ClassType for NSNotificationQueue {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSNotificationQueue {
        #[method_id(defaultQueue)]
        pub unsafe fn defaultQueue() -> Id<NSNotificationQueue, Shared>;

        #[method_id(initWithNotificationCenter:)]
        pub unsafe fn initWithNotificationCenter(
            &self,
            notificationCenter: &NSNotificationCenter,
        ) -> Id<Self, Shared>;

        #[method(enqueueNotification:postingStyle:)]
        pub unsafe fn enqueueNotification_postingStyle(
            &self,
            notification: &NSNotification,
            postingStyle: NSPostingStyle,
        );

        #[method(enqueueNotification:postingStyle:coalesceMask:forModes:)]
        pub unsafe fn enqueueNotification_postingStyle_coalesceMask_forModes(
            &self,
            notification: &NSNotification,
            postingStyle: NSPostingStyle,
            coalesceMask: NSNotificationCoalescing,
            modes: Option<&NSArray<NSRunLoopMode>>,
        );

        #[method(dequeueNotificationsMatching:coalesceMask:)]
        pub unsafe fn dequeueNotificationsMatching_coalesceMask(
            &self,
            notification: &NSNotification,
            coalesceMask: NSUInteger,
        );
    }
);
