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
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUserNotification;
    unsafe impl ClassType for NSUserNotification {
        type Super = NSObject;
    }
);
impl NSUserNotification {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn title(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, title]
    }
    pub unsafe fn setTitle(&self, title: Option<&NSString>) {
        msg_send![self, setTitle: title]
    }
    pub unsafe fn subtitle(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, subtitle]
    }
    pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>) {
        msg_send![self, setSubtitle: subtitle]
    }
    pub unsafe fn informativeText(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, informativeText]
    }
    pub unsafe fn setInformativeText(&self, informativeText: Option<&NSString>) {
        msg_send![self, setInformativeText: informativeText]
    }
    pub unsafe fn actionButtonTitle(&self) -> Id<NSString, Shared> {
        msg_send_id![self, actionButtonTitle]
    }
    pub unsafe fn setActionButtonTitle(&self, actionButtonTitle: &NSString) {
        msg_send![self, setActionButtonTitle: actionButtonTitle]
    }
    pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>> {
        msg_send_id![self, userInfo]
    }
    pub unsafe fn setUserInfo(&self, userInfo: Option<&NSDictionary<NSString, Object>>) {
        msg_send![self, setUserInfo: userInfo]
    }
    pub unsafe fn deliveryDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, deliveryDate]
    }
    pub unsafe fn setDeliveryDate(&self, deliveryDate: Option<&NSDate>) {
        msg_send![self, setDeliveryDate: deliveryDate]
    }
    pub unsafe fn deliveryTimeZone(&self) -> Option<Id<NSTimeZone, Shared>> {
        msg_send_id![self, deliveryTimeZone]
    }
    pub unsafe fn setDeliveryTimeZone(&self, deliveryTimeZone: Option<&NSTimeZone>) {
        msg_send![self, setDeliveryTimeZone: deliveryTimeZone]
    }
    pub unsafe fn deliveryRepeatInterval(&self) -> Option<Id<NSDateComponents, Shared>> {
        msg_send_id![self, deliveryRepeatInterval]
    }
    pub unsafe fn setDeliveryRepeatInterval(
        &self,
        deliveryRepeatInterval: Option<&NSDateComponents>,
    ) {
        msg_send![self, setDeliveryRepeatInterval: deliveryRepeatInterval]
    }
    pub unsafe fn actualDeliveryDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, actualDeliveryDate]
    }
    pub unsafe fn isPresented(&self) -> bool {
        msg_send![self, isPresented]
    }
    pub unsafe fn isRemote(&self) -> bool {
        msg_send![self, isRemote]
    }
    pub unsafe fn soundName(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, soundName]
    }
    pub unsafe fn setSoundName(&self, soundName: Option<&NSString>) {
        msg_send![self, setSoundName: soundName]
    }
    pub unsafe fn hasActionButton(&self) -> bool {
        msg_send![self, hasActionButton]
    }
    pub unsafe fn setHasActionButton(&self, hasActionButton: bool) {
        msg_send![self, setHasActionButton: hasActionButton]
    }
    pub unsafe fn activationType(&self) -> NSUserNotificationActivationType {
        msg_send![self, activationType]
    }
    pub unsafe fn otherButtonTitle(&self) -> Id<NSString, Shared> {
        msg_send_id![self, otherButtonTitle]
    }
    pub unsafe fn setOtherButtonTitle(&self, otherButtonTitle: &NSString) {
        msg_send![self, setOtherButtonTitle: otherButtonTitle]
    }
    pub unsafe fn identifier(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, identifier]
    }
    pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>) {
        msg_send![self, setIdentifier: identifier]
    }
    pub unsafe fn contentImage(&self) -> Option<Id<NSImage, Shared>> {
        msg_send_id![self, contentImage]
    }
    pub unsafe fn setContentImage(&self, contentImage: Option<&NSImage>) {
        msg_send![self, setContentImage: contentImage]
    }
    pub unsafe fn hasReplyButton(&self) -> bool {
        msg_send![self, hasReplyButton]
    }
    pub unsafe fn setHasReplyButton(&self, hasReplyButton: bool) {
        msg_send![self, setHasReplyButton: hasReplyButton]
    }
    pub unsafe fn responsePlaceholder(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, responsePlaceholder]
    }
    pub unsafe fn setResponsePlaceholder(&self, responsePlaceholder: Option<&NSString>) {
        msg_send![self, setResponsePlaceholder: responsePlaceholder]
    }
    pub unsafe fn response(&self) -> Option<Id<NSAttributedString, Shared>> {
        msg_send_id![self, response]
    }
    pub unsafe fn additionalActions(
        &self,
    ) -> Option<Id<NSArray<NSUserNotificationAction>, Shared>> {
        msg_send_id![self, additionalActions]
    }
    pub unsafe fn setAdditionalActions(
        &self,
        additionalActions: Option<&NSArray<NSUserNotificationAction>>,
    ) {
        msg_send![self, setAdditionalActions: additionalActions]
    }
    pub unsafe fn additionalActivationAction(
        &self,
    ) -> Option<Id<NSUserNotificationAction, Shared>> {
        msg_send_id![self, additionalActivationAction]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUserNotificationAction;
    unsafe impl ClassType for NSUserNotificationAction {
        type Super = NSObject;
    }
);
impl NSUserNotificationAction {
    pub unsafe fn actionWithIdentifier_title(
        identifier: Option<&NSString>,
        title: Option<&NSString>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            actionWithIdentifier: identifier,
            title: title
        ]
    }
    pub unsafe fn identifier(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, identifier]
    }
    pub unsafe fn title(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, title]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUserNotificationCenter;
    unsafe impl ClassType for NSUserNotificationCenter {
        type Super = NSObject;
    }
);
impl NSUserNotificationCenter {
    pub unsafe fn scheduleNotification(&self, notification: &NSUserNotification) {
        msg_send![self, scheduleNotification: notification]
    }
    pub unsafe fn removeScheduledNotification(&self, notification: &NSUserNotification) {
        msg_send![self, removeScheduledNotification: notification]
    }
    pub unsafe fn deliverNotification(&self, notification: &NSUserNotification) {
        msg_send![self, deliverNotification: notification]
    }
    pub unsafe fn removeDeliveredNotification(&self, notification: &NSUserNotification) {
        msg_send![self, removeDeliveredNotification: notification]
    }
    pub unsafe fn removeAllDeliveredNotifications(&self) {
        msg_send![self, removeAllDeliveredNotifications]
    }
    pub unsafe fn defaultUserNotificationCenter() -> Id<NSUserNotificationCenter, Shared> {
        msg_send_id![Self::class(), defaultUserNotificationCenter]
    }
    pub unsafe fn delegate(&self) -> Option<Id<id, Shared>> {
        msg_send_id![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: Option<&id>) {
        msg_send![self, setDelegate: delegate]
    }
    pub unsafe fn scheduledNotifications(&self) -> Id<NSArray<NSUserNotification>, Shared> {
        msg_send_id![self, scheduledNotifications]
    }
    pub unsafe fn setScheduledNotifications(
        &self,
        scheduledNotifications: &NSArray<NSUserNotification>,
    ) {
        msg_send![self, setScheduledNotifications: scheduledNotifications]
    }
    pub unsafe fn deliveredNotifications(&self) -> Id<NSArray<NSUserNotification>, Shared> {
        msg_send_id![self, deliveredNotifications]
    }
}
pub type NSUserNotificationCenterDelegate = NSObject;
