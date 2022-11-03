//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSUserNotificationActivationType {
        NSUserNotificationActivationTypeNone = 0,
        NSUserNotificationActivationTypeContentsClicked = 1,
        NSUserNotificationActivationTypeActionButtonClicked = 2,
        NSUserNotificationActivationTypeReplied = 3,
        NSUserNotificationActivationTypeAdditionalActionClicked = 4,
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSUserNotification;

    unsafe impl ClassType for NSUserNotification {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSUserNotification {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString, Shared>>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Id<NSString, Shared>>;

        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

        #[method_id(@__retain_semantics Other informativeText)]
        pub unsafe fn informativeText(&self) -> Option<Id<NSString, Shared>>;

        #[method(setInformativeText:)]
        pub unsafe fn setInformativeText(&self, informativeText: Option<&NSString>);

        #[method_id(@__retain_semantics Other actionButtonTitle)]
        pub unsafe fn actionButtonTitle(&self) -> Id<NSString, Shared>;

        #[method(setActionButtonTitle:)]
        pub unsafe fn setActionButtonTitle(&self, actionButtonTitle: &NSString);

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;

        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, userInfo: Option<&NSDictionary<NSString, Object>>);

        #[method_id(@__retain_semantics Other deliveryDate)]
        pub unsafe fn deliveryDate(&self) -> Option<Id<NSDate, Shared>>;

        #[method(setDeliveryDate:)]
        pub unsafe fn setDeliveryDate(&self, deliveryDate: Option<&NSDate>);

        #[method_id(@__retain_semantics Other deliveryTimeZone)]
        pub unsafe fn deliveryTimeZone(&self) -> Option<Id<NSTimeZone, Shared>>;

        #[method(setDeliveryTimeZone:)]
        pub unsafe fn setDeliveryTimeZone(&self, deliveryTimeZone: Option<&NSTimeZone>);

        #[method_id(@__retain_semantics Other deliveryRepeatInterval)]
        pub unsafe fn deliveryRepeatInterval(&self) -> Option<Id<NSDateComponents, Shared>>;

        #[method(setDeliveryRepeatInterval:)]
        pub unsafe fn setDeliveryRepeatInterval(
            &self,
            deliveryRepeatInterval: Option<&NSDateComponents>,
        );

        #[method_id(@__retain_semantics Other actualDeliveryDate)]
        pub unsafe fn actualDeliveryDate(&self) -> Option<Id<NSDate, Shared>>;

        #[method(isPresented)]
        pub unsafe fn isPresented(&self) -> bool;

        #[method(isRemote)]
        pub unsafe fn isRemote(&self) -> bool;

        #[method_id(@__retain_semantics Other soundName)]
        pub unsafe fn soundName(&self) -> Option<Id<NSString, Shared>>;

        #[method(setSoundName:)]
        pub unsafe fn setSoundName(&self, soundName: Option<&NSString>);

        #[method(hasActionButton)]
        pub unsafe fn hasActionButton(&self) -> bool;

        #[method(setHasActionButton:)]
        pub unsafe fn setHasActionButton(&self, hasActionButton: bool);

        #[method(activationType)]
        pub unsafe fn activationType(&self) -> NSUserNotificationActivationType;

        #[method_id(@__retain_semantics Other otherButtonTitle)]
        pub unsafe fn otherButtonTitle(&self) -> Id<NSString, Shared>;

        #[method(setOtherButtonTitle:)]
        pub unsafe fn setOtherButtonTitle(&self, otherButtonTitle: &NSString);

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString, Shared>>;

        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);

        #[method(hasReplyButton)]
        pub unsafe fn hasReplyButton(&self) -> bool;

        #[method(setHasReplyButton:)]
        pub unsafe fn setHasReplyButton(&self, hasReplyButton: bool);

        #[method_id(@__retain_semantics Other responsePlaceholder)]
        pub unsafe fn responsePlaceholder(&self) -> Option<Id<NSString, Shared>>;

        #[method(setResponsePlaceholder:)]
        pub unsafe fn setResponsePlaceholder(&self, responsePlaceholder: Option<&NSString>);

        #[method_id(@__retain_semantics Other response)]
        pub unsafe fn response(&self) -> Option<Id<NSAttributedString, Shared>>;

        #[method_id(@__retain_semantics Other additionalActions)]
        pub unsafe fn additionalActions(
            &self,
        ) -> Option<Id<NSArray<NSUserNotificationAction>, Shared>>;

        #[method(setAdditionalActions:)]
        pub unsafe fn setAdditionalActions(
            &self,
            additionalActions: Option<&NSArray<NSUserNotificationAction>>,
        );

        #[method_id(@__retain_semantics Other additionalActivationAction)]
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
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:)]
        pub unsafe fn actionWithIdentifier_title(
            identifier: Option<&NSString>,
            title: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString, Shared>>;
    }
);

extern_static!(NSUserNotificationDefaultSoundName: &'static NSString);

extern_class!(
    #[derive(Debug)]
    pub struct NSUserNotificationCenter;

    unsafe impl ClassType for NSUserNotificationCenter {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSUserNotificationCenter {
        #[method_id(@__retain_semantics Other defaultUserNotificationCenter)]
        pub unsafe fn defaultUserNotificationCenter() -> Id<NSUserNotificationCenter, Shared>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSUserNotificationCenterDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSUserNotificationCenterDelegate>);

        #[method_id(@__retain_semantics Other scheduledNotifications)]
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

        #[method_id(@__retain_semantics Other deliveredNotifications)]
        pub unsafe fn deliveredNotifications(&self) -> Id<NSArray<NSUserNotification>, Shared>;

        #[method(deliverNotification:)]
        pub unsafe fn deliverNotification(&self, notification: &NSUserNotification);

        #[method(removeDeliveredNotification:)]
        pub unsafe fn removeDeliveredNotification(&self, notification: &NSUserNotification);

        #[method(removeAllDeliveredNotifications)]
        pub unsafe fn removeAllDeliveredNotifications(&self);
    }
);

extern_protocol!(
    pub struct NSUserNotificationCenterDelegate;

    unsafe impl NSUserNotificationCenterDelegate {
        #[optional]
        #[method(userNotificationCenter:didDeliverNotification:)]
        pub unsafe fn userNotificationCenter_didDeliverNotification(
            &self,
            center: &NSUserNotificationCenter,
            notification: &NSUserNotification,
        );

        #[optional]
        #[method(userNotificationCenter:didActivateNotification:)]
        pub unsafe fn userNotificationCenter_didActivateNotification(
            &self,
            center: &NSUserNotificationCenter,
            notification: &NSUserNotification,
        );

        #[optional]
        #[method(userNotificationCenter:shouldPresentNotification:)]
        pub unsafe fn userNotificationCenter_shouldPresentNotification(
            &self,
            center: &NSUserNotificationCenter,
            notification: &NSUserNotification,
        ) -> bool;
    }
);
