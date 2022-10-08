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
use objc2::{extern_class, extern_methods, ClassType};
pub type NSUserActivityPersistentIdentifier = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSUserActivity;
    unsafe impl ClassType for NSUserActivity {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSUserActivity {
        # [method_id (initWithActivityType :)]
        pub unsafe fn initWithActivityType(&self, activityType: &NSString) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(activityType)]
        pub unsafe fn activityType(&self) -> Id<NSString, Shared>;
        #[method_id(title)]
        pub unsafe fn title(&self) -> Option<Id<NSString, Shared>>;
        # [method (setTitle :)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);
        #[method_id(userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary, Shared>>;
        # [method (setUserInfo :)]
        pub unsafe fn setUserInfo(&self, userInfo: Option<&NSDictionary>);
        # [method (addUserInfoEntriesFromDictionary :)]
        pub unsafe fn addUserInfoEntriesFromDictionary(&self, otherDictionary: &NSDictionary);
        #[method_id(requiredUserInfoKeys)]
        pub unsafe fn requiredUserInfoKeys(&self) -> Option<Id<NSSet<NSString>, Shared>>;
        # [method (setRequiredUserInfoKeys :)]
        pub unsafe fn setRequiredUserInfoKeys(
            &self,
            requiredUserInfoKeys: Option<&NSSet<NSString>>,
        );
        #[method(needsSave)]
        pub unsafe fn needsSave(&self) -> bool;
        # [method (setNeedsSave :)]
        pub unsafe fn setNeedsSave(&self, needsSave: bool);
        #[method_id(webpageURL)]
        pub unsafe fn webpageURL(&self) -> Option<Id<NSURL, Shared>>;
        # [method (setWebpageURL :)]
        pub unsafe fn setWebpageURL(&self, webpageURL: Option<&NSURL>);
        #[method_id(referrerURL)]
        pub unsafe fn referrerURL(&self) -> Option<Id<NSURL, Shared>>;
        # [method (setReferrerURL :)]
        pub unsafe fn setReferrerURL(&self, referrerURL: Option<&NSURL>);
        #[method_id(expirationDate)]
        pub unsafe fn expirationDate(&self) -> Option<Id<NSDate, Shared>>;
        # [method (setExpirationDate :)]
        pub unsafe fn setExpirationDate(&self, expirationDate: Option<&NSDate>);
        #[method_id(keywords)]
        pub unsafe fn keywords(&self) -> Id<NSSet<NSString>, Shared>;
        # [method (setKeywords :)]
        pub unsafe fn setKeywords(&self, keywords: &NSSet<NSString>);
        #[method(supportsContinuationStreams)]
        pub unsafe fn supportsContinuationStreams(&self) -> bool;
        # [method (setSupportsContinuationStreams :)]
        pub unsafe fn setSupportsContinuationStreams(&self, supportsContinuationStreams: bool);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSUserActivityDelegate, Shared>>;
        # [method (setDelegate :)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSUserActivityDelegate>);
        #[method_id(targetContentIdentifier)]
        pub unsafe fn targetContentIdentifier(&self) -> Option<Id<NSString, Shared>>;
        # [method (setTargetContentIdentifier :)]
        pub unsafe fn setTargetContentIdentifier(&self, targetContentIdentifier: Option<&NSString>);
        #[method(becomeCurrent)]
        pub unsafe fn becomeCurrent(&self);
        #[method(resignCurrent)]
        pub unsafe fn resignCurrent(&self);
        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
        # [method (getContinuationStreamsWithCompletionHandler :)]
        pub unsafe fn getContinuationStreamsWithCompletionHandler(
            &self,
            completionHandler: TodoBlock,
        );
        #[method(isEligibleForHandoff)]
        pub unsafe fn isEligibleForHandoff(&self) -> bool;
        # [method (setEligibleForHandoff :)]
        pub unsafe fn setEligibleForHandoff(&self, eligibleForHandoff: bool);
        #[method(isEligibleForSearch)]
        pub unsafe fn isEligibleForSearch(&self) -> bool;
        # [method (setEligibleForSearch :)]
        pub unsafe fn setEligibleForSearch(&self, eligibleForSearch: bool);
        #[method(isEligibleForPublicIndexing)]
        pub unsafe fn isEligibleForPublicIndexing(&self) -> bool;
        # [method (setEligibleForPublicIndexing :)]
        pub unsafe fn setEligibleForPublicIndexing(&self, eligibleForPublicIndexing: bool);
        #[method(isEligibleForPrediction)]
        pub unsafe fn isEligibleForPrediction(&self) -> bool;
        # [method (setEligibleForPrediction :)]
        pub unsafe fn setEligibleForPrediction(&self, eligibleForPrediction: bool);
        #[method_id(persistentIdentifier)]
        pub unsafe fn persistentIdentifier(
            &self,
        ) -> Option<Id<NSUserActivityPersistentIdentifier, Shared>>;
        # [method (setPersistentIdentifier :)]
        pub unsafe fn setPersistentIdentifier(
            &self,
            persistentIdentifier: Option<&NSUserActivityPersistentIdentifier>,
        );
        # [method (deleteSavedUserActivitiesWithPersistentIdentifiers : completionHandler :)]
        pub unsafe fn deleteSavedUserActivitiesWithPersistentIdentifiers_completionHandler(
            persistentIdentifiers: &NSArray<NSUserActivityPersistentIdentifier>,
            handler: TodoBlock,
        );
        # [method (deleteAllSavedUserActivitiesWithCompletionHandler :)]
        pub unsafe fn deleteAllSavedUserActivitiesWithCompletionHandler(handler: TodoBlock);
    }
);
pub type NSUserActivityDelegate = NSObject;
