use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSNotificationName = NSString;
use super::__exported::NSDictionary;
use super::__exported::NSOperationQueue;
use super::__exported::NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSNotification;
    unsafe impl ClassType for NSNotification {
        type Super = NSObject;
    }
);
impl NSNotification {
    pub unsafe fn initWithName_object_userInfo(
        &self,
        name: &NSNotificationName,
        object: Option<&Object>,
        userInfo: Option<&NSDictionary>,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithName: name, object: object, userInfo: userInfo]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
    pub unsafe fn name(&self) -> Id<NSNotificationName, Shared> {
        msg_send_id![self, name]
    }
    pub unsafe fn object(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, object]
    }
    pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>> {
        msg_send_id![self, userInfo]
    }
}
#[doc = "NSNotificationCreation"]
impl NSNotification {
    pub unsafe fn notificationWithName_object(
        aName: &NSNotificationName,
        anObject: Option<&Object>,
    ) -> Id<Self, Shared> {
        msg_send_id![Self::class(), notificationWithName: aName, object: anObject]
    }
    pub unsafe fn notificationWithName_object_userInfo(
        aName: &NSNotificationName,
        anObject: Option<&Object>,
        aUserInfo: Option<&NSDictionary>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            notificationWithName: aName,
            object: anObject,
            userInfo: aUserInfo
        ]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSNotificationCenter;
    unsafe impl ClassType for NSNotificationCenter {
        type Super = NSObject;
    }
);
impl NSNotificationCenter {
    pub unsafe fn addObserver_selector_name_object(
        &self,
        observer: &Object,
        aSelector: Sel,
        aName: Option<&NSNotificationName>,
        anObject: Option<&Object>,
    ) {
        msg_send![
            self,
            addObserver: observer,
            selector: aSelector,
            name: aName,
            object: anObject
        ]
    }
    pub unsafe fn postNotification(&self, notification: &NSNotification) {
        msg_send![self, postNotification: notification]
    }
    pub unsafe fn postNotificationName_object(
        &self,
        aName: &NSNotificationName,
        anObject: Option<&Object>,
    ) {
        msg_send![self, postNotificationName: aName, object: anObject]
    }
    pub unsafe fn postNotificationName_object_userInfo(
        &self,
        aName: &NSNotificationName,
        anObject: Option<&Object>,
        aUserInfo: Option<&NSDictionary>,
    ) {
        msg_send![
            self,
            postNotificationName: aName,
            object: anObject,
            userInfo: aUserInfo
        ]
    }
    pub unsafe fn removeObserver(&self, observer: &Object) {
        msg_send![self, removeObserver: observer]
    }
    pub unsafe fn removeObserver_name_object(
        &self,
        observer: &Object,
        aName: Option<&NSNotificationName>,
        anObject: Option<&Object>,
    ) {
        msg_send![
            self,
            removeObserver: observer,
            name: aName,
            object: anObject
        ]
    }
    pub unsafe fn addObserverForName_object_queue_usingBlock(
        &self,
        name: Option<&NSNotificationName>,
        obj: Option<&Object>,
        queue: Option<&NSOperationQueue>,
        block: TodoBlock,
    ) -> TodoGenerics {
        msg_send![
            self,
            addObserverForName: name,
            object: obj,
            queue: queue,
            usingBlock: block
        ]
    }
    pub unsafe fn defaultCenter() -> Id<NSNotificationCenter, Shared> {
        msg_send_id![Self::class(), defaultCenter]
    }
}
