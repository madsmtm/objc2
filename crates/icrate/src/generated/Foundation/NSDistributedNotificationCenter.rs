use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSNotification::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSDistributedNotificationCenterType = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSDistributedNotificationCenter;
    unsafe impl ClassType for NSDistributedNotificationCenter {
        type Super = NSNotificationCenter;
    }
);
extern_methods!(
    unsafe impl NSDistributedNotificationCenter {
        # [method_id (notificationCenterForType :)]
        pub unsafe fn notificationCenterForType(
            notificationCenterType: &NSDistributedNotificationCenterType,
        ) -> Id<NSDistributedNotificationCenter, Shared>;
        #[method_id(defaultCenter)]
        pub unsafe fn defaultCenter() -> Id<NSDistributedNotificationCenter, Shared>;
        # [method (addObserver : selector : name : object : suspensionBehavior :)]
        pub unsafe fn addObserver_selector_name_object_suspensionBehavior(
            &self,
            observer: &Object,
            selector: Sel,
            name: Option<&NSNotificationName>,
            object: Option<&NSString>,
            suspensionBehavior: NSNotificationSuspensionBehavior,
        );
        # [method (postNotificationName : object : userInfo : deliverImmediately :)]
        pub unsafe fn postNotificationName_object_userInfo_deliverImmediately(
            &self,
            name: &NSNotificationName,
            object: Option<&NSString>,
            userInfo: Option<&NSDictionary>,
            deliverImmediately: bool,
        );
        # [method (postNotificationName : object : userInfo : options :)]
        pub unsafe fn postNotificationName_object_userInfo_options(
            &self,
            name: &NSNotificationName,
            object: Option<&NSString>,
            userInfo: Option<&NSDictionary>,
            options: NSDistributedNotificationOptions,
        );
        #[method(suspended)]
        pub unsafe fn suspended(&self) -> bool;
        # [method (setSuspended :)]
        pub unsafe fn setSuspended(&self, suspended: bool);
        # [method (addObserver : selector : name : object :)]
        pub unsafe fn addObserver_selector_name_object(
            &self,
            observer: &Object,
            aSelector: Sel,
            aName: Option<&NSNotificationName>,
            anObject: Option<&NSString>,
        );
        # [method (postNotificationName : object :)]
        pub unsafe fn postNotificationName_object(
            &self,
            aName: &NSNotificationName,
            anObject: Option<&NSString>,
        );
        # [method (postNotificationName : object : userInfo :)]
        pub unsafe fn postNotificationName_object_userInfo(
            &self,
            aName: &NSNotificationName,
            anObject: Option<&NSString>,
            aUserInfo: Option<&NSDictionary>,
        );
        # [method (removeObserver : name : object :)]
        pub unsafe fn removeObserver_name_object(
            &self,
            observer: &Object,
            aName: Option<&NSNotificationName>,
            anObject: Option<&NSString>,
        );
    }
);
