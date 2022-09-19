use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSNotification::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSDistributedNotificationCenterType = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSDistributedNotificationCenter;
    unsafe impl ClassType for NSDistributedNotificationCenter {
        type Super = NSNotificationCenter;
    }
);
impl NSDistributedNotificationCenter {
    pub unsafe fn notificationCenterForType(
        notificationCenterType: &NSDistributedNotificationCenterType,
    ) -> Id<NSDistributedNotificationCenter, Shared> {
        msg_send_id![
            Self::class(),
            notificationCenterForType: notificationCenterType
        ]
    }
    pub unsafe fn defaultCenter() -> Id<NSDistributedNotificationCenter, Shared> {
        msg_send_id![Self::class(), defaultCenter]
    }
    pub unsafe fn addObserver_selector_name_object_suspensionBehavior(
        &self,
        observer: &Object,
        selector: Sel,
        name: Option<&NSNotificationName>,
        object: Option<&NSString>,
        suspensionBehavior: NSNotificationSuspensionBehavior,
    ) {
        msg_send![
            self,
            addObserver: observer,
            selector: selector,
            name: name,
            object: object,
            suspensionBehavior: suspensionBehavior
        ]
    }
    pub unsafe fn postNotificationName_object_userInfo_deliverImmediately(
        &self,
        name: &NSNotificationName,
        object: Option<&NSString>,
        userInfo: Option<&NSDictionary>,
        deliverImmediately: bool,
    ) {
        msg_send![
            self,
            postNotificationName: name,
            object: object,
            userInfo: userInfo,
            deliverImmediately: deliverImmediately
        ]
    }
    pub unsafe fn postNotificationName_object_userInfo_options(
        &self,
        name: &NSNotificationName,
        object: Option<&NSString>,
        userInfo: Option<&NSDictionary>,
        options: NSDistributedNotificationOptions,
    ) {
        msg_send![
            self,
            postNotificationName: name,
            object: object,
            userInfo: userInfo,
            options: options
        ]
    }
    pub unsafe fn addObserver_selector_name_object(
        &self,
        observer: &Object,
        aSelector: Sel,
        aName: Option<&NSNotificationName>,
        anObject: Option<&NSString>,
    ) {
        msg_send![
            self,
            addObserver: observer,
            selector: aSelector,
            name: aName,
            object: anObject
        ]
    }
    pub unsafe fn postNotificationName_object(
        &self,
        aName: &NSNotificationName,
        anObject: Option<&NSString>,
    ) {
        msg_send![self, postNotificationName: aName, object: anObject]
    }
    pub unsafe fn postNotificationName_object_userInfo(
        &self,
        aName: &NSNotificationName,
        anObject: Option<&NSString>,
        aUserInfo: Option<&NSDictionary>,
    ) {
        msg_send![
            self,
            postNotificationName: aName,
            object: anObject,
            userInfo: aUserInfo
        ]
    }
    pub unsafe fn removeObserver_name_object(
        &self,
        observer: &Object,
        aName: Option<&NSNotificationName>,
        anObject: Option<&NSString>,
    ) {
        msg_send![
            self,
            removeObserver: observer,
            name: aName,
            object: anObject
        ]
    }
    pub unsafe fn suspended(&self) -> bool {
        msg_send![self, suspended]
    }
    pub unsafe fn setSuspended(&self, suspended: bool) {
        msg_send![self, setSuspended: suspended]
    }
}
