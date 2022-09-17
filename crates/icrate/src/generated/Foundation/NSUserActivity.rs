use super::__exported::NSArray;
use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSError;
use super::__exported::NSInputStream;
use super::__exported::NSOutputStream;
use super::__exported::NSSet;
use super::__exported::NSString;
use super::__exported::NSURL;
use crate::Foundation::generated::NSObjCRuntime::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUserActivity;
    unsafe impl ClassType for NSUserActivity {
        type Super = NSObject;
    }
);
impl NSUserActivity {
    pub unsafe fn initWithActivityType(&self, activityType: &NSString) -> Id<Self, Shared> {
        msg_send_id![self, initWithActivityType: activityType]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn addUserInfoEntriesFromDictionary(&self, otherDictionary: &NSDictionary) {
        msg_send![self, addUserInfoEntriesFromDictionary: otherDictionary]
    }
    pub unsafe fn becomeCurrent(&self) {
        msg_send![self, becomeCurrent]
    }
    pub unsafe fn resignCurrent(&self) {
        msg_send![self, resignCurrent]
    }
    pub unsafe fn invalidate(&self) {
        msg_send![self, invalidate]
    }
    pub unsafe fn getContinuationStreamsWithCompletionHandler(&self, completionHandler: TodoBlock) {
        msg_send![
            self,
            getContinuationStreamsWithCompletionHandler: completionHandler
        ]
    }
    pub unsafe fn deleteSavedUserActivitiesWithPersistentIdentifiers_completionHandler(
        persistentIdentifiers: TodoGenerics,
        handler: TodoBlock,
    ) {
        msg_send![
            Self::class(),
            deleteSavedUserActivitiesWithPersistentIdentifiers: persistentIdentifiers,
            completionHandler: handler
        ]
    }
    pub unsafe fn deleteAllSavedUserActivitiesWithCompletionHandler(handler: TodoBlock) {
        msg_send![
            Self::class(),
            deleteAllSavedUserActivitiesWithCompletionHandler: handler
        ]
    }
    pub unsafe fn activityType(&self) -> Id<NSString, Shared> {
        msg_send_id![self, activityType]
    }
    pub unsafe fn title(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, title]
    }
    pub unsafe fn setTitle(&self, title: Option<&NSString>) {
        msg_send![self, setTitle: title]
    }
    pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>> {
        msg_send_id![self, userInfo]
    }
    pub unsafe fn setUserInfo(&self, userInfo: Option<&NSDictionary>) {
        msg_send![self, setUserInfo: userInfo]
    }
    pub unsafe fn requiredUserInfoKeys(&self) -> TodoGenerics {
        msg_send![self, requiredUserInfoKeys]
    }
    pub unsafe fn setRequiredUserInfoKeys(&self, requiredUserInfoKeys: TodoGenerics) {
        msg_send![self, setRequiredUserInfoKeys: requiredUserInfoKeys]
    }
    pub unsafe fn needsSave(&self) -> bool {
        msg_send![self, needsSave]
    }
    pub unsafe fn setNeedsSave(&self, needsSave: bool) {
        msg_send![self, setNeedsSave: needsSave]
    }
    pub unsafe fn webpageURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, webpageURL]
    }
    pub unsafe fn setWebpageURL(&self, webpageURL: Option<&NSURL>) {
        msg_send![self, setWebpageURL: webpageURL]
    }
    pub unsafe fn referrerURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, referrerURL]
    }
    pub unsafe fn setReferrerURL(&self, referrerURL: Option<&NSURL>) {
        msg_send![self, setReferrerURL: referrerURL]
    }
    pub unsafe fn expirationDate(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, expirationDate]
    }
    pub unsafe fn setExpirationDate(&self, expirationDate: Option<&NSDate>) {
        msg_send![self, setExpirationDate: expirationDate]
    }
    pub unsafe fn keywords(&self) -> TodoGenerics {
        msg_send![self, keywords]
    }
    pub unsafe fn setKeywords(&self, keywords: TodoGenerics) {
        msg_send![self, setKeywords: keywords]
    }
    pub unsafe fn supportsContinuationStreams(&self) -> bool {
        msg_send![self, supportsContinuationStreams]
    }
    pub unsafe fn setSupportsContinuationStreams(&self, supportsContinuationStreams: bool) {
        msg_send![
            self,
            setSupportsContinuationStreams: supportsContinuationStreams
        ]
    }
    pub unsafe fn delegate(&self) -> TodoGenerics {
        msg_send![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: TodoGenerics) {
        msg_send![self, setDelegate: delegate]
    }
    pub unsafe fn targetContentIdentifier(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, targetContentIdentifier]
    }
    pub unsafe fn setTargetContentIdentifier(&self, targetContentIdentifier: Option<&NSString>) {
        msg_send![self, setTargetContentIdentifier: targetContentIdentifier]
    }
    pub unsafe fn isEligibleForHandoff(&self) -> bool {
        msg_send![self, isEligibleForHandoff]
    }
    pub unsafe fn setEligibleForHandoff(&self, eligibleForHandoff: bool) {
        msg_send![self, setEligibleForHandoff: eligibleForHandoff]
    }
    pub unsafe fn isEligibleForSearch(&self) -> bool {
        msg_send![self, isEligibleForSearch]
    }
    pub unsafe fn setEligibleForSearch(&self, eligibleForSearch: bool) {
        msg_send![self, setEligibleForSearch: eligibleForSearch]
    }
    pub unsafe fn isEligibleForPublicIndexing(&self) -> bool {
        msg_send![self, isEligibleForPublicIndexing]
    }
    pub unsafe fn setEligibleForPublicIndexing(&self, eligibleForPublicIndexing: bool) {
        msg_send![
            self,
            setEligibleForPublicIndexing: eligibleForPublicIndexing
        ]
    }
    pub unsafe fn isEligibleForPrediction(&self) -> bool {
        msg_send![self, isEligibleForPrediction]
    }
    pub unsafe fn setEligibleForPrediction(&self, eligibleForPrediction: bool) {
        msg_send![self, setEligibleForPrediction: eligibleForPrediction]
    }
    pub unsafe fn persistentIdentifier(&self) -> NSUserActivityPersistentIdentifier {
        msg_send![self, persistentIdentifier]
    }
    pub unsafe fn setPersistentIdentifier(
        &self,
        persistentIdentifier: NSUserActivityPersistentIdentifier,
    ) {
        msg_send![self, setPersistentIdentifier: persistentIdentifier]
    }
}
pub type NSUserActivityDelegate = NSObject;
