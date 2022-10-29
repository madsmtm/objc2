#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
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
