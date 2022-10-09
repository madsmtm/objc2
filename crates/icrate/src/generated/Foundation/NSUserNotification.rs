use super::__exported::NSArray;
use super::__exported::NSAttributedString;
use super::__exported::NSDate;
use super::__exported::NSDateComponents;
use super::__exported::NSDictionary;
use super::__exported::NSImage;
use super::__exported::NSString;
use super::__exported::NSTimeZone;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUserNotification;
    unsafe impl ClassType for NSUserNotification {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSUserNotification {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(title)]
        pub unsafe fn title(&self) -> Option<Id<NSString, Shared>>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);
        #[method_id(subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Id<NSString, Shared>>;
        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);
        #[method_id(informativeText)]
        pub unsafe fn informativeText(&self) -> Option<Id<NSString, Shared>>;
        #[method(setInformativeText:)]
        pub unsafe fn setInformativeText(&self, informativeText: Option<&NSString>);
        #[method_id(actionButtonTitle)]
        pub unsafe fn actionButtonTitle(&self) -> Id<NSString, Shared>;
        #[method(setActionButtonTitle:)]
        pub unsafe fn setActionButtonTitle(&self, actionButtonTitle: &NSString);
        #[method_id(userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, userInfo: Option<&NSDictionary<NSString, Object>>);
        #[method_id(deliveryDate)]
        pub unsafe fn deliveryDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method(setDeliveryDate:)]
        pub unsafe fn setDeliveryDate(&self, deliveryDate: Option<&NSDate>);
        #[method_id(deliveryTimeZone)]
        pub unsafe fn deliveryTimeZone(&self) -> Option<Id<NSTimeZone, Shared>>;
        #[method(setDeliveryTimeZone:)]
        pub unsafe fn setDeliveryTimeZone(&self, deliveryTimeZone: Option<&NSTimeZone>);
        #[method_id(deliveryRepeatInterval)]
        pub unsafe fn deliveryRepeatInterval(&self) -> Option<Id<NSDateComponents, Shared>>;
        #[method(setDeliveryRepeatInterval:)]
        pub unsafe fn setDeliveryRepeatInterval(
            &self,
            deliveryRepeatInterval: Option<&NSDateComponents>,
        );
        #[method_id(actualDeliveryDate)]
        pub unsafe fn actualDeliveryDate(&self) -> Option<Id<NSDate, Shared>>;
        #[method(isPresented)]
        pub unsafe fn isPresented(&self) -> bool;
        #[method(isRemote)]
        pub unsafe fn isRemote(&self) -> bool;
        #[method_id(soundName)]
        pub unsafe fn soundName(&self) -> Option<Id<NSString, Shared>>;
        #[method(setSoundName:)]
        pub unsafe fn setSoundName(&self, soundName: Option<&NSString>);
        #[method(hasActionButton)]
        pub unsafe fn hasActionButton(&self) -> bool;
        #[method(setHasActionButton:)]
        pub unsafe fn setHasActionButton(&self, hasActionButton: bool);
        #[method(activationType)]
        pub unsafe fn activationType(&self) -> NSUserNotificationActivationType;
        #[method_id(otherButtonTitle)]
        pub unsafe fn otherButtonTitle(&self) -> Id<NSString, Shared>;
        #[method(setOtherButtonTitle:)]
        pub unsafe fn setOtherButtonTitle(&self, otherButtonTitle: &NSString);
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString, Shared>>;
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);
        #[method_id(contentImage)]
        pub unsafe fn contentImage(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setContentImage:)]
        pub unsafe fn setContentImage(&self, contentImage: Option<&NSImage>);
        #[method(hasReplyButton)]
        pub unsafe fn hasReplyButton(&self) -> bool;
        #[method(setHasReplyButton:)]
        pub unsafe fn setHasReplyButton(&self, hasReplyButton: bool);
        #[method_id(responsePlaceholder)]
        pub unsafe fn responsePlaceholder(&self) -> Option<Id<NSString, Shared>>;
        #[method(setResponsePlaceholder:)]
        pub unsafe fn setResponsePlaceholder(&self, responsePlaceholder: Option<&NSString>);
        #[method_id(response)]
        pub unsafe fn response(&self) -> Option<Id<NSAttributedString, Shared>>;
        #[method_id(additionalActions)]
        pub unsafe fn additionalActions(
            &self,
        ) -> Option<Id<NSArray<NSUserNotificationAction>, Shared>>;
        #[method(setAdditionalActions:)]
        pub unsafe fn setAdditionalActions(
            &self,
            additionalActions: Option<&NSArray<NSUserNotificationAction>>,
        );
        #[method_id(additionalActivationAction)]
        pub unsafe fn additionalActivationAction(
            &self,
        ) -> Option<Id<NSUserNotificationAction, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUserNotificationAction;
    unsafe impl ClassType for NSUserNotificationAction {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSUserNotificationAction {
        #[method_id(actionWithIdentifier:title:)]
        pub unsafe fn actionWithIdentifier_title(
            identifier: Option<&NSString>,
            title: Option<&NSString>,
        ) -> Id<Self, Shared>;
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(title)]
        pub unsafe fn title(&self) -> Option<Id<NSString, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSUserNotificationCenter;
    unsafe impl ClassType for NSUserNotificationCenter {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSUserNotificationCenter {
        #[method_id(defaultUserNotificationCenter)]
        pub unsafe fn defaultUserNotificationCenter() -> Id<NSUserNotificationCenter, Shared>;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSUserNotificationCenterDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSUserNotificationCenterDelegate>);
        #[method_id(scheduledNotifications)]
        pub unsafe fn scheduledNotifications(&self) -> Id<NSArray<NSUserNotification>, Shared>;
        #[method(setScheduledNotifications:)]
        pub unsafe fn setScheduledNotifications(
            &self,
            scheduledNotifications: &NSArray<NSUserNotification>,
        );
        #[method(scheduleNotification:)]
        pub unsafe fn scheduleNotification(&self, notification: &NSUserNotification);
        #[method(removeScheduledNotification:)]
        pub unsafe fn removeScheduledNotification(&self, notification: &NSUserNotification);
        #[method_id(deliveredNotifications)]
        pub unsafe fn deliveredNotifications(&self) -> Id<NSArray<NSUserNotification>, Shared>;
        #[method(deliverNotification:)]
        pub unsafe fn deliverNotification(&self, notification: &NSUserNotification);
        #[method(removeDeliveredNotification:)]
        pub unsafe fn removeDeliveredNotification(&self, notification: &NSUserNotification);
        #[method(removeAllDeliveredNotifications)]
        pub unsafe fn removeAllDeliveredNotifications(&self);
    }
);
pub type NSUserNotificationCenterDelegate = NSObject;
