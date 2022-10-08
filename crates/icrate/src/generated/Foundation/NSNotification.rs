use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
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
extern_methods!(
    unsafe impl NSNotification {
        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSNotificationName, Shared>;
        #[method_id(object)]
        pub unsafe fn object(&self) -> Option<Id<Object, Shared>>;
        #[method_id(userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>>;
        # [method_id (initWithName : object : userInfo :)]
        pub unsafe fn initWithName_object_userInfo(
            &self,
            name: &NSNotificationName,
            object: Option<&Object>,
            userInfo: Option<&NSDictionary>,
        ) -> Id<Self, Shared>;
        # [method_id (initWithCoder :)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSNotificationCreation"]
    unsafe impl NSNotification {
        # [method_id (notificationWithName : object :)]
        pub unsafe fn notificationWithName_object(
            aName: &NSNotificationName,
            anObject: Option<&Object>,
        ) -> Id<Self, Shared>;
        # [method_id (notificationWithName : object : userInfo :)]
        pub unsafe fn notificationWithName_object_userInfo(
            aName: &NSNotificationName,
            anObject: Option<&Object>,
            aUserInfo: Option<&NSDictionary>,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSNotificationCenter;
    unsafe impl ClassType for NSNotificationCenter {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSNotificationCenter {
        #[method_id(defaultCenter)]
        pub unsafe fn defaultCenter() -> Id<NSNotificationCenter, Shared>;
        # [method (addObserver : selector : name : object :)]
        pub unsafe fn addObserver_selector_name_object(
            &self,
            observer: &Object,
            aSelector: Sel,
            aName: Option<&NSNotificationName>,
            anObject: Option<&Object>,
        );
        # [method (postNotification :)]
        pub unsafe fn postNotification(&self, notification: &NSNotification);
        # [method (postNotificationName : object :)]
        pub unsafe fn postNotificationName_object(
            &self,
            aName: &NSNotificationName,
            anObject: Option<&Object>,
        );
        # [method (postNotificationName : object : userInfo :)]
        pub unsafe fn postNotificationName_object_userInfo(
            &self,
            aName: &NSNotificationName,
            anObject: Option<&Object>,
            aUserInfo: Option<&NSDictionary>,
        );
        # [method (removeObserver :)]
        pub unsafe fn removeObserver(&self, observer: &Object);
        # [method (removeObserver : name : object :)]
        pub unsafe fn removeObserver_name_object(
            &self,
            observer: &Object,
            aName: Option<&NSNotificationName>,
            anObject: Option<&Object>,
        );
        # [method_id (addObserverForName : object : queue : usingBlock :)]
        pub unsafe fn addObserverForName_object_queue_usingBlock(
            &self,
            name: Option<&NSNotificationName>,
            obj: Option<&Object>,
            queue: Option<&NSOperationQueue>,
            block: TodoBlock,
        ) -> Id<NSObject, Shared>;
    }
);
